use relm4::RelmApp;

use hermes::components::main_window::AppModel;

const STYLES_CSS: &str = include_str!("./styles.css");

fn main() {
    let app = RelmApp::new("dev.themosthigh.hermes");
    relm4::set_global_css(STYLES_CSS);
    app.run::<AppModel>(());
}
