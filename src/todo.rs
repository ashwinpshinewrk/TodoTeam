#[derive(Savefile, Debug, Clone)]
pub struct Todo {
    pub text: String,
    pub is_checked: bool,
}

impl Todo {
    pub fn new(text: String) -> Self {
        Self {
            text,
            is_checked: false,
        }
    }
}

pub type Todos = Vec<Todo>;
