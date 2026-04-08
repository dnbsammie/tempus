use crate::domain::{task::Task, timer::Timer};

pub struct Session { pub tasks: Vec<Task>, pub current: usize, pub timer: Timer, }

impl Session {
    pub fn tick(&mut self) {
        let _ = self.timer.remaining();
    }
}
