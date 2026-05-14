use relm4::{gtk, gtk::prelude::*, prelude::*};

pub struct Model {
    response: String,
}

#[derive(Debug)]
pub enum Msg {
    Update(String),
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = String;
    type Input = Msg;
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 5,
            set_margin_all: 5,

            gtk::Label {
                #[watch]
                set_label: model.response.as_str(),
            },
        }
    }

    fn init(
        response: Self::Init,
        root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Model { response };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::Update(response) => {
                self.response = response;
            }
        }
    }
}
