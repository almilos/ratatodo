use ratatui::style::{Color, Style}; // https://ratatui.rs/examples/style/
use ratatui::widgets::{Block, Paragraph}; // https://ratatui.rs/concepts/widgets/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // https://docs.rs/ratatui/latest/ratatui/fn.run.html
    ratatui::run(|terminal| {
        terminal.draw(|frame| {
            let block = Block::bordered().title("Hello");
            let greeting = Paragraph::new("world!")
                .centered()
                .style(Style::default().fg(Color::Yellow))
                .block(block);
            frame.render_widget(greeting, frame.area());
        })?;
        std::thread::sleep(std::time::Duration::from_secs(5));
        Ok(())
    })
}
