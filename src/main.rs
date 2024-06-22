use iced::{
    widget::{button, column, text, text_input},
    window, Application, Command, Element, Executor, Settings, Theme,
};

use savefile::{load_file, save_file};

#[macro_use]
extern crate savefile_derive;

#[derive(Clone, Debug)]
enum Message {
    Create(Todo),
    Toggle(usize),
    Delete(usize),
    TextInputChange(String),
    SaveState(Vec<Todo>),
}

#[derive(Debug, Clone, PartialEq, Savefile)]
struct Todo {
    text: String,
    is_checked: bool,
}
struct App {
    todos: Vec<Todo>,
    text_area_data: String,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();
    fn new(_flags: ()) -> (App, iced::Command<Message>) {
        let mut todo = load_data();
        todo.push(Todo {
            text: "First Task".to_string(),
            is_checked: false,
        });
        (
            Self {
                todos: todo,
                text_area_data: String::new(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Todo App")
    }
    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Create(todo) => {
                println!("{:?}", todo.clone());
                self.todos.push(todo);
                return iced::Command::none();
            }
            Message::Toggle(pos) => {
                let todo = self.todos.get_mut(pos).unwrap();
                todo.is_checked = !todo.is_checked;
                return iced::Command::none();
            }
            Message::Delete(pos) => {
                let _ = self.todos.remove(pos);
                return iced::Command::none();
            }
            Message::TextInputChange(s) => {
                self.text_area_data = s;
                return iced::Command::none();
            }
            Message::SaveState(data) => {
                //on load
                let _ = save_data(&data);
                let a = window::close::<Message>(iced::window::Id::MAIN);
                println!("{:?}", a);
                return window::close(window::Id::MAIN);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let text_area = text_input("Enter your task..", &self.text_area_data)
            .on_input(|s| Message::TextInputChange(s));
        let submit_button = button("Submit").on_press(Message::Create(Todo {
            text: self.text_area_data.clone(),
            is_checked: false,
        }));

        let save_close = button("Save and Close").on_press(Message::SaveState(self.todos.clone()));

        let mut col = column![text_area, submit_button, save_close,];

        for texts in self.todos.clone() {
            col = col.push(text(texts.text.clone()));
        }

        col.into()
    }
}
fn save_data(data: &Vec<Todo>) {
    let done = save_file("data.bin", 0, data);
    match done {
        Ok(_) => {
            println!("Saved to data.bin");
        }
        Err(err) => {
            println!("Coudln't Save to data.bin\n Err: {:?}", err);
        }
    }
}

fn load_data() -> Vec<Todo> {
    let done = load_file("data.bin", 0);
    match done {
        Ok(values) => {
            println!("Data loaded from data.bin");
            return values;
        }
        Err(err) => {
            print!("Couldn't load data\n Err: {}\nMaking an empty Todo", err);
            return Vec::new();
        }
    }
}

fn main() {
    let _ = App::run(Settings::default());
}
