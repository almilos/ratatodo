use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        ExecutableCommand,
        event::{self, Event, KeyCode, KeyEvent, MouseEvent},
    },
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
    EnteringItemName,
    EnteringItemDesc,
    ShouldExit,
}

struct Tui {
    state: TuiState,
    todo: TodoList,

    cur_long: String,
    cur_short: String,
}

struct TodoList {
    state: ListState,
    list: Vec<TodoItem>,
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
    done: bool,
    name: String,
    desc: String,
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
            cur_long: String::new(),
            cur_short: String::new(),
        }
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) {
        if mouse.kind.is_down() {
            let i = (mouse.row.saturating_sub(1)) as usize;
            if i < self.todo.list.len() {
                self.todo.state.select(i.into())
            }
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) if matches!(self.state, TuiState::EnteringItemName) => {
                self.cur_short.push_str(&c.to_string())
            }

            KeyCode::Char(c) if matches!(self.state, TuiState::EnteringItemDesc) => {
                self.cur_long.push_str(&c.to_string())
            }

            KeyCode::Backspace if matches!(self.state, TuiState::EnteringItemName) => {
                self.cur_short.pop();
            }

            KeyCode::Backspace if matches!(self.state, TuiState::EnteringItemDesc) => {
                self.cur_long.pop();
            }

            KeyCode::Backspace if matches!(self.state, TuiState::Idle) => {
                if let Some(i) = self.todo.state.selected()
                    && i < self.todo.list.len()
                {
                    self.todo.list[i].done = !self.todo.list[i].done;
                };
            }

            KeyCode::Esc => self.state = TuiState::ShouldExit,

            KeyCode::Down => self.todo.state.select_next(),

            KeyCode::Up => self.todo.state.select_previous(),

            KeyCode::Enter => match self.state {
                TuiState::Idle => self.state = TuiState::EnteringItemName,
                TuiState::EnteringItemName => self.state = TuiState::EnteringItemDesc,
                TuiState::EnteringItemDesc => {
                    self.state = TuiState::Idle;
                    self.todo
                        .list
                        .push(TodoItem::new(&self.cur_short, &self.cur_long));
                    self.cur_long = String::new();
                    self.cur_short = String::new();
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        while !matches!(self.state, TuiState::ShouldExit) {
            terminal.draw(|frame| self.render(frame))?;
            let event = event::read()?;
            match event {
                Event::Key(key) => self.handle_key(key),
                Event::Mouse(mouse) => self.handle_mouse(mouse),
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
            .iter()
            .map(|item| {
                let mut line = Line::from(item.name.clone());
                if item.done == true {
                    line = line.crossed_out();
                };
                line
            })
            .collect();

        let list = List::new(items)
            .block(block_list)
            .highlight_style(Style::new().bg(Color::Red))
            .direction(ListDirection::TopToBottom);

        let text = if let Some(i) = self.todo.state.selected()
            && i < self.todo.list.len()
        {
            self.todo.list[i].desc.clone()
        } else {
            String::new()
        };

        let content = Paragraph::new(text)
            .left_aligned()
            .white()
            .block(block_content);

        let status_text = match self.state {
            TuiState::EnteringItemName => &format!("Item name: {}", &self.cur_short),
            TuiState::EnteringItemDesc => &format!("Item description: {}", &self.cur_long),
            _ => "[Enter] to enter item title, [Enter] again to add description, [Esc] to exit",
        };

        let status = Paragraph::new(status_text).block(block_status);

        let chunks_v = Layout::vertical(&[Fill(1), Length(3)]).split(frame.area());
        let chunks_h = Layout::horizontal(&[Percentage(20), Percentage(80)]).split(chunks_v[0]);

        frame.render_stateful_widget(list, chunks_h[0], &mut self.todo.state);
        frame.render_widget(content, chunks_h[1]);
        frame.render_widget(status, chunks_v[1]);
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::io::stdout().execute(crossterm::event::EnableMouseCapture)?;
    ratatui::run(|terminal| Tui::new().run(terminal))?;
    std::io::stdout().execute(crossterm::event::DisableMouseCapture)?;
    Ok(())
}
