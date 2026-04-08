use crossterm::event::KeyCode;

use crate::app::state::{AppState, View};
use crate::ui::events::AppEvent;

pub fn update(state: &mut AppState, event: AppEvent) {
    match event {
        AppEvent::Input(key) => match key.code {
            KeyCode::Char('q') => state.running = false,
            KeyCode::Char('h') => state.current_view = View::Home,
            KeyCode::Char('s') => state.current_view = View::Session,
            _ => {}
        },
        AppEvent::Tick => {
            if let Some(session) = &mut state.session {
                session.tick();
            }
        }
    }
}
