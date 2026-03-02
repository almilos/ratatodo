use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    layout::{
        Constraint::{Fill, Length, Percentage},
        Layout,
    },
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, List, ListDirection, ListState, Paragraph},
};

enum TuiState {
    Idle,
    ShouldExit,
}

struct Tui {
    state: TuiState, // состояние приложения
    todo: TodoList,
}

// Список задач
struct TodoList {
    state: ListState, // https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html состояние списка
    list: Vec<TodoItem>, // Вектор задач
}

impl Default for TodoList {
    fn default() -> Self {
        let mut list = Vec::new();
        list.push(TodoItem::new("item1", "content1"));
        list.push(TodoItem::new("item2", "content2"));
        list.push(TodoItem::new("item3", "content3"));
        list.push(TodoItem::new("item4", "content4"));
        TodoList {
            state: ListState::default(),
            list,
        }
    }
}

struct TodoItem {
    done: bool,   // выполнен ли пункт
    name: String, // название
    desc: String, // описание
}

impl TodoItem {
    fn new(name: &str, desc: &str) -> Self {
        Self {
            done: false,
            name: name.to_string(),
            desc: desc.to_string(),
        }
    }
}
impl Tui {
    fn new() -> Self {
        Self {
            state: TuiState::Idle,
            todo: TodoList::default(),
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.state = TuiState::ShouldExit,
            _ => {}
        }
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        while !matches!(self.state, TuiState::ShouldExit) {
            terminal.draw(|frame| self.render(frame))?;
            let event = event::read()?;
            match event {
                Event::Key(key) => self.handle_key(key),
                _ => {}
            }
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let block_list = Block::bordered().title("List");
        let block_content = Block::bordered()
            .title("Content")
            .border_style(Style::default().fg(Color::Cyan));

        let block_status = Block::bordered().border_style(Style::default().fg(Color::Cyan));

        let items: Vec<_> = self
            .todo
            .list
            .iter() // собираем вектор Line для того, чтобы передать их в список
            .map(|item| {
                let mut line = Line::from(item.name.clone());
                if item.done == true {
                    line = line.crossed_out(); // зачеркиваем текст если пункт выполнен
                };
                line
            })
            .collect();

        let list = List::new(items)
            .block(block_list)
            .highlight_style(Style::new().bg(Color::Red))
            .direction(ListDirection::TopToBottom);

        let content = Paragraph::new("content")
            .left_aligned()
            .block(block_content);

        let status = Paragraph::new("[Esc] to exit").block(block_status);

        let chunks_v = Layout::vertical(&[Fill(1), Length(3)]).split(frame.area());
        let chunks_h = Layout::horizontal(&[Percentage(20), Percentage(80)]).split(chunks_v[0]);

        frame.render_stateful_widget(list, chunks_h[0], &mut self.todo.state); // https://docs.rs/ratatui/latest/ratatui/prelude/trait.StatefulWidget.html
        frame.render_widget(content, chunks_h[1]);
        frame.render_widget(status, chunks_v[1]);
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| Tui::new().run(terminal))
}
