use crossterm::event::{self, Event, KeyEvent};

pub enum AppEvent { Input(KeyEvent), Tick, }

pub fn next_event() -> AppEvent {
    if event::poll(std::time::Duration::from_millis(250)).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            return AppEvent::Input(key);
        }
    }
    AppEvent::Tick
}