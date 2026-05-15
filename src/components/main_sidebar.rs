use adw::gtk;
use relm4::{ComponentParts, SimpleComponent};

#[derive(Debug)]
pub struct Model {}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::ToolbarView {
            add_top_bar = &adw::HeaderBar {
                set_title_widget = Some(&gtk::Label::new(Some("Hermes"))),
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Model {};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
