use relm4::RelmApp;

mod app;

const STYLES_CSS: &str = include_str!("./styles.css");

fn main() {
    let app = RelmApp::new("dev.themosthigh.hermes");
    relm4::set_global_css(STYLES_CSS);
    app.run::<app::Model>(());
}
