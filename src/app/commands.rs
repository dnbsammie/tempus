#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Quit,
    NoOp,
}

pub fn parse(input: &str) -> Command {
    match input.trim() {
        "q" | "quit" | "exit" => Command::Quit,
        _ => Command::NoOp,
    }
}
