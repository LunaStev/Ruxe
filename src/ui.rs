use druid::{Widget, widget::{Button, Flex, TextBox}, Menu, MenuItem};
use druid::WidgetExt;
use crate::commands::{RUN_CODE, main_menu};

pub fn build_ui() -> impl Widget<String> {
    let editor = TextBox::multiline()
        .with_placeholder("Write your Rust code here...")
        .fix_height(400.0)
        .expand_width();

    let terminal = TextBox::multiline()
        .with_placeholder("Output will be shown here...")
        .fix_height(200.0)
        .expand_width();

    let run_button = Button::new("Run")
        .on_click(|ctx, data: &mut String, _env| {
            ctx.submit_command(RUN_CODE);
        });

    Flex::column()
        .with_child(editor)
        .with_spacer(10.0)
        .with_child(terminal)
        .with_spacer(10.0)
        .with_child(run_button)
}
