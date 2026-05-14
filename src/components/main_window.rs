use relm4::{adw, gtk::prelude::*, prelude::*};

use crate::components::request_bar;
use crate::components::response_preview;

pub struct AppModel {
    request_bar_widget: Controller<request_bar::Model>,
    response_preview_widget: Controller<response_preview::Model>,
    response_preview: String,
}

#[derive(Debug)]
pub enum AppMsg {
    ReceiveRequest(request_bar::Model),
}

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

                    // Request bar
                    gtk::Box {
                        append = model.request_bar_widget.widget(),
                    },

                    // Response preview
                    gtk::ScrolledWindow {
                        set_vexpand: true,
                        set_hscrollbar_policy: gtk::PolicyType::Never,
                        set_vscrollbar_policy: gtk::PolicyType::Automatic,

                        gtk::Box {
                            append = model.response_preview_widget.widget(),
                        }

                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        window: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let req = request_bar::Model::builder()
            .launch(request_bar::Model {
                url: "https://www.google.com".to_string(),
                method: "GET".to_string(),
            })
            .forward(sender.input_sender(), |req_output| match req_output {
                // get request from request bar
                request_bar::Output::Send(req) => AppMsg::ReceiveRequest(req),
            });

        let res = response_preview::Model::builder()
            .launch(String::from("Response preview section"))
            .detach();

        let model = AppModel {
            request_bar_widget: req,
            response_preview_widget: res,
            response_preview: String::from("Response preview section"),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            AppMsg::ReceiveRequest(req) => {
                self.response_preview = req.url;

                let child_sender = self.response_preview_widget.sender();
                let _ = child_sender.send(response_preview::Msg::Update(
                    self.response_preview.to_string(),
                ));
            }
        }
    }
}
