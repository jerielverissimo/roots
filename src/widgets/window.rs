use gtk::prelude::*;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
    builder: gtk::Builder,
}

impl Window {
    pub fn new() -> Self {
        let glade_src = include_str!("../../data/resources/ui/window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);
        get_widget!(&builder, gtk::ApplicationWindow, window);

        Self {
            widget: window,
            builder,
        }
    }
}
