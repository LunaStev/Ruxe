use druid::{AppLauncher, Data, lens, Widget, widget::{Flex, TextBox, Label, List, Button, Scroll}, WidgetExt};
use crate::commands::RUN_CODE;

#[derive(Data, Clone)]
struct AppData {
    code: String,
    file_list: Vec<String>,
}

pub fn build_ui() -> impl Widget<AppData> {
    let editor = TextBox::multiline()
        .with_placeholder("Write your Rust code here...")
        .lens(lens!(AppData, code))  // Focus on the code field of AppData
        .expand_width();

    let terminal = TextBox::multiline()
        .with_placeholder("Output will be shown here...")
        .fix_height(200.0)
        .expand_width();

    let run_button = Button::new("Run")
        .on_click(|ctx, data: &mut AppData, _env| {
            ctx.submit_command(RUN_CODE);
        });

    let file_explorer = List::new(|| {
        Label::new(|item: &String, _env: &_| item.clone())
    })
        .lens(lens!(AppData, file_list))  // Focus on the file_list field of AppData
        .scroll()
        .fix_width(200.0);

    Flex::row()
        .with_child(file_explorer)
        .with_flex_child(Flex::column()
                             .with_child(editor)
                             .with_child(terminal)
                             .with_child(run_button), 1.0)
}

