use ratatui::Frame;

use crate::app::state::AppState;

pub fn render(frame: &mut Frame, state: &AppState) {
    super::home::render(frame, state);
}
