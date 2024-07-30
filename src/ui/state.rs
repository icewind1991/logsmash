use crate::app::{App, LogMatch};
use crate::logline::{FullLogLine, LogLine};
use crate::ui::table::ScrollbarTableState;
use crate::{copy_osc, parse_line_full};
use derive_more::From;
use ratatui::widgets::TableState;

#[derive(Clone, From)]
pub enum UiState<'a> {
    MatchList(MatchListState),
    Match(MatchState<'a>),
    Logs(LogsState<'a>),
    Log(LogState<'a>),
    Quit,
}

#[derive(Clone)]
pub struct MatchListState {
    pub table_state: ScrollbarTableState,
}

impl MatchListState {
    fn selected(&self) -> usize {
        self.table_state.selected()
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
}

#[derive(Clone)]
pub struct LogState<'a> {
    pub trace_len: usize,
    pub log: &'a LogLine,
    pub full_line: FullLogLine,
    pub table_state: ScrollbarTableState,
    pub previous: Box<UiState<'a>>,
}

impl<'a> UiState<'a> {
    pub fn new(app: &App) -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        UiState::MatchList(MatchListState {
            table_state: ScrollbarTableState::new(app.match_lines()),
        })
    }

    pub fn page(&self) -> UiPage {
        match self {
            UiState::Quit | UiState::MatchList(_) => UiPage::MatchList,
            UiState::Match(_) => UiPage::Match,
            UiState::Logs(_) => UiPage::Logs,
            UiState::Log(_) => UiPage::Log,
        }
    }

    fn table_state(&mut self) -> Option<&mut ScrollbarTableState> {
        match self {
            UiState::MatchList(state) => Some(&mut state.table_state),
            UiState::Match(state) => Some(&mut state.table_state),
            UiState::Logs(state) => Some(&mut state.table_state),
            UiState::Log(state) => Some(&mut state.table_state),
            _ => None,
        }
    }

    pub fn process(self, event: UiEvent, app: &'a App) -> (bool, UiState) {
        match (self, event) {
            (UiState::Quit, _) => (true, UiState::Quit),
            (_, UiEvent::Quit) => (true, UiState::Quit),
            (UiState::MatchList(_), UiEvent::Back) => (true, UiState::Quit),
            (mut state, UiEvent::Down(step)) => {
                if let Some(table_state) = state.table_state() {
                    table_state.down(step);
                }
                (true, state)
            }
            (mut state, UiEvent::Up(step)) => {
                if let Some(table_state) = state.table_state() {
                    table_state.up(step);
                }
                (true, state)
            }
            (UiState::MatchList(state), UiEvent::Select) => {
                let selected = state.selected();

                let result = if selected == 0 {
                    &app.all
                } else if selected == app.match_lines() - 1 {
                    &app.unmatched
                } else {
                    &app.matches[selected - 1]
                };
                let table_state = ScrollbarTableState::new(result.grouped.len());
                (
                    true,
                    UiState::Match(MatchState {
                        result,
                        table_state,
                        previous: Box::new(state.into()),
                    }),
                )
            }
            (UiState::Match(state), UiEvent::Select) => {
                let selected = state.selected();
                let mut table_state = TableState::default();
                table_state.select(Some(0));

                let lines = state.result.grouped[selected].lines.as_slice();
                let table_state = ScrollbarTableState::new(lines.len());
                (
                    true,
                    UiState::Logs(LogsState {
                        lines,
                        table_state,
                        previous: Box::new(state.into()),
                    }),
                )
            }
            (UiState::Logs(state), UiEvent::Select) => {
                let selected = state.selected();

                let line = state.lines[selected];
                let log = &app.lines[line];
                let raw_line = app.get_line(log.index).unwrap();
                let full_line = parse_line_full(raw_line).unwrap();
                let trace_len = if let Some(exception) = &full_line.exception {
                    exception.stack().map(|e| 1 + e.trace.len()).sum()
                } else {
                    0
                };

                let table_state = ScrollbarTableState::new(trace_len);
                (
                    true,
                    UiState::Log(LogState {
                        log,
                        full_line,
                        trace_len,
                        table_state,
                        previous: Box::new(state.into()),
                    }),
                )
            }
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
            (
                UiState::Match(MatchState { previous, .. })
                | UiState::Logs(LogsState { previous, .. })
                | UiState::Log(LogState { previous, .. }),
                UiEvent::Back,
            ) => (true, *previous),
            (state, _) => (false, state),
        }
    }
}

pub enum UiEvent {
    Quit,
    Back,
    Up(usize),
    Down(usize),
    Select,
    Copy,
}

#[derive(PartialEq)]
pub enum UiPage {
    MatchList,
    Match,
    Logs,
    Log,
}
