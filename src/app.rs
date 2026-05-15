use adw::prelude::*;
use hermes::components::{main_content, main_sidebar};
use relm4::prelude::*;

#[derive(Debug)]
pub struct Model {
    sidebar_controller: Controller<main_sidebar::Model>,
    content_controller: Controller<main_content::Model>,
}

#[derive(Debug)]
pub enum Message {}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = ();
    type Input = Message;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("Hermes"),
            set_size_request: (350, 500),
            set_default_size: (800, 800),

            adw::OverlaySplitView {
                set_sidebar: Some(model.sidebar_controller.widget()),
                set_content: Some(model.content_controller.widget()),
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Model {
            // init content
            sidebar_controller: main_sidebar::Model::builder().launch(()).detach(),
            content_controller: main_content::Model::builder().launch(()).detach(),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
