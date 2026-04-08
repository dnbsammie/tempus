use uuid::Uuid;

use crate::domain::eisenhower::Quadrant;

#[derive(Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub priority: Priority,
}

impl Task {
    pub fn new(title: impl Into<String>, priority: Priority) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            priority,
        }
    }
}

#[derive(Clone)]
pub enum Priority {
    UrgentImportant,
    NotUrgentImportant,
    UrgentNotImportant,
    NotUrgentNotImportant,
}

impl From<Priority> for Quadrant {
    fn from(value: Priority) -> Self {
        match value {
            Priority::UrgentImportant => Quadrant::UrgentImportant,
            Priority::NotUrgentImportant => Quadrant::NotUrgentImportant,
            Priority::UrgentNotImportant => Quadrant::UrgentNotImportant,
            Priority::NotUrgentNotImportant => Quadrant::NotUrgentNotImportant,
        }
    }
}
