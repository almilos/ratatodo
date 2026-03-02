use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    layout::{
        Constraint::{Fill, Length, Percentage},
        Layout,
    },
    style::{Color, Style},
    widgets::{Block, List, ListDirection, Paragraph},
};

// состояние приложения, пока обозначаем нужно ли выходить или нет
enum TuiState {
    Idle,
    ShouldExit,
}

struct Tui {
    state: TuiState,
}

impl Tui {
    fn new() -> Self {
        Self {
            state: TuiState::Idle,
        }
    }

    // https://docs.rs/crossterm/latest/crossterm/event/struct.KeyEvent.html
    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.state = TuiState::ShouldExit,
            _ => {}
        }
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        // main loop
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

        let items = ["item1", "item2", "item3"];

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

        frame.render_widget(list, chunks_h[0]);
        frame.render_widget(content, chunks_h[1]);
        frame.render_widget(status, chunks_v[1]);
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| Tui::new().run(terminal))
}
