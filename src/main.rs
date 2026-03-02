use ratatui::{
    layout::{
        Constraint::{Fill, Length, Percentage}, // https://ratatui.rs/examples/layout/constraints/
        Layout,                                 // https://ratatui.rs/concepts/layout/
    },
    style::{Color, Style},
    widgets::{Block, List, ListDirection, Paragraph},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| {
        terminal.draw(|frame| {
            let block_list = Block::bordered().title("List"); // блок со списком задач

            let block_content = Block::bordered() // блок с описанием задачи
                .title("Content")
                .border_style(Style::default().fg(Color::Cyan));

            let block_status = Block::bordered().border_style(Style::default().fg(Color::Cyan)); // блок со статус-баром

            let items = ["item1", "item2", "item3"]; // заглушка для списка

            let list = List::new(items) // https://ratatui.rs/examples/widgets/list/
                .block(block_list)
                .highlight_style(Style::new().bg(Color::Red))
                .direction(ListDirection::TopToBottom);

            let content = Paragraph::new("content") // заглушка для описания задачи
                .left_aligned()
                .block(block_content);

            let status = Paragraph::new("status").block(block_status); // заглушка для статуса

            let chunks_v = Layout::vertical(&[Fill(1), Length(3)]).split(frame.area()); // разделяем экран вертикально
            let chunks_h = Layout::horizontal(&[Percentage(20), Percentage(80)]).split(chunks_v[0]); // разделяем экран горизонтально

            frame.render_widget(list, chunks_h[0]);
            frame.render_widget(content, chunks_h[1]);
            frame.render_widget(status, chunks_v[1]);
        })?;
        std::thread::sleep(std::time::Duration::from_secs(5));
        Ok(())
    })
}
