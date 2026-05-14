use relm4::{gtk, gtk::prelude::*, prelude::*};
use sourceview5::prelude::*;

pub struct Model {
    response: String,
    source_buffer: sourceview5::Buffer,
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

        gtk::ScrolledWindow {
            set_vexpand: true,
            set_hscrollbar_policy: gtk::PolicyType::Never,
            set_vscrollbar_policy: gtk::PolicyType::Automatic,

            #[name="text_view"]
            sourceview5::View {
                set_buffer: Some(&model.source_buffer),
                set_editable: false,
            },
        }
    }

    fn init(
        response: Self::Init,
        root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Model {
            response,
            source_buffer: init_source_buffer(),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::Update(response) => {
                self.response = response;
                self.source_buffer.set_text(&self.response);
            }
        }
    }
}

fn init_source_buffer() -> sourceview5::Buffer {
    let buffer = sourceview5::Buffer::new(None);
    let lang_manager = sourceview5::LanguageManager::default();
    if let Some(rust_lang) = lang_manager.language("json") {
        buffer.set_language(Some(&rust_lang));
    }
    let scheme_manager = sourceview5::StyleSchemeManager::default();
    if let Some(theme_scheme) = scheme_manager.scheme("Adwaita-dark") {
        buffer.set_style_scheme(Some(&theme_scheme));
    }
    buffer
}
