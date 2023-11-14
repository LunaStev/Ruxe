mod commands;
mod ui;

use druid::AppLauncher;
use druid::PlatformError;
use druid::WindowDesc;

//... 기타 코드 ...

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui::build_ui())
        .title("Ruxe")
        .window_size((800.0, 600.0))
        .menu(|_window_id, _data, _env| {
            ui::build_menu()
        });

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(ui::default_text())
}
