extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
        );
    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("TITLE");
        window.set_default_size(300, 100);
        let win_title = gtk::Label::new(None);
        win_title.set_markup("LABEL");
        let input = gtk::Entry::new();
        let row = gtk::Box::new(gtk::Orientation::Vertical, 5);
        row.add(&win_title);
        row.add(&input);
        let button = Button::with_label("BUTTON");
        button.connect_clicked(move |_| {
            println!("{}", input);
        });
        row.add(&button);
        window.add(&row);
        window.show_all();
    });
    application.run();
}
