use relm4::gtk;
use relm4::gtk::prelude::*;

pub fn register_shortcut<W, F>(root: &W, action_name: &str, key_combo: &str, callback: F)
where
    W: IsA<gtk::Widget>,
    F: Fn() + 'static,
{
    let action = gtk::gio::SimpleAction::new(action_name, None);
    action.connect_activate(move |_, _| {
        callback();
    });

    let action_group = gtk::gio::SimpleActionGroup::new();
    action_group.add_action(&action);
    root.insert_action_group("shorcut", Some(&action_group));

    let shortcut = gtk::Shortcut::builder()
        .trigger(&gtk::ShortcutTrigger::parse_string(key_combo).unwrap())
        .action(&gtk::NamedAction::new(&format!("shorcut.{action_name}")))
        .build();

    let shortcut_controller = gtk::ShortcutController::new();
    shortcut_controller.set_scope(gtk::ShortcutScope::Global);
    shortcut_controller.add_shortcut(shortcut);

    root.add_controller(shortcut_controller);
}
