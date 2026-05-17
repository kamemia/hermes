use relm4::{gtk, gtk::prelude::*, prelude::*};

use crate::{request::METHODS, utils::shortcut::register_shortcut};

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
    Send,
    UpdateRequestFromBar(Model),
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = Model;
    type Input = Msg;
    type Output = Output;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            add_css_class : "response-bar",

            gtk::DropDown {
                add_css_class: "method-dropdown",
                set_model: Some(gtk::StringList::new(METHODS).upcast_ref::<gtk::gio::ListModel>()),
                // Compute index from static slice
                #[watch]
                set_selected: METHODS.iter().position(|&method| method == &model.method).unwrap() as u32,

                // Update method from dropdown
                connect_selected_notify[sender] => move |dropdown| {
                    let index = dropdown.selected() as usize;
                    sender.input(Msg::MethodChanged(METHODS[index].to_string()));
                },
            },

            #[name="entry"]
            gtk::Entry {
                set_placeholder_text: Some("Enter URL ..."),
                set_text: &model.url,
                set_hexpand: true,
                connect_changed[sender] => move |entry| {
                    let text = entry.text().to_string();
                    sender.input(Msg::UrlChanged(text));
                },
                connect_activate[sender] => move |_entry| {
                    let _ = sender.output(Output::Send);
                },
                inline_css: "border-radius: 0px"
            },

            gtk::Button {
                set_label: "Send",
                connect_clicked => Msg::Send,
                add_css_class: "send-button",
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

        // Focus entry on ctrl+l
        let entry_clone = widgets.entry.clone();
        register_shortcut(&root, "focus_entry", "<Control>l", move || {
            entry_clone.grab_focus();
        });

        // Send request on ctrl+r
        register_shortcut(&root, "send_request", "<Control>r", move || {
            let _ = sender.output(Output::Send);
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::Send => {
                let _ = sender.output(Output::Send);
            }
            Msg::UrlChanged(url) => {
                self.url = url;
                let _ = sender.output(Output::UpdateRequestFromBar(Model {
                    url: self.url.clone(),
                    method: self.method.clone(),
                }));
            }
            Msg::MethodChanged(method) => {
                self.method = method;
                let _ = sender.output(Output::UpdateRequestFromBar(Model {
                    url: self.url.clone(),
                    method: self.method.clone(),
                }));
            }
        }
    }
}
