use crux_core::{
    macros::effect,
    render::{render, RenderOperation},
    App, Command,
};
use facet::Facet;
use serde::{Deserialize, Serialize};

use crate::capabilities::wall_clock::{self, ClockOperation};

#[derive(Facet, Serialize, Deserialize, Clone, Debug)]
#[repr(C)]
pub enum Event {
    Increase,
    Decrease,
    ToggleRunning,
    Reset,
    #[serde(skip)]
    Tick,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Model {
    NotStarted { goal_time: u32 },
    Running { goal_time: u32, elapsed_time: u32 },
    Paused { goal_time: u32, elapsed_time: u32 },
    Finished { goal_time: u32 },
}

impl Default for Model {
    fn default() -> Self {
        Model::NotStarted { goal_time: 30 }
    }
}

#[derive(Clone, PartialEq, Facet, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[facet(rename_all = "camelCase")]
#[repr(C)]
pub struct ViewModel {
    minutes: u32,
    seconds: u32,
    percentage: u32,
    running: bool,
    finished: bool,
    can_edit: bool,
    can_toggle_runnning: bool,
    can_reset: bool,
}

#[effect(facet_typegen)]
pub enum Effect {
    Render(RenderOperation),
    WallClock(ClockOperation),
}

#[derive(Default)]
pub struct EggTimer;

impl App for EggTimer {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Effect = Effect;

    fn update(&self, event: Self::Event, model: &mut Self::Model) -> Command<Effect, Event> {
        let clock_command = match event {
            Event::Increase => {
                if let Model::NotStarted { goal_time } = model {
                    *goal_time += 10;
                }

                Command::done()
            }
            Event::Decrease => {
                if let Model::NotStarted { goal_time } = model {
                    *goal_time = goal_time.saturating_sub(10);
                }

                Command::done()
            }
            Event::ToggleRunning => match model {
                Model::NotStarted { goal_time } => {
                    *model = Model::Running {
                        goal_time: *goal_time,
                        elapsed_time: 0,
                    };

                    wall_clock::start(1).then_send(|_| Event::Tick)
                }
                Model::Running {
                    goal_time,
                    elapsed_time,
                } => {
                    *model = Model::Paused {
                        goal_time: *goal_time,
                        elapsed_time: *elapsed_time,
                    };

                    wall_clock::stop()
                }
                Model::Paused {
                    goal_time,
                    elapsed_time,
                } => {
                    *model = Model::Running {
                        goal_time: *goal_time,
                        elapsed_time: *elapsed_time,
                    };

                    wall_clock::start(1).then_send(|_| Event::Tick)
                }
                Model::Finished { .. } => {
                    // Can't toggle a finished timer
                    Command::done()
                }
            },
            Event::Reset => {
                let goal_time = match model {
                    Model::NotStarted { goal_time } => goal_time,
                    Model::Running { goal_time, .. } => goal_time,
                    Model::Paused { goal_time, .. } => goal_time,
                    Model::Finished { goal_time } => goal_time,
                };

                *model = Model::NotStarted {
                    goal_time: *goal_time,
                };

                wall_clock::stop()
            }
            Event::Tick => {
                if let Model::Running {
                    goal_time,
                    elapsed_time,
                } = model
                {
                    *elapsed_time += 1;

                    if *elapsed_time >= *goal_time {
                        *model = Model::Finished {
                            goal_time: *goal_time,
                        };

                        wall_clock::stop()
                    } else {
                        Command::done()
                    }
                } else {
                    Command::done()
                }
            }
        };

        clock_command.and(render())
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        match model {
            Model::NotStarted { goal_time } => ViewModel {
                minutes: goal_time / 60,
                seconds: goal_time % 60,
                percentage: 100,
                running: false,
                finished: false,
                can_edit: true,
                can_toggle_runnning: true,
                can_reset: false,
            },
            Model::Running {
                goal_time,
                elapsed_time,
            } => {
                let remaining_time = goal_time.saturating_sub(*elapsed_time);
                let percentage = (remaining_time as f64 / *goal_time as f64 * 100.0) as u32;

                ViewModel {
                    minutes: remaining_time / 60,
                    seconds: remaining_time % 60,
                    percentage,
                    running: true,
                    finished: false,
                    can_edit: false,
                    can_toggle_runnning: true,
                    can_reset: true,
                }
            }
            Model::Paused {
                goal_time,
                elapsed_time,
            } => {
                let remaining_time = goal_time.saturating_sub(*elapsed_time);
                let percentage = (remaining_time as f64 / *goal_time as f64 * 100.0) as u32;

                ViewModel {
                    minutes: remaining_time / 60,
                    seconds: remaining_time % 60,
                    percentage,
                    running: false,
                    finished: false,
                    can_edit: false,
                    can_toggle_runnning: true,
                    can_reset: true,
                }
            }
            Model::Finished { .. } => ViewModel {
                minutes: 0,
                seconds: 0,
                percentage: 100,
                running: false,
                finished: true,
                can_edit: false,
                can_toggle_runnning: false,
                can_reset: true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crux_core::App as _;
    // use pretty_assertions::assert_eq;

    use crate::capabilities::wall_clock::{ClockOperation, ClockOutput};

    use super::{EggTimer, Event, Model};

    #[test]
    fn view_model() {
        let app = EggTimer;

        let model = Model::NotStarted { goal_time: 30 };
        let view_model = app.view(&model);

        assert_eq!(
            view_model,
            crate::ViewModel {
                minutes: 0,
                seconds: 30,
                percentage: 100,
                running: false,
                finished: false,
                can_edit: true,
                can_toggle_runnning: true,
                can_reset: false,
            }
        );

        let model = Model::Running {
            goal_time: 80,
            elapsed_time: 10,
        };
        let view_model = app.view(&model);

        assert_eq!(
            view_model,
            crate::ViewModel {
                minutes: 1,
                seconds: 10,
                percentage: 87,
                running: true,
                finished: false,
                can_edit: false,
                can_toggle_runnning: true,
                can_reset: true
            }
        );

        let model = Model::Paused {
            goal_time: 70,
            elapsed_time: 10,
        };
        let view_model = app.view(&model);

        assert_eq!(
            view_model,
            crate::ViewModel {
                minutes: 1,
                seconds: 0,
                percentage: 85,
                running: false,
                finished: false,
                can_edit: false,
                can_toggle_runnning: true,
                can_reset: true
            }
        );

        let model = Model::Finished { goal_time: 30 };
        let view_model = app.view(&model);

        assert_eq!(
            view_model,
            crate::ViewModel {
                minutes: 0,
                seconds: 0,
                percentage: 100,
                running: false,
                finished: true,
                can_edit: false,
                can_toggle_runnning: false,
                can_reset: true
            }
        );
    }

    #[test]
    fn set_goal_time() {
        let app = EggTimer;
        let mut model = Model::NotStarted { goal_time: 30 };

        let mut command = app.update(Event::Increase, &mut model);
        command.expect_one_effect().expect_render();

        assert_eq!(model, Model::NotStarted { goal_time: 40 });

        let mut command = app.update(Event::Decrease, &mut model);
        command.expect_one_effect().expect_render();

        assert_eq!(model, Model::NotStarted { goal_time: 30 });

        let mut model = Model::Running {
            goal_time: 30,
            elapsed_time: 15,
        };

        let mut command = app.update(Event::Increase, &mut model);
        command.expect_one_effect().expect_render();

        assert_eq!(
            model,
            Model::Running {
                goal_time: 30,
                elapsed_time: 15
            }
        );

        let mut command = app.update(Event::Decrease, &mut model);
        command.expect_one_effect().expect_render();

        assert_eq!(
            model,
            Model::Running {
                goal_time: 30,
                elapsed_time: 15
            }
        );
    }

    #[test]
    fn tick() {
        let app = EggTimer;
        let mut model = Model::Running {
            goal_time: 30,
            elapsed_time: 0,
        };

        let mut command = app.update(Event::Tick, &mut model);
        command.expect_one_effect().expect_render();

        assert_eq!(
            model,
            Model::Running {
                goal_time: 30,
                elapsed_time: 1
            }
        );

        let mut model = Model::Running {
            goal_time: 30,
            elapsed_time: 29,
        };

        let mut command = app.update(Event::Tick, &mut model);
        let clock_req = command.expect_effect().expect_wall_clock();
        assert_eq!(clock_req.operation, ClockOperation::Stop);

        command.expect_one_effect().expect_render();

        assert_eq!(model, Model::Finished { goal_time: 30 });
    }

    #[test]
    fn reset() {
        let app = EggTimer;

        let mut model = Model::Running {
            goal_time: 27,
            elapsed_time: 15,
        };

        let mut command = app.update(Event::Reset, &mut model);

        let clock_req = command.expect_effect().expect_wall_clock();
        assert_eq!(clock_req.operation, ClockOperation::Stop);

        command.expect_one_effect().expect_render();

        assert_eq!(model, Model::NotStarted { goal_time: 27 });

        command.expect_no_effect_or_events();
    }

    #[test]
    fn starting_timer() {
        let app = EggTimer;

        let mut model = Model::NotStarted { goal_time: 30 };

        let mut command = app.update(Event::ToggleRunning, &mut model);
        assert_eq!(
            model,
            Model::Running {
                goal_time: 30,
                elapsed_time: 0
            }
        );

        command.expect_no_events();
        let mut clock_req = command.expect_effect().expect_wall_clock();

        let ClockOperation::Start(interval) = clock_req.operation else {
            panic!("Expected start operation");
        };
        assert_eq!(interval, 1);

        clock_req.resolve(ClockOutput::Tick).expect("to resolve");

        let event = command.expect_one_event();
        let mut command = app.update(event, &mut model);
        command.expect_one_effect().expect_render();

        assert_eq!(
            model,
            Model::Running {
                goal_time: 30,
                elapsed_time: 1
            }
        );
    }

    #[test]
    fn pausing_timer() {
        let app = EggTimer;

        let mut model = Model::Running {
            goal_time: 30,
            elapsed_time: 15,
        };

        let mut command = app.update(Event::ToggleRunning, &mut model);
        assert_eq!(
            model,
            Model::Paused {
                goal_time: 30,
                elapsed_time: 15
            }
        );

        command.expect_no_events();

        let clock_req = command.expect_effect().expect_wall_clock();
        command.expect_effect().expect_render();

        let ClockOperation::Stop = clock_req.operation else {
            panic!("Expected stop operation");
        };

        command.expect_done();
    }

    #[test]
    fn stops_the_clock_when_finished() {
        let app = EggTimer;

        let mut model = Model::Running {
            goal_time: 30,
            elapsed_time: 29,
        };

        let mut command = app.update(Event::Tick, &mut model);
        assert_eq!(model, Model::Finished { goal_time: 30 });

        command.expect_no_events();
        let clock_req = command.expect_effect().expect_wall_clock();

        let ClockOperation::Stop = clock_req.operation else {
            panic!("Expected stop operation");
        };

        command.expect_one_effect().expect_render();
    }

    #[test]
    fn stops_the_clock_when_reset() {
        let app = EggTimer;

        let mut model = Model::Running {
            goal_time: 30,
            elapsed_time: 15,
        };

        let mut command = app.update(Event::Reset, &mut model);
        assert_eq!(model, Model::NotStarted { goal_time: 30 });

        command.expect_no_events();
        let clock_req = command.expect_effect().expect_wall_clock();

        let ClockOperation::Stop = clock_req.operation else {
            panic!("Expected stop operation");
        };

        command.expect_one_effect().expect_render();
        command.expect_done();
    }
}
