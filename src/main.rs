use ratatui::{
    DefaultTerminal,
    Frame,
    crossterm::event::{self}, // https://docs.rs/crossterm/latest/crossterm/event/index.html
    layout::{
        Constraint::{Fill, Length, Percentage},
        Layout,
    },
    style::{Color, Style},
    widgets::{Block, List, ListDirection, Paragraph},
};

// структура, отвечающая за работу нашего tui
struct Tui {}

impl Tui {
    fn new() -> Self {
        Self {}
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        terminal.draw(|frame| self.render(frame))?;

        // вместо sleep
        // ожидаем любой ввод и выходим из приложения
        let event = event::read()?; //https://docs.rs/crossterm/latest/crossterm/event/index.html

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

        let status = Paragraph::new("status").block(block_status);

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
