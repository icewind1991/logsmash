use crate::app::App;
use crate::error::UiError;
use crate::ui::footer::footer;
use crate::ui::histogram::UiHistogram;
use crate::ui::match_list::match_list;
use crate::ui::raw_logs::raw_logs;
use crate::ui::single_match::grouped_lines;
use crate::ui::state::{UiEvent, UiState};
use ratatui::crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::crossterm::{event, ExecutableCommand};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Scrollbar, ScrollbarOrientation};
use ratatui::Terminal;
use std::io;
use std::io::stdout;

mod footer;
mod histogram;
mod match_list;
mod raw_logs;
mod single_match;
mod state;
pub mod style;

pub fn run_ui(app: App) -> Result<(), UiError> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut ui_state = UiState::new(&app);

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
                    KeyCode::Esc | KeyCode::Left => Some(UiEvent::Back),
                    KeyCode::Down => Some(UiEvent::Down(1)),
                    KeyCode::Up => Some(UiEvent::Up(1)),
                    KeyCode::PageDown => Some(UiEvent::Down(10)),
                    KeyCode::PageUp => Some(UiEvent::Up(10)),
                    KeyCode::Enter | KeyCode::Right => Some(UiEvent::Select),
                    _ => None,
                });
            }
        }
    }
    Ok(None)
}

fn ui(frame: &mut Frame, app: &App, state: &mut UiState) {
    let page = state.page();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(5),
            Constraint::Percentage(100),
            Constraint::Length(1),
        ])
        .split(frame.size());

    match state {
        UiState::Quit => {}
        UiState::MatchList {
            table_state,
            scroll_state,
        } => {
            let selected = table_state.selected().unwrap_or(0);
            let histogram = if selected == 0 {
                &app.all.histogram
            } else if selected < app.matches.len() + 1 {
                let log_match = &app.matches[selected - 1];
                &log_match.histogram
            } else {
                &app.unmatched.histogram
            };
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            frame.render_widget(UiHistogram::new(histogram), layout[0]);
            frame.render_stateful_widget(
                match_list(app).block(Block::new().borders(Borders::RIGHT)),
                layout[1],
                table_state,
            );
            frame.render_stateful_widget(
                scrollbar,
                layout[1].inner(Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                scroll_state,
            );
            frame.render_widget(footer(app, page), layout[2]);
        }
        UiState::Match {
            result,
            table_state,
            scroll_state,
            ..
        } => {
            let selected_group = &result.grouped[table_state.selected().unwrap_or_default()];
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            frame.render_widget(UiHistogram::new(&selected_group.histogram), layout[0]);
            frame.render_stateful_widget(
                grouped_lines(app, result).block(Block::new().borders(Borders::RIGHT)),
                layout[1],
                table_state,
            );
            frame.render_stateful_widget(
                scrollbar,
                layout[1].inner(Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                scroll_state,
            );
            frame.render_widget(footer(app, page), layout[2]);
        }
        UiState::Logs {
            lines,
            table_state,
            scroll_state,
            ..
        } => {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));
            frame.render_stateful_widget(
                raw_logs(app, lines).block(Block::new().borders(Borders::RIGHT)),
                layout[0].union(layout[1]),
                table_state,
            );
            frame.render_stateful_widget(
                scrollbar,
                layout[0].union(layout[1]).inner(Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                scroll_state,
            );
            frame.render_widget(footer(app, page), layout[2]);
        }
    }
}
