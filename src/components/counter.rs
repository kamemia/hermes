use relm4::{
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
    gtk::{self, prelude::*},
};

pub struct CounterModel {
    count: u8,
}

#[derive(Debug)]
pub enum CounterMsg {
    Increment,
    Decrement,
}

#[relm4::component(pub)]
impl SimpleComponent for CounterModel {
    type Init = u8;
    type Input = CounterMsg;
    type Output = ();

    view! {
        gtk::Box {
            set_align: gtk::Align::Center,
            set_spacing: 12,
            set_margin_all : 12,

            gtk::Button {
                set_label : "Decrement",
                connect_clicked => CounterMsg::Decrement,
            },

            gtk::Label {
                #[watch]
                set_label: &model.count.to_string()
            },

            gtk::Button {
                set_label : "Increment",
                connect_clicked => CounterMsg::Increment,
            }
        }
    }

    fn init(
        count: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = CounterModel { count };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            CounterMsg::Increment => {
                self.count += 1;
            }
            CounterMsg::Decrement => {
                self.count -= 1;
            }
        }
    }
}
