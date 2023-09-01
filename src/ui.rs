use crate::commands::{file_dialog_options, SAVE_AS};
use druid::widget::{Align, Flex, TextBox};
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

#[derive(Clone, Data, Lens)]
pub struct EditorState {
    text: String,
}

pub fn default_text() -> EditorState {
    EditorState {
        text: "Write something...".into(),
    }
}

pub fn build_ui() -> impl Widget<EditorState> {
    let textbox = TextBox::new()
        .with_placeholder("Write something...")
        .lens(EditorState::text);

    let layout = Flex::column().with_flex_child(textbox, 1.0);

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
