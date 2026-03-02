use ratatui::widgets::ListState;

pub struct TodoList {
    pub state: ListState,
    pub list: Vec<TodoItem>,
}

impl Default for TodoList {
    fn default() -> Self {
        let list = vec![
            TodoItem::new("item1", "content1"),
            TodoItem::new("item2", "content2"),
            TodoItem::new("item3", "content3"),
            TodoItem::new("item4", "content4"),
        ];
        TodoList {
            state: ListState::default(),
            list,
        }
    }
}

pub struct TodoItem {
    pub done: bool,
    pub name: String,
    pub desc: String,
}

impl TodoItem {
    pub fn new(name: &str, desc: &str) -> Self {
        Self {
            done: false,
            name: name.to_string(),
            desc: desc.to_string(),
        }
    }
}
