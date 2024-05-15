use fltk::app::App;
use fltk::prelude::*;
use fltk::window::Window;

fn main() {
    let app = App::default();
    let mut window = Window::default()
        .with_size(800, 600)
        .with_label(internal_lib::app_title());
    window.show();
    app.run().ok();
}
