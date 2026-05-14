use relm4::{adw, gtk::prelude::*, prelude::*};

use crate::components::counter::CounterModel;

pub struct AppModel {
    counter_child: Controller<CounterModel>,
}

#[derive(Debug)]
pub enum AppMsg {}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("Hermes"),
            set_default_height: 300,
            set_default_width: 400,

            // children
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {},

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_vexpand:  true,
                    append = model.counter_child.widget(),
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        window: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let counter_child = CounterModel::builder().launch(0).detach();

        let model = AppModel { counter_child };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
