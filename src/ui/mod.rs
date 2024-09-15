use crate::app::App;
use crate::error::UiError;
use crate::ui::error_list::error_list;
use crate::ui::footer::footer;
use crate::ui::histogram::UiHistogram;
use crate::ui::match_list::match_list;
use crate::ui::raw_logs::raw_logs;
use crate::ui::single_log::single_log;
use crate::ui::single_match::grouped_lines;
use crate::ui::state::{
    ErrorState, LogState, LogsState, MatchListState, MatchState, UiEvent, UiPage, UiState,
};
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseEventKind};
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::crossterm::{event, ExecutableCommand};
use ratatui::prelude::*;
use ratatui::Terminal;
use std::io;
use std::io::stdout;
use std::time::Duration;

mod error_list;
mod footer;
mod histogram;
mod match_list;
mod raw_logs;
mod single_log;
mod single_match;
mod state;
pub mod style;
mod table;

pub fn run_ui(app: App) -> Result<(), UiError> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear().ok();

    let mut ui_state = UiState::new(&app);
    let mut update = true;

    while !matches!(ui_state, UiState::Quit) {
        if update {
            terminal.draw(|frame| ui(frame, &app, &mut ui_state))?;
        }
        update = false;
        if let Some(event) = handle_events(ui_state.page())? {
            (update, ui_state) = ui_state.process(event, &app);
        }
    }

    disable_raw_mode()?;
    stdout().execute(DisableMouseCapture)?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(page: UiPage) -> io::Result<Option<UiEvent>> {
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key) if key.kind == event::KeyEventKind::Press=> {
                return Ok(match key.code {
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                        Some(UiEvent::Quit)
                    }
                    KeyCode::Char('q') => Some(UiEvent::Quit),
                    KeyCode::Esc => Some(UiEvent::Back),
                    KeyCode::Char('e') if page == UiPage::MatchList => Some(UiEvent::Errors),
                    KeyCode::Left if page != UiPage::MatchList => Some(UiEvent::Back),
                    KeyCode::Down => Some(UiEvent::Down(1, true)),
                    KeyCode::Up => Some(UiEvent::Up(1, true)),
                    KeyCode::PageDown => Some(UiEvent::Down(10, false)),
                    KeyCode::PageUp => Some(UiEvent::Up(10, false)),
                    KeyCode::End => Some(UiEvent::Down(usize::MAX, false)),
                    KeyCode::Home => Some(UiEvent::Up(usize::MAX, false)),
                    KeyCode::Enter | KeyCode::Right => Some(UiEvent::Select),
                    KeyCode::Char('c') => Some(UiEvent::Copy),
                    _ => None,
                });
            }
            Event::Mouse(mouse) => {
                return Ok(match mouse.kind {
                    MouseEventKind::ScrollUp => Some(UiEvent::Up(1, false)),
                    MouseEventKind::ScrollDown => Some(UiEvent::Down(1, false)),
                    _ => None,
                })
            },
            _ => {}
        }
    }
    Ok(None)
}

const UI_HEADER_SIZE: u16 = 5;

fn ui(frame: &mut Frame, app: &App, state: &mut UiState) {
    let page = state.page();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(UI_HEADER_SIZE),
            Constraint::Percentage(100),
            Constraint::Length(1),
        ])
        .split(frame.size());

    match state {
        UiState::Quit => {}
        UiState::MatchList(MatchListState { table_state }) => {
            let selected = table_state.selected();
            let histogram = if selected == 0 {
                &app.all.histogram
            } else if selected < app.matches.len() + 1 {
                let log_match = &app.matches[selected - 1];
                &log_match.histogram
            } else {
                &app.unmatched.histogram
            };

            frame.render_widget(UiHistogram::new(histogram), layout[0]);
            frame.render_stateful_widget(match_list(app), layout[1], table_state);
            frame.render_widget(footer(app, page), layout[2]);
        }
        UiState::Match(MatchState {
            result,
            table_state,
            ..
        }) => {
            let selected_group = &result.grouped[table_state.selected()];

            frame.render_widget(UiHistogram::new(&selected_group.histogram), layout[0]);
            frame.render_stateful_widget(grouped_lines(app, result), layout[1], table_state);
            frame.render_widget(footer(app, page), layout[2]);
        }
        UiState::Logs(LogsState {
            lines, table_state, ..
        }) => {
            frame.render_stateful_widget(
                raw_logs(app, lines),
                layout[0].union(layout[1]),
                table_state,
            );
            frame.render_widget(footer(app, page), layout[2]);
        }
        UiState::Log(LogState {
            log, table_state, ..
        }) => {
            frame.render_stateful_widget(
                single_log(app, log),
                layout[0].union(layout[1]),
                table_state,
            );
            frame.render_widget(footer(app, page), layout[2]);
        }
        UiState::Errors(ErrorState { table_state, .. }) => {
            frame.render_stateful_widget(error_list(app), layout[0].union(layout[1]), table_state);
            frame.render_widget(footer(app, page), layout[2]);
        }
    }
}
