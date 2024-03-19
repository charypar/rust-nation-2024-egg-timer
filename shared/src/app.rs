use crux_core::{render::Render, App};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    Hello,
}

#[derive(Default)]
pub struct Model {
    message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ViewModel {
    message: String,
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
            Event::Hello => {
                model.message = Some("Hello, World!".to_string());
            }
        }
        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            message: model
                .message
                .as_ref()
                .unwrap_or(&"Nothing to see".to_string())
                .to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crux_core::testing::AppTester;
    use pretty_assertions::assert_eq;

    use crate::{EggTimer, Event, Model};

    #[test]
    fn test_hello() {
        let app_tester = AppTester::<EggTimer, _>::default();
        let app = app_tester;
        let mut model = Model::default();

        let view = app.view(&model);
        assert_eq!(view.message, "Nothing to see");

        app.update(Event::Hello, &mut model);

        let view = app.view(&model);
        assert_eq!(view.message, "Hello, World!");
    }
}
