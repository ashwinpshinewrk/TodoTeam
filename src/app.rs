use iced::Application;

use crate::{
    file::{load_data, save_data, TODO_DATA_FILE},
    message::Message,
    todo::Todos,
    ui::ui::display,
};

//TEMP
use iced::theme::Theme;
pub struct App {
    pub todo_data: Todos,
    pub text_buffer: String,
    pub selected_theme: Option<Theme>,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                todo_data: load_data(TODO_DATA_FILE),
                text_buffer: String::new(),

                selected_theme: Some(Theme::Dark),
            },
            iced::Command::none(),
        )
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Create(todo) => {
                self.todo_data.push(todo);
                //Clears everything
                self.text_buffer.clear();
                return iced::Command::none();
            }
            Message::TextInputChange(s) => {
                self.text_buffer = s;
                return iced::Command::none();
            }
            Message::ToggleTheme(theme) => {
                self.selected_theme = Some(theme);
                return iced::Command::none();
            }
            Message::DeleteTodo(index) => {
                self.todo_data.remove(index);
                return iced::Command::none();
            }
            Message::SaveAndClose(data) => {
                save_data(&data, TODO_DATA_FILE);
                return iced::window::close(iced::window::Id::MAIN);
            }
        }
    }
    fn title(&self) -> String {
        String::from("TodoTeam")
    }

    fn theme(&self) -> Self::Theme {
        self.selected_theme.clone().unwrap()
    }
    fn view(&self) -> iced::Element<Message> {
        // FOR THE COMPILER
        display(self)
        //
    }
}
