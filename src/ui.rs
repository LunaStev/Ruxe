use crate::commands::{file_dialog_options, SAVE_AS};
use druid::widget::{Align, Flex, TextBox, List, Scroll, Label, Split};
use druid::{Event, WindowId};
use druid::AppDelegate;
use druid::Command;
use druid::Data;
use druid::DelegateCtx;
use druid::Env;
use druid::Lens;
use druid::menu::MenuDesc;
use druid::Widget;
use druid::WidgetExt;
use crate::commands;
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
pub struct EditorState {
    text: String,
    files: Arc<Vec<String>>,
    terminal_text: String,
}

pub fn default_text() -> EditorState {
    EditorState {
        text: "Write something...".into(),
        files: Arc::new(vec!["file1.rs".into(), "file2.rs".into()]),
        terminal_text: "".into(),
    }
}

pub fn build_ui() -> impl Widget<EditorState> {
    let textbox = TextBox::new()
        .with_placeholder("Write something...")
        .lens(EditorState::text)
        .expand_height();

    let file_list = List::new(|| {
        Label::new(|item: &String, _env: &_| item.clone())
    }).lens(EditorState::files)
        .expand_height();

    let terminal = TextBox::multiline()
        .with_placeholder("Terminal output...")
        .lens(EditorState::terminal_text)
        .expand_width();

    let main_editor = Flex::column()
        .with_flex_child(textbox, 3.0)
        .with_flex_child(terminal, 1.0)
        .expand_width();

    let layout = Flex::row()
        .with_flex_child(file_list, 1.0)
        .with_flex_child(main_editor, 4.0);

    Align::centered(layout)
}

pub fn build_menu() -> MenuDesc<EditorState> {
    MenuDesc::empty().append_entry(commands::make_file_menu())
}

pub struct Delegate;

impl AppDelegate<EditorState> for Delegate {
    fn event(
        &mut self,
        ctx: &mut DelegateCtx,
        window_id: WindowId,
        event: Event,
        data: &mut EditorState,
        env: &Env,
    ) -> Option<Event> {
        if let Event::Command(cmd) = &event {
            if let Some(info) = cmd.get(druid::commands::OPEN_FILE) {
                // Implement opening a file here
            } else if cmd.is(SAVE_AS) {
                ctx.submit_command(Command::new(druid::commands::SHOW_SAVE_PANEL, file_dialog_options(), druid::Target::Global));
            }
        }
        Some(event)
    }
}