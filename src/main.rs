use fltk::{
    app, button::Button, draw, enums, prelude::*, printer, window::Window
};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "FLTK Printer Example");
    let mut but = Button::new(150, 125, 100, 40, "Print");
    
    but.set_callback(|widget| {
        let mut printer = printer::Printer::default();
        if printer.begin_job(1).is_ok() {
            printer.begin_page().ok();
            let (width, height) = printer.printable_rect();
            draw::set_draw_color(enums::Color::Black);
            draw::set_line_style(draw::LineStyle::Solid, 2);
            draw::draw_rect(0, 0, width, height);
            draw::set_font(enums::Font::Courier, 12);
            printer.set_origin(width / 2, height / 2);
            printer.print_widget(widget, -widget.width() / 2, -widget.height() / 2);
            printer.end_page().ok();
            printer.end_job();
        }
    });
    
    wind.end();
    wind.show();
    app.run().unwrap();
}
