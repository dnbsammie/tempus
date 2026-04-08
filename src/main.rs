mod app;
mod cli;
mod domain;
mod infrastructure;
mod ui;
mod utils;

use std::io;

use app::{controller::update, state::AppState};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{DefaultTerminal, Terminal, backend::CrosstermBackend};
use ui::events::next_event;

fn main() -> anyhow::Result<()> {
    color_eyre::install().map_err(|error| anyhow::anyhow!(error.to_string()))?;
    let mut terminal = setup_terminal()?;
    let result = run(&mut terminal);
    restore_terminal(&mut terminal)?;
    result
}

fn run(terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
    let mut state = AppState::new();
    while state.running {
        terminal.draw(|frame| ui::render(frame, &state))?;
        let event = next_event();
        update(&mut state, event);
    }
    Ok(())
}

fn setup_terminal() -> anyhow::Result<DefaultTerminal> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
