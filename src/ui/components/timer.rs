use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, area: Rect, content: &str) {
    let widget = Paragraph::new(content).block(Block::default().title("Timer").borders(Borders::ALL));
    frame.render_widget(widget, area);
}
