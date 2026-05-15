use relm4::{adw, gtk::prelude::*, prelude::*};

use crate::components::request_bar;
use crate::components::response_preview;
use crate::utils::network::send_request;

pub struct AppModel {
    request_bar_widget: Controller<request_bar::Model>,
    response_preview_widget: Controller<response_preview::Model>,
    response_preview: String,
}

#[derive(Debug)]
pub enum AppMsg {
    SendRequest(request_bar::Model),
    UpdateResponse(String),
}

const DEFAULT_URL: &str = "https://rickandmortyapi.com/api";

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
                    set_spacing: 5,
                    set_margin_all: 5,

                    // Request bar
                    append = model.request_bar_widget.widget(),
                    // Response preview
                    append = model.response_preview_widget.widget(),
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
                url: DEFAULT_URL.to_string(),
                method: "GET".to_string(),
            })
            .forward(sender.input_sender(), |req_output| match req_output {
                // get request from request bar
                request_bar::Output::Send(req) => AppMsg::SendRequest(req),
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

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            AppMsg::SendRequest(req) => {
                // Show loading state
                sender.input(AppMsg::UpdateResponse(String::from("Loading...")));

                // Send request
                sender
                    .clone()
                    .command(move |_sender, _shutdown| async move {
                        match send_request(req.url, req.method).await {
                            Ok(response) => {
                                let _ = sender.input(AppMsg::UpdateResponse(response));
                            }
                            Err(error) => {
                                let _ = sender.input(AppMsg::UpdateResponse(error));
                            }
                        };
                    });
            }
            AppMsg::UpdateResponse(response) => {
                self.response_preview = response;
                let _ = self
                    .response_preview_widget
                    .sender()
                    .send(response_preview::Msg::Update(
                        self.response_preview.to_string(),
                    ));
            }
        }
    }
}
