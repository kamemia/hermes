use relm4::{gtk, gtk::prelude::*, prelude::*};

pub struct Model {
    response: String,
    text_buffer: gtk::TextBuffer,
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
            gtk::TextView {
                set_buffer: Some(&model.text_buffer),

                //misc
                set_editable: false,
                set_cursor_visible: true,
                set_wrap_mode: gtk::WrapMode::Word,
                set_monospace: true,
                set_margin_all: 5,

                set_pixels_above_lines: 5,
                set_pixels_below_lines: 5,
                set_pixels_inside_wrap: 5,

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
            text_buffer: gtk::TextBuffer::new(None),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::Update(response) => {
                self.response = response;
                self.text_buffer.set_text(&self.response);
            }
        }
    }
}
