use crate::domain::{task::Task, session::Session};

pub enum View { Home, Config, Session, }

pub struct AppState {
    pub current_view: View,
    pub tasks: Vec<Task>,
    pub session: Option<Session>,
    pub selected_task: usize,
    pub running: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_view: View::Home,
            tasks: Vec::new(),
            session: None,
            selected_task: 0,
            running: true,
        }
    }
}
