mod ui;
mod commands;

use druid::{AppLauncher, WindowDesc};
use ui::build_ui;

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .menu(crate::commands::main_menu)
        .title("Ruxe")
        .window_size((600.0, 500.0));

    AppLauncher::with_window(main_window)
        .launch(String::new())
        .expect("Failed to launch the application");
}
