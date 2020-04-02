use gio::prelude::*;
use gtk::prelude::*;

pub struct Application {
    app: gtk::Application,
}

impl Application {
    pub fn new() -> Self {
        let app = gtk::Application::new(Some("com.github.roots"), Default::default())
            .expect("Failed to create an Application");
        Application { app }
    }

    pub fn run(&self) {
        self.setup_signals();
        self.app.run(&[]);
    }

    fn setup_signals(&self) {
        self.app.connect_activate(|app| {
            let win = gtk::ApplicationWindow::new(app);

            win.set_default_size(320, 240);
            win.set_title("Roots");

            win.show_all();
        });
    }
}
