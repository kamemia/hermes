use relm4::{gtk, gtk::prelude::*, prelude::*};

#[derive(Debug, Clone)]
pub struct Model {
    pub url: String,
    pub method: String,
}

#[derive(Debug)]
pub enum Msg {
    Send,
    UrlChanged(String),
    MethodChanged(String),
}

#[derive(Debug)]
pub enum Output {
    Send(Model),
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = Model;
    type Input = Msg;
    type Output = Output;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 5,
            set_margin_all: 5,

            gtk::Entry {
                set_placeholder_text: Some("Enter URL ..."),
                set_text: &model.url,
                set_hexpand: true,
                connect_changed[sender] => move |entry| {
                    let text = entry.text().to_string();
                    sender.input(Msg::UrlChanged(text));
                },
            },

            gtk::Button {
                set_label: "Send",
                connect_clicked => Msg::Send,
            }

        }
    }

    fn init(
        intial_values: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Model {
            url: intial_values.url,
            method: intial_values.method,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::Send => {
                let _ = sender.output(Output::Send(Model {
                    url: self.url.clone(),
                    method: self.method.clone(),
                }));
            }
            Msg::UrlChanged(url) => {
                self.url = url;
            }
            Msg::MethodChanged(method) => {
                self.method = method;
            }
        }
    }
}
