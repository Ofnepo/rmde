use fltk::{window::MenuWindow, app::App, prelude::*};

fn main() {
    let app = App::default();
    let mut win = MenuWindow::default();


    win.end();
    win.show();
    app.run().unwrap();
}
