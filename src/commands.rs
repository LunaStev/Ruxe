use druid::{Selector, Command, MenuItem, Menu, WindowId, Env, commands::{NEW_FILE, SHOW_PREFERENCES}};
use std::process::Command as SystemCommand;

pub const RUN_CODE: Selector = Selector::new("ruxe.run-code");

pub fn main_menu<T: druid::Data>(_window_id: Option<WindowId>, _data: &T, _env: &Env) -> Menu<T> {
    Menu::empty()
        .entry(MenuItem::new("New").command(NEW_FILE))
        .entry(MenuItem::new("Preferences").command(SHOW_PREFERENCES))
        .separator()
        .entry(MenuItem::new("Run").on_activate(|ctx, _data, _env| {
            ctx.submit_command(RUN_CODE);
        }))
        .separator()
        .entry(MenuItem::new("Exit").command(druid::commands::CLOSE_WINDOW))
}

pub fn run_rust_code(data: &String) -> String {
    std::fs::write("temp_code.rs", data).expect("Failed to write to temp file");

    let output = SystemCommand::new("cargo")
        .arg("run")
        .output()
        .expect("Failed to run the Rust code");

    format!("{:?}", output)
}
