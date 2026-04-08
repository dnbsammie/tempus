pub mod components;
pub mod events;
pub mod layout;
pub mod views;

use ratatui::Frame;

use crate::app::state::{AppState, View};

pub fn render(frame: &mut Frame, state: &AppState) {
    match state.current_view {
        View::Home | View::Config | View::Session => views::home::render(frame, state),
    }
}
