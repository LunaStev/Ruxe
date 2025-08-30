use druid::{commands, Selector};
use druid::FileDialogOptions;
use druid::Menu;
use druid::MenuItem;
use crate::ui::EditorState;

pub const SAVE_AS: Selector<()> = Selector::new("text-editor.save-as");

pub fn make_file_menu() -> Menu<EditorState> {
    Menu::empty()
        .entry(MenuItem::new("Open").on_activate(|ctx, _, _| {
            let options = FileDialogOptions::new()
                .title("Open Text File")
                .button_text("Open");
            ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(options));
        }))
        .entry(MenuItem::new("Save As").on_activate(|ctx, _, _| {
            ctx.submit_command(SAVE_AS);
        }))
        .separator()
        .entry(MenuItem::new("Quit").on_activate(|ctx, _, _| {
            ctx.submit_command(commands::QUIT_APP);
        }))
}

pub fn file_dialog_options() -> FileDialogOptions {
    FileDialogOptions::new()
        .name_label("Name")
        .title("Save As")
        .button_text("Save")
}
