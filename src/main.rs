#[macro_use]
mod utils;

mod application;
mod widgets;

fn main() {
    let app = application::Application::new();
    app.run();
}
