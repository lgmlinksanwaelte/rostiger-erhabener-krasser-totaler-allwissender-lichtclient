use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

pub(crate) fn run() {
    let app = Application::new(
        Some("R.E.K.T.A.L-Launcher"),
        Default::default(),
    );

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("R.E.K.T.A.Launcher"));
    window.set_default_size(400, 300);

    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    window.set_child(Some(&main_box));

    let button = gtk4::Button::with_label("Farbe ausgeben");
    button.connect_clicked(|_| {
        println!("Button clicked!");
    });

    main_box.append(&button);
    window.show();
}
