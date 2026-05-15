use adw::prelude::*;
use relm4::prelude::*;

use crate::components::request_bar;
use crate::components::response_preview;
use crate::utils::network::send_request;

#[derive(Debug)]
pub struct Model {
    request_bar_widget: Controller<request_bar::Model>,
    response_preview_widget: Controller<response_preview::Model>,
    response_preview: String,
}

#[derive(Debug)]
pub enum Msg {
    SendRequest(request_bar::Model),
    UpdateResponse(String),
}

const DEFAULT_URL: &str = "https://rickandmortyapi.com/api";

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        adw::ToolbarView {
            add_top_bar = &adw::HeaderBar{
                set_css_classes: &["flat"],
                set_hexpand: true,
                set_show_start_title_buttons: false,
                set_show_end_title_buttons: true,
                set_title_widget = Some(&gtk::Label::new(Some("Request"))),
            },

            #[wrap(Some)]
            set_content = &gtk::Box {
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
                request_bar::Output::Send(req) => Msg::SendRequest(req),
            });

        let res = response_preview::Model::builder()
            .launch(String::from("Response preview section"))
            .detach();

        let model = Model {
            request_bar_widget: req,
            response_preview_widget: res,
            response_preview: String::from("Response preview section"),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::SendRequest(req) => {
                // Show loading state
                sender.input(Msg::UpdateResponse(String::from("Loading...")));

                // Send request
                sender
                    .clone()
                    .command(move |_sender, _shutdown| async move {
                        match send_request(req.url, req.method).await {
                            Ok(response) => {
                                let _ = sender.input(Msg::UpdateResponse(response));
                            }
                            Err(error) => {
                                let _ = sender.input(Msg::UpdateResponse(error));
                            }
                        };
                    });
            }
            Msg::UpdateResponse(response) => {
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
