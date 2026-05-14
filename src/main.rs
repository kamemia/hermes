use relm4::RelmApp;

use hermes::components::main_window::AppModel;

fn main() {
    let app = RelmApp::new("dev.themosthigh.hermes");
    app.run::<AppModel>(());
}
