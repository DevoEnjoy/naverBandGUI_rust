use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, Box};
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use glib::clone;

mod browser_controller;
use browser_controller::BrowserController;

fn main() {
    let application = Application::new(
        Some("com.example.browsercontroller"),
        Default::default(),
    )
    .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        // Create a new window
        let window = ApplicationWindow::new(app);
        window.set_title("Browser Controller");
        window.set_default_size(350, 70);

        // Create a Box layout
        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        window.add(&vbox);

        // Create an Entry widget for URL
        let url_entry = Entry::new();
        url_entry.set_placeholder_text("Enter URL");
        vbox.pack_start(&url_entry, true, true, 0);

        // Create a Button to navigate to URL
        let navigate_button = Button::with_label("Navigate");
        vbox.pack_start(&navigate_button, true, true, 0);

        // Create an Entry widget for search query
        let search_entry = Entry::new();
        search_entry.set_placeholder_text("Enter search query");
        vbox.pack_start(&search_entry, true, true, 0);

        // Create a Button to perform search
        let search_button = Button::with_label("Search");
        vbox.pack_start(&search_button, true, true, 0);

        // Create a Runtime for Tokio
        let rt = Runtime::new().expect("Failed to create Tokio runtime");

        // Create an Arc<Mutex<BrowserController>> for thread-safe shared state
        let browser_controller = Arc::new(Mutex::new(BrowserController::new("path/to/chromedriver/chromedriver.exe")));

        // Start the ChromeDriver
        {
            let browser_controller = Arc::clone(&browser_controller);
            rt.block_on(async {
                let mut bc = browser_controller.lock().unwrap();
                bc.start_driver().await.expect("Failed to start driver");
            });
        }

        // Connect the navigate button to the browser controller
        navigate_button.connect_clicked(clone!(@strong browser_controller, @strong rt => move |_| {
            let url = url_entry.get_text().to_string();
            let browser_controller = Arc::clone(&browser_controller);
            rt.spawn(async move {
                let bc = browser_controller.lock().unwrap();
                bc.navigate_to(&url).await.expect("Failed to navigate");
            });
        }));

        // Connect the search button to the browser controller
        search_button.connect_clicked(clone!(@strong browser_controller, @strong rt => move |_| {
            let query = search_entry.get_text().to_string();
            let browser_controller = Arc::clone(&browser_controller);
            rt.spawn(async move {
                let bc = browser_controller.lock().unwrap();
                bc.search(&query).await.expect("Failed to search");
            });
        }));

        window.show_all();
    });

    application.run(&[]);
}
