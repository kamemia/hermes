use relm4::{adw::prelude::*, prelude::*};

#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub enum Msg {}

#[relm4::factory(pub)]
impl FactoryComponent for Model {
    type Init = Model;
    type Input = Msg;
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 5,
            set_margin_all: 5,

            #[name(header_name)]
            gtk::Label {
                set_label: &self.name,
            },

            #[name(header_value)]
            gtk::Label {
                set_label: &self.value,
            },
        }
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            name: value.name,
            value: value.value,
        }
    }

    fn update(&mut self, message: Self::Input, _sender: FactorySender<Self>) {
        match message {}
    }
}
