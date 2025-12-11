use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let app = Application::new(Some("com.erhabener.loetgott.lights"), Default::default());

    app.connect_activate(|app| {
        let window1 = ApplicationWindow::new(app);
        window1.set_title(Option::from("Fenster 1"));
        window1.set_default_size(300, 200);
        let button1 = Button::with_label("Hallo Fenster 1");
        window1.set_child(Some(&button1));
        window1.show();

        let window2 = ApplicationWindow::new(app);
        window2.set_title(Option::from("Fenster 2"));
        window2.set_default_size(300, 200);
        let button2 = Button::with_label("Hallo Fenster 2");
        window2.set_child(Some(&button2));
        window2.show();
    });

    app.run();
}
