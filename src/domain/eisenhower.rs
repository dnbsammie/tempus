#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quadrant {
    UrgentImportant,
    NotUrgentImportant,
    UrgentNotImportant,
    NotUrgentNotImportant,
}
