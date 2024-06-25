use crate::todo::{Todo, Todos};

//Change usize to int or find a better implementation
#[derive(Debug, Clone)]
pub enum Message {
    Create(Todo),
    TextInputChange(String),
    ToggleTheme(iced::Theme),
    DeleteTodo(usize),
    SaveAndClose(Todos),
}
