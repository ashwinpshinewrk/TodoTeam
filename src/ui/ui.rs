use iced::{
    widget::{self, column, row},
    Element, Theme,
};

use crate::{app::App, message::Message, todo::Todo, WINDOW_WIDTH};

//None for FULL_WIDTH
fn get_percent(percent: u16, width: Option<u16>) -> u16 {
    match width {
        Some(w) => (w * percent) / 100,
        None => (WINDOW_WIDTH * percent) / 100,
    }
}

pub fn display<'a>(app: &'a App) -> Element<'a, Message> {
    let todo_input_area = widget::text_input("Take a walk", &app.text_buffer)
        .on_input(|s| Message::TextInputChange(s))
        .on_submit(Message::Create(Todo::new(app.text_buffer.clone())))
        .padding(5);
    let todo_submit_btn =
        widget::button("Add").on_press(Message::Create(Todo::new(app.text_buffer.clone())));

    let theme_selector = widget::pick_list(&Theme::ALL[..], app.selected_theme.clone(), |theme| {
        Message::ToggleTheme(theme)
    });
    let save_btn =
        widget::button("Save and Close").on_press(Message::SaveAndClose(app.todo_data.clone()));
    let text_row = row![todo_input_area, todo_submit_btn, theme_selector, save_btn]
        .spacing(10)
        .width(get_percent(80, None))
        .align_items(iced::Alignment::Center);

    let todo_add = column![text_row].align_items(iced::Alignment::Center);

    let mut todos = column![];
    for (key, data) in app.todo_data.iter().enumerate() {
        let mut row = row![].spacing(20);
        row = row.push(widget::Space::with_height(50));
        row = row.push(widget::text(data.text.clone()));

        row = row.push(widget::button("Delete").on_press(Message::DeleteTodo(key)));
        todos = todos.push(row);
    }
    let spacing_above = widget::Space::with_height(40);
    let todos_scroll = iced::widget::scrollable(todos);

    column!(todo_add, spacing_above, todos_scroll)
        .align_items(iced::Alignment::Center)
        .into()
}
