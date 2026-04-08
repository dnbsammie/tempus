use crate::app::state::AppState;

pub trait Storage {
    fn save(&self, state: &AppState);
    fn load(&self) -> Option<AppState>;
}
