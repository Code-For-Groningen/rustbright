use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box as GtkBox, Orientation};

const APP_ID: &str = "org.rustbright.window";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("RustBright")
        .default_width(400)
        .default_height(300)
        .build();

    // Create main container
    let main_box = GtkBox::new(Orientation::Vertical, 10);
    
    // Set the main container as the window's child
    window.set_child(Some(&main_box));

    // Present window
    window.present();
}