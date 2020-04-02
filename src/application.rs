use crate::widgets::window::Window;

use gio::prelude::*;
use gtk::prelude::*;

pub struct Application {
    app: gtk::Application,
    window: Window,
}

impl Application {
    pub fn new() -> Self {
        let app = gtk::Application::new(Some("com.github.roots"), Default::default())
            .expect("Failed to create an Application");

        let window = Window::new();
        Application { app, window }
    }

    pub fn run(&self) {
        self.setup_gactions();
        self.setup_signals();
        self.app.run(&[]);
    }

    fn setup_gactions(&self) {
        let app = self.app.clone();
        action!(self.app, "quit", move |_, _| app.quit());
    }

    fn setup_signals(&self) {
        let win = self.window.widget.clone();
        self.app.connect_activate(move |app| {
            win.set_application(Some(app));
            app.add_window(&win);
            win.present();
        });
    }
}
