use relm4::{adw::prelude::*, prelude::*};

mod header_list_item;

#[derive(Debug)]
pub struct Model {
    headers: Vec<(String, String)>,
    headers_list_widgets: FactoryVecDeque<header_list_item::Model>,
}

#[derive(Debug)]
pub enum Msg {
    HeadersChanged(Vec<(String, String)>),
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = Vec<(String, String)>;
    type Input = Msg;
    type Output = ();

    view! {
        gtk::ScrolledWindow {
            set_vexpand: true,
            set_hscrollbar_policy: gtk::PolicyType::Automatic,
            set_vscrollbar_policy: gtk::PolicyType::Automatic,
            add_css_class: "card",

            #[local_ref]
            headers_list_box -> gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let headers_list_widgets = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .detach();

        let model = Model {
            headers: init,
            headers_list_widgets,
        };

        let headers_list_box = model.headers_list_widgets.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::HeadersChanged(headers) => {
                self.headers = headers;

                let mut guard = self.headers_list_widgets.guard();
                guard.clear();

                for (name, value) in &self.headers {
                    guard.push_back(header_list_item::Model {
                        name: name.clone(),
                        value: value.clone(),
                    });
                }
            }
        }
    }
}
