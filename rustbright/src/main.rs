use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box as GtkBox, Orientation};
use std::io::Write;

const APP_ID: &str = "org.rustbright.window";

mod auth;
use auth::{start_login, complete_2fa};
fn main(){
    test_login();

    // // Create a new application
    // let app = Application::builder().application_id(APP_ID).build();

    // // Connect to "activate" signal of `app`
    // app.connect_activate(build_ui);

    // // Run the application
    // app.run()

}

fn test_login() {
    println!("Testing login functionality...");

    let username = std::env::var("USERNAME").expect("USERNAME environment variable not set");
    let password = std::env::var("PASSWORD").expect("PASSWORD environment variable not set");
    match start_login(&username, &password) {
        Ok(Some(session)) => {
            println!("Login successful, 2FA required");
            print!("Enter 2FA code: ");
            std::io::stdout().flush().unwrap();
            
            let mut two_fa_code = String::new();
            std::io::stdin().read_line(&mut two_fa_code).unwrap();
            let two_fa_code = two_fa_code.trim();
            
            match complete_2fa(session, two_fa_code) {
            Ok(cookies) => println!("Cookies received: {:?}", cookies),
            Err(e) => eprintln!("2FA failed: {}", e),
            }
        },
        Ok(None) => {
            println!("Already logged in, no 2FA needed");
        },
        Err(e) => {
            eprintln!("Login failed: {}", e);
        }
    }
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