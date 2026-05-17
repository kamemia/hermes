use adw::glib;
use relm4::{gtk::prelude::*, prelude::*};

use crate::request::RequestHeader;

#[derive(Debug)]
pub struct Model {
    pub index: usize,
    pub header: RequestHeader,
}

#[derive(Debug)]
pub enum Input {
    HeaderChanged,
    // internal state tracking
    OnNameChanged(String),
    OnValueChanged(String),
    OnEnabledChanged(bool),
}

#[derive(Debug)]
pub enum Output {
    EmitHeaderChanged(usize, RequestHeader),
}

#[relm4::factory(pub)]
impl FactoryComponent for Model {
    type Init = RequestHeader;
    type Input = Input;
    type Output = Output;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 5,
            set_hexpand: true,

            #[name(header_name)]
            gtk::Entry {
                set_placeholder_text: Some("Name"),
                set_text: &self.header.name.as_str(),
                set_hexpand: true,
                connect_changed[sender] => move |entry| {
                    let text = entry.text().to_string();
                    sender.input(Input::OnNameChanged(text));
                },
            },

            #[name(header_value)]
            gtk::Entry {
                set_placeholder_text: Some("Value"),
                set_text: &self.header.value.as_str(),
                set_hexpand: true,
                connect_changed[sender] => move |entry| {
                    let text = entry.text().to_string();
                    sender.input(Input::OnValueChanged(text));
                },
            },

            gtk::Switch {
                set_active: self.header.enabled,
                set_valign: gtk::Align::Center,
                connect_state_set[sender] => move |_, state| {
                    sender.input(Input::OnEnabledChanged(state));
                    glib::Propagation::Proceed
                },
            },
        }
    }

    fn init_model(init: Self::Init, index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        return Model {
            index: index.current_index(),
            header: init,
        };
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            Input::HeaderChanged => {
                let _ = sender.output(Output::EmitHeaderChanged(self.index, self.header.clone()));
            }
            Input::OnNameChanged(name) => {
                self.header.name = name;
                sender.input(Input::HeaderChanged);
            }
            Input::OnValueChanged(value) => {
                self.header.value = value;
                sender.input(Input::HeaderChanged);
            }
            Input::OnEnabledChanged(enabled) => {
                self.header.enabled = enabled;
                sender.input(Input::HeaderChanged);
            }
        }
    }
}
