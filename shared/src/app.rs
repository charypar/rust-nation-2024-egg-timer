use crux_core::{render::Render, App};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
#[effect(app = "EggTimer")]
pub struct Capabilities {
    render: Render<Event>,
}

#[derive(Default)]
pub struct EggTimer;

impl App for EggTimer {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        match event {
            Event::Increase => todo!(),
            Event::Decrease => todo!(),
            Event::ToggleRunning => todo!(),
            Event::Reset => todo!(),
            Event::Tick => todo!(),
        };

        caps.render.render();
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
    use crux_core::testing::AppTester;
    use pretty_assertions::assert_eq;

    use super::{EggTimer, Event, Model};

    #[test]
    fn view_model() {
        let app = AppTester::<EggTimer, _>::default();

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
}
