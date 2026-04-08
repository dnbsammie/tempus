use ratatui::{
    Frame,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::state::AppState;

pub fn render(frame: &mut Frame, state: &AppState) {
    let area = frame.area();
    let title = match state.current_view {
        crate::app::state::View::Home => "Tempus - Home",
        crate::app::state::View::Config => "Tempus - Config",
        crate::app::state::View::Session => "Tempus - Session",
    };
    let content = Paragraph::new("Primera ejecucion lista.\n\nq: salir\nh: home\ns: session")
        .block(Block::default().title(title).borders(Borders::ALL));
    frame.render_widget(content, area);
}
