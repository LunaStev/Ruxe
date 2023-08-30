mod ui;
mod commands;

use druid::{AppLauncher, WindowDesc};
use ui::build_ui;
use druid::Data;
use druid::kurbo::Arc;

#[derive(Data, Clone)]
struct AppData {
    code: String,
    file_list: Vec<String>
}

fn main() {
    let file_list = Arc::new(vec!["file1.rs".into(), "file2.rs".into(), "file3.rs".into()]);

    let main_window = WindowDesc::new(build_ui)
        .menu(crate::commands::main_menu)
        .title("Ruxe IDE")
        .window_size((800.0, 600.0));

    let data = AppData {
        code: String::new(),
        file_list: vec!["file1.rs".to_string(), "file2.rs".to_string(), "file3.rs".to_string()],
    };

    AppLauncher::with_window(main_window)
        .launch(data)
        .expect("Failed to launch the application");

}
