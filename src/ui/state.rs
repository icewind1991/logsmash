use std::iter::once;
use crate::app::{App, LogMatch};
use crate::logline::{FullLogLine, LogLine};
use crate::ui::table::ScrollbarTableState;
use crate::ui::UI_HEADER_SIZE;
use crate::{copy_osc, parse_line_full};
use derive_more::From;
use ratatui::widgets::TableState;

#[derive(Clone, From, PartialEq)]
pub enum UiState<'a> {
    MatchList(MatchListState<'a>),
    Match(MatchState<'a>),
    Logs(LogsState<'a>),
    Log(LogState<'a>),
    Errors(ErrorState<'a>),
    Quit,
}

#[derive(Clone)]
pub struct MatchListState<'a> {
    app: &'a App<'a>,
    pub table_state: ScrollbarTableState,
}

impl<'a> MatchListState<'a> {
    fn selected(&self) -> usize {
        self.table_state.selected()
    }

    fn enter(self, selected: usize, app: &'a App) -> UiState<'a> {
        let result = if selected == 0 {
            &app.all
        } else if selected == app.match_lines() - 1 {
            &app.unmatched
        } else {
            &app.matches[selected - 1]
        };
        let table_state = ScrollbarTableState::new(result.grouped.len());
        UiState::Match(MatchState {
            result,
            table_state,
            previous: Box::new(self.into()),
        })
    }
}

impl PartialEq for MatchListState<'_> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct MatchState<'a> {
    pub result: &'a LogMatch,
    pub table_state: ScrollbarTableState,
    pub previous: Box<UiState<'a>>,
}

impl<'a> MatchState<'a> {
    fn selected(&self) -> usize {
        self.table_state.selected()
    }

    fn enter(self, selected: usize) -> UiState<'a> {
        let mut table_state = TableState::default();
        table_state.select(Some(0));

        let lines = self.result.grouped[selected].lines.as_slice();
        let table_state = ScrollbarTableState::new(lines.len());
        UiState::Logs(LogsState {
            lines,
            table_state,
            previous: Box::new(self.into()),
        })
    }
}

impl PartialEq for MatchState<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.result.result == other.result.result
    }
}

#[derive(Clone)]
pub struct LogsState<'a> {
    pub lines: &'a [usize],
    pub table_state: ScrollbarTableState,
    pub previous: Box<UiState<'a>>,
}

impl<'a> LogsState<'a> {
    fn selected(&self) -> usize {
        self.table_state.selected()
    }

    fn enter(self, selected: usize, app: &'a App<'a>) -> UiState<'a> {
        let line = self.lines[selected];
        let log = &app.lines[line];
        let raw_line = app.get_line(log.index).unwrap();
        let full_line = parse_line_full(raw_line).unwrap();
        let trace_len = if let Some(exception) = &full_line.exception {
            exception.stack().map(|e| 1 + e.trace.len()).sum()
        } else {
            0
        };

        let table_state = ScrollbarTableState::new(trace_len);
        UiState::Log(LogState {
            log,
            full_line,
            trace_len,
            table_state,
            previous: Box::new(self.into()),
        })
    }
}

impl PartialEq for LogsState<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.lines == other.lines
    }
}

#[derive(Clone)]
pub struct ErrorState<'a> {
    pub table_state: ScrollbarTableState,
    pub previous: Box<UiState<'a>>,
}

impl PartialEq for ErrorState<'_> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct LogState<'a> {
    pub trace_len: usize,
    pub log: &'a LogLine<'a>,
    pub full_line: FullLogLine,
    pub table_state: ScrollbarTableState,
    pub previous: Box<UiState<'a>>,
}

impl PartialEq for LogState<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.log.index == other.log.index
    }
}

impl<'a> UiState<'a> {
    pub fn new(app: &'a App<'a>) -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        UiState::MatchList(MatchListState {
            app,
            table_state: ScrollbarTableState::new(app.match_lines()),
        })
    }

    pub fn page(&self) -> UiPage {
        match self {
            UiState::Quit | UiState::MatchList(_) => UiPage::MatchList,
            UiState::Match(_) => UiPage::Match,
            UiState::Logs(_) => UiPage::Logs,
            UiState::Log(_) => UiPage::Log,
            UiState::Errors(_) => UiPage::Errors,
        }
    }

    fn table_state(&self) -> Option<&ScrollbarTableState> {
        match self {
            UiState::MatchList(state) => Some(&state.table_state),
            UiState::Match(state) => Some(&state.table_state),
            UiState::Logs(state) => Some(&state.table_state),
            UiState::Log(state) => Some(&state.table_state),
            UiState::Errors(state) => Some(&state.table_state),
            _ => None,
        }
    }

    fn table_state_mut(&mut self) -> Option<&mut ScrollbarTableState> {
        match self {
            UiState::MatchList(state) => Some(&mut state.table_state),
            UiState::Match(state) => Some(&mut state.table_state),
            UiState::Logs(state) => Some(&mut state.table_state),
            UiState::Log(state) => Some(&mut state.table_state),
            UiState::Errors(state) => Some(&mut state.table_state),
            _ => None,
        }
    }

    pub fn scroll_offset(&self) -> usize {
        if let Some(table_state) = self.table_state() {
            table_state.offset()
        } else {
            0
        }
    }

    pub fn row_count(&self) -> usize {
        if let Some(table_state) = self.table_state() {
            table_state.row_count()
        } else {
            0
        }
    }

    pub fn index_for_row(&self, row: usize) -> usize {
        match self {
            UiState::MatchList(MatchListState { app, .. }) => {
                let mut total_height = 0;
                let match_row_counts = app.matches.iter().map(|m| m.row_count());
                for (index, row_count) in once(1).chain(match_row_counts).chain(once(1)).enumerate().skip(self.scroll_offset())
                {
                    if total_height > row {
                        return index - 1;
                    }
                    total_height += row_count;
                }
                if total_height > row {
                    app.matches.len() + 1
                } else {
                    app.matches.len() + 2
                }
            }
            _ => row + self.scroll_offset(),
        }
    }

    /// get the offset of the "main content" from the top of the screen
    pub fn content_offset(&self) -> u16 {
        match self {
            UiState::MatchList(_) => UI_HEADER_SIZE + 1,
            UiState::Match(_) => UI_HEADER_SIZE + 1,
            UiState::Logs(_) => 0,
            UiState::Log(_) => 0,
            UiState::Errors(_) => 0,
            UiState::Quit => 0,
        }
    }

    pub fn process(self, event: UiEvent, app: &'a App<'a>) -> (bool, UiState) {
        match (self, event) {
            (UiState::Quit, _) => (true, UiState::Quit),
            (_, UiEvent::Quit) => (true, UiState::Quit),
            (UiState::MatchList(_), UiEvent::Back) => (true, UiState::Quit),
            (mut state, UiEvent::Down(step, rollover)) => {
                if let Some(table_state) = state.table_state_mut() {
                    table_state.down(step, rollover);
                }
                (true, state)
            }
            (mut state, UiEvent::Up(step, rollover)) => {
                if let Some(table_state) = state.table_state_mut() {
                    table_state.up(step, rollover);
                }
                (true, state)
            }
            (mut state, UiEvent::SelectAt(selected)) => {
                if let Some(table_state) = state.table_state_mut() {
                    table_state.select(selected);
                }
                (true, state)
            }
            (UiState::MatchList(state), UiEvent::Select) => {
                let selected = state.selected();
                (true, state.enter(selected, app))
            }
            (UiState::MatchList(state), UiEvent::Enter(selected)) => {
                (true, state.enter(selected, app))
            }
            (UiState::MatchList(state), UiEvent::Errors) => {
                let table_state = ScrollbarTableState::new(app.error_lines.len());
                (
                    true,
                    UiState::Errors(ErrorState {
                        table_state,
                        previous: Box::new(state.into()),
                    }),
                )
            }
            (UiState::Match(state), UiEvent::Select) => {
                let selected = state.selected();
                (true, state.enter(selected))
            }
            (UiState::Match(state), UiEvent::Enter(selected)) => (true, state.enter(selected)),
            (UiState::Logs(state), UiEvent::Select) => {
                let selected = state.selected();
                (true, state.enter(selected, app))
            }
            (UiState::Logs(state), UiEvent::Enter(selected)) => (true, state.enter(selected, app)),
            (UiState::Logs(state), UiEvent::Copy) => {
                let selected = state.selected();
                let mut table_state = TableState::default();
                table_state.select(Some(0));

                let line = &app.lines[state.lines[selected]];
                let raw = app.get_line(line.index).unwrap_or_default();
                copy_osc(raw);
                (false, UiState::Logs(state))
            }
            (UiState::Log(state), UiEvent::Copy) => {
                let raw = app.get_line(state.log.index).unwrap_or_default();
                copy_osc(raw);
                (false, UiState::Log(state))
            }
            (UiState::Errors(state), UiEvent::Copy) => {
                let raw = app
                    .error_lines
                    .get(state.table_state.selected())
                    .map(|(line, _)| line.as_str())
                    .unwrap_or_default();
                copy_osc(raw);
                (false, UiState::Errors(state))
            }
            (
                UiState::Match(MatchState { previous, .. })
                | UiState::Logs(LogsState { previous, .. })
                | UiState::Log(LogState { previous, .. })
                | UiState::Errors(ErrorState { previous, .. }),
                UiEvent::Back,
            ) => (true, *previous),
            (state, _) => (false, state),
        }
    }
}

pub enum UiEvent {
    Quit,
    Back,
    Up(usize, bool),
    Down(usize, bool),
    Errors,
    Select,
    #[allow(dead_code)]
    SelectAt(usize),
    Enter(usize),
    Copy,
}

#[derive(PartialEq)]
pub enum UiPage {
    MatchList,
    Match,
    Logs,
    Log,
    Errors,
}
