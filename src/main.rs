use ratatui::crossterm::{self, ExecutableCommand};

mod todo;
mod tui;

use tui::Tui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::io::stdout().execute(crossterm::event::EnableMouseCapture)?;
    let mut terminal = ratatui::init();
    Tui::new().run(&mut terminal).await?;
    ratatui::restore();
    std::io::stdout().execute(crossterm::event::DisableMouseCapture)?;
    Ok(())
}
