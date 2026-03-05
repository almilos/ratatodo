use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, MouseEvent};
use futures::{StreamExt, future::FutureExt};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{
        Constraint::{Fill, Length, Percentage},
        Layout,
    },
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, List, ListDirection, Paragraph},
};
use throbber_widgets_tui::{BRAILLE_DOUBLE, Throbber, ThrobberState, WhichUse};
use tokio_util::sync::CancellationToken;

use super::todo::{TodoItem, TodoList};

enum TuiState {
    Idle,
    EnteringItemName,
    EnteringItemDesc,
}

pub struct Tui {
    state: TuiState,
    todo: TodoList,

    cur_desc: String,
    cur_name: String,

    cancel_token: CancellationToken, // индикатор необходимости завершения работы

    throbber_state: ThrobberState, // state для анимации
}

impl Tui {
    pub fn new() -> Self {
        Self {
            state: TuiState::Idle,
            todo: TodoList::default(),
            cur_desc: String::new(),
            cur_name: String::new(),
            cancel_token: CancellationToken::new(),
            throbber_state: ThrobberState::default(),
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
                self.cur_name.push(c)
            }

            KeyCode::Char(c) if matches!(self.state, TuiState::EnteringItemDesc) => {
                self.cur_desc.push(c)
            }

            KeyCode::Backspace if matches!(self.state, TuiState::EnteringItemName) => {
                self.cur_name.pop();
            }

            KeyCode::Backspace if matches!(self.state, TuiState::EnteringItemDesc) => {
                self.cur_desc.pop();
            }

            KeyCode::Delete => {
                if let Some(i) = self.todo.state.selected()
                    && i < self.todo.list.len()
                {
                    self.todo.list[i].done = !self.todo.list[i].done;
                };
            }

            KeyCode::Esc => self.cancel_token.cancel(),

            KeyCode::Down => self.todo.state.select_next(),

            KeyCode::Up => self.todo.state.select_previous(),

            KeyCode::Enter => match self.state {
                TuiState::Idle => self.state = TuiState::EnteringItemName,
                TuiState::EnteringItemName => self.state = TuiState::EnteringItemDesc,
                TuiState::EnteringItemDesc => {
                    self.state = TuiState::Idle;
                    self.todo
                        .list
                        .push(TodoItem::new(&self.cur_name, &self.cur_desc));
                    self.cur_desc = String::new();
                    self.cur_name = String::new();
                }
            },
            _ => {}
        }
    }

    pub async fn run(
        mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut event_stream = EventStream::new();

        // Main loop
        loop {
            tokio::select! {
                biased;

                _ = self.cancel_token.cancelled() => {
                    break Ok(()); // выходим, если выставлен индикатор необходимости выхода
                }

                _ = tokio::time::sleep(tokio::time::Duration::from_millis(40)) => {
                    terminal.draw(|frame|self.render(frame))?; // рисуем наш туи каждые 40 мс
                },

                // Обработка ввода
                crossterm_event = event_stream.next().fuse() => {
                    if let Some(Ok(event)) = crossterm_event {
                        match event {
                            Event::Key(key) => { self.handle_key(key); },
                            Event::Mouse(mouse) => { self.handle_mouse(mouse); },
                            _ => {}
                        }
                    } else {
                        panic!("crossterm error");
                    }
                },
            }
        }
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
                if item.done {
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
            TuiState::EnteringItemName => &format!("Item name: {}", &self.cur_name),
            TuiState::EnteringItemDesc => &format!("Item description: {}", &self.cur_desc),
            _ => "[Enter] to enter item title, [Enter] again to add description, [Esc] to exit",
        };

        let status = Paragraph::new(status_text).block(block_status);

        let chunks_v = Layout::vertical([Fill(1), Length(3)]).split(frame.area());
        let chunks_h = Layout::horizontal([Percentage(20), Percentage(80)]).split(chunks_v[0]);

        frame.render_stateful_widget(list, chunks_h[0], &mut self.todo.state);
        frame.render_widget(content, chunks_h[1]);
        frame.render_widget(status, chunks_v[1]);

        // Анимация
        let throbber_chunk = Layout::horizontal([Fill(1), Length(4)]).split(chunks_v[1])[1];
        self.throbber_state.calc_next();

        let throb = Throbber::default()
            .throbber_set(BRAILLE_DOUBLE)
            .use_type(WhichUse::Spin);

        frame.render_stateful_widget(throb, throbber_chunk, &mut self.throbber_state);
    }
}
