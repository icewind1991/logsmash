use crate::app::App;
use crate::error::UiError;
use crate::ui::error_list::error_list;
use crate::ui::footer::footer;
use crate::ui::grouped_logs::grouped_logs;
use crate::ui::histogram::UiHistogram;
use crate::ui::input::handle_events;
use crate::ui::match_list::match_list;
use crate::ui::single_log::single_log;
use crate::ui::single_match::grouped_lines;
use crate::ui::state::{
    ErrorState, GroupedLogsState, LogState, MatchListState, MatchState, UiState,
};
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::crossterm::ExecutableCommand;
use ratatui::prelude::*;
use ratatui::Terminal;
use std::io;
use std::io::stdout;
use std::panic::{set_hook, take_hook};

mod error_list;
mod footer;
mod grouped_logs;
mod histogram;
mod input;
mod match_list;
mod single_log;
mod single_match;
mod state;
pub mod style;
mod table;

pub fn run_ui(app: App) -> Result<(), UiError> {
    init_panic_hook();
    let mut terminal = init_tui()?;
    terminal.clear().ok();

    let mut ui_state = UiState::new(&app);
    let mut update = true;

    while !matches!(ui_state, UiState::Quit) {
        if update {
            terminal.draw(|frame| ui(frame, &app, &mut ui_state))?;
        }
        update = false;
        if let Some(event) = handle_events(ui_state.page(), &ui_state)? {
            (update, ui_state) = ui_state.process(event, &app);
        }
    }

    Ok(())
}

pub fn init_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        // intentionally ignore errors here since we're already in a panic
        let _ = restore_tui();
        original_hook(panic_info);
    }));
}

pub fn init_tui() -> io::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore_tui() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(DisableMouseCapture)?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn find_hit_row(row: u16, ui_state: &UiState) -> Option<usize> {
    if let Some(table_row) = row.checked_sub(ui_state.content_offset()) {
        let selected = ui_state.index_for_row(table_row as usize);
        if selected < ui_state.row_count() {
            Some(selected)
        } else {
            None
        }
    } else {
        None
    }
}

const UI_HEADER_SIZE: u16 = 5;

fn ui(frame: &mut Frame, app: &App, state: &mut UiState) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(UI_HEADER_SIZE),
            Constraint::Percentage(100),
            Constraint::Length(1),
        ])
        .split(frame.area());

    match state {
        UiState::Quit => {}
        UiState::MatchList(MatchListState {
            table_state,
            filter,
            ..
        }) => {
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
            frame.render_stateful_widget(match_list(app, filter), layout[1], table_state);
            frame.render_widget(footer(app, state.footer_params()), layout[2]);
        }
        UiState::Match(MatchState {
            result,
            table_state,
            filter,
            ..
        }) => {
            let selected = table_state.selected();
            let selected_group = if selected == 0 {
                &result.all
            } else {
                &result.grouped[selected - 1]
            };

            frame.render_widget(UiHistogram::new(&selected_group.histogram), layout[0]);
            frame.render_stateful_widget(
                grouped_lines(app, result, filter),
                layout[1],
                table_state,
            );
            frame.render_widget(footer(app, state.footer_params()), layout[2]);
        }
        UiState::GroupedLogs(GroupedLogsState {
            lines,
            table_state,
            filter,
            ..
        }) => {
            frame.render_stateful_widget(
                grouped_logs(app, lines, filter),
                layout[0].union(layout[1]),
                table_state,
            );
            frame.render_widget(footer(app, state.footer_params()), layout[2]);
        }
        UiState::Log(LogState {
            table_state,
            full_line,
            ..
        }) => {
            frame.render_stateful_widget(
                single_log(full_line),
                layout[0].union(layout[1]),
                table_state,
            );
            frame.render_widget(footer(app, state.footer_params()), layout[2]);
        }
        UiState::Errors(ErrorState { table_state, .. }) => {
            frame.render_stateful_widget(error_list(app), layout[0].union(layout[1]), table_state);
            frame.render_widget(footer(app, state.footer_params()), layout[2]);
        }
    }
}
