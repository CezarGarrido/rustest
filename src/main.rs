use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, PrintOperation};

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Print");
        button.connect_clicked(move |_| {
            let print_operation = PrintOperation::new();
            
            
            print_operation.set_allow_async(true);
            
            print_operation.run(gtk::PrintOperationAction::PrintDialog, None::<&ApplicationWindow>);
        });
        
        window.add(&button);
        window.show_all();
    });

    application.run();
}
