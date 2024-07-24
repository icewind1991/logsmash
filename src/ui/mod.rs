use crate::app::App;
use crate::error::UiError;
use crate::ui::match_list::match_list;
use crate::ui::state::{UiEvent, UiState};
use ratatui::crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::crossterm::{event, ExecutableCommand};
use ratatui::prelude::*;
use ratatui::Terminal;
use std::io;
use std::io::stdout;

mod match_list;
mod state;

pub fn run_ui(app: App) -> Result<(), UiError> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut ui_state = UiState::default();

    while !matches!(ui_state, UiState::Quit) {
        terminal.draw(|frame| ui(frame, &app, &mut ui_state))?;
        if let Some(event) = handle_events()? {
            ui_state = ui_state.process(event, &app);
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<Option<UiEvent>> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(match key.code {
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                        Some(UiEvent::Quit)
                    }
                    KeyCode::Char('q') => Some(UiEvent::Quit),
                    KeyCode::Esc => Some(UiEvent::Back),
                    KeyCode::Down => Some(UiEvent::Down),
                    KeyCode::Up => Some(UiEvent::Up),
                    _ => None,
                });
            }
        }
    }
    Ok(None)
}

fn ui(frame: &mut Frame, app: &App, state: &mut UiState) {
    match state {
        UiState::Quit => {}
        UiState::MatchList { table_state } => {
            frame.render_stateful_widget(match_list(app), frame.size(), table_state);
        }
    }
}
