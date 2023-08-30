use druid::{Selector, Command, MenuItem, Menu, WindowId, Env, commands::{NEW_FILE, SHOW_PREFERENCES}};

pub const RUN_CODE: Selector = Selector::new("ruxe.run-code");

pub fn main_menu<T: druid::Data>(_window_id: Option<WindowId>, _data: &T, _env: &Env) -> Menu<T> {
    Menu::empty()
        .entry(MenuItem::new("New").command(NEW_FILE))
        .entry(MenuItem::new("Preferences").command(SHOW_PREFERENCES))
        .separator()
        .entry(MenuItem::new("Exit").command(druid::commands::CLOSE_WINDOW))
}
