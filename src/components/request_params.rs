use relm4::gtk::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
pub struct Model {}

#[derive(Debug)]
pub enum Msg {}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        gtk::ScrolledWindow {
            set_vexpand: true,
            set_hscrollbar_policy: gtk::PolicyType::Automatic,
            set_vscrollbar_policy: gtk::PolicyType::Automatic,

            gtk::Label {
                set_label: "Params",
            },

        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Model {};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
