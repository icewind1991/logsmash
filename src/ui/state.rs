use crate::app::{App, LogMatch};
use crate::copy_osc;
use crate::logline::{FullLogLine, LogLine};
use derive_more::From;
use ratatui::widgets::{ScrollbarState, TableState};
use table_state::TableStateExt;

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
    pub table_state: TableState,
    pub scroll_state: ScrollbarState,
}

impl MatchListState {
    fn selected(&self) -> usize {
        self.table_state.selected().unwrap()
    }
}

#[derive(Clone)]
pub struct MatchState<'a> {
    pub result: &'a LogMatch,
    pub table_state: TableState,
    pub scroll_state: ScrollbarState,
    pub previous: Box<UiState<'a>>,
}

impl<'a> MatchState<'a> {
    fn selected(&self) -> usize {
        self.table_state.selected().unwrap()
    }
}

#[derive(Clone)]
pub struct LogsState<'a> {
    pub lines: &'a [usize],
    pub table_state: TableState,
    pub scroll_state: ScrollbarState,
    pub previous: Box<UiState<'a>>,
}

impl<'a> LogsState<'a> {
    fn selected(&self) -> usize {
        self.table_state.selected().unwrap()
    }
}

#[derive(Clone)]
pub struct LogState<'a> {
    pub trace_len: usize,
    pub log: &'a LogLine,
    pub full_line: FullLogLine,
    pub table_state: TableState,
    pub previous: Box<UiState<'a>>,
}

impl<'a> UiState<'a> {
    pub fn new(app: &App) -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        UiState::MatchList(MatchListState {
            table_state,
            scroll_state: ScrollbarState::new(app.match_lines()),
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

    fn table_state(&mut self) -> Option<&mut TableState> {
        match self {
            UiState::MatchList(state) => Some(&mut state.table_state),
            UiState::Match(state) => Some(&mut state.table_state),
            UiState::Logs(state) => Some(&mut state.table_state),
            UiState::Log(state) => Some(&mut state.table_state),
            _ => None,
        }
    }

    fn scroll_state(&mut self) -> Option<&mut ScrollbarState> {
        match self {
            UiState::MatchList(state) => Some(&mut state.scroll_state),
            UiState::Match(state) => Some(&mut state.scroll_state),
            UiState::Logs(state) => Some(&mut state.scroll_state),
            _ => None,
        }
    }

    fn row_count(&self, app: &App) -> usize {
        match self {
            UiState::MatchList(_) => app.match_lines(),
            UiState::Match(state) => state.result.grouped.len(),
            UiState::Logs(state) => state.lines.len(),
            UiState::Log(state) => state.trace_len,
            _ => 0,
        }
    }

    pub fn process(self, event: UiEvent, app: &'a App) -> UiState {
        match (self, event) {
            (UiState::Quit, _) => UiState::Quit,
            (_, UiEvent::Quit) => UiState::Quit,
            (UiState::MatchList(_), UiEvent::Back) => UiState::Quit,
            (mut state, UiEvent::Down(step)) => {
                let count = state.row_count(app);
                if let Some(table_state) = state.table_state() {
                    let pos = table_state.down(count, step);
                    if let Some(scroll_state) = state.scroll_state() {
                        *scroll_state = scroll_state.position(pos);
                    }
                }
                state
            }
            (mut state, UiEvent::Up(step)) => {
                let count = state.row_count(app);
                if let Some(table_state) = state.table_state() {
                    let pos = table_state.up(count, step);
                    if let Some(scroll_state) = state.scroll_state() {
                        *scroll_state = scroll_state.position(pos);
                    }
                }
                state
            }
            (UiState::MatchList(state), UiEvent::Select) => {
                let selected = state.selected();
                let mut table_state = TableState::default();
                table_state.select(Some(0));

                let result = if selected == 0 {
                    &app.all
                } else if selected == app.match_lines() - 1 {
                    &app.unmatched
                } else {
                    &app.matches[selected - 1]
                };
                UiState::Match(MatchState {
                    result,
                    table_state,
                    scroll_state: ScrollbarState::new(result.count()),
                    previous: Box::new(state.into()),
                })
            }
            (UiState::Match(state), UiEvent::Select) => {
                let selected = state.selected();
                let mut table_state = TableState::default();
                table_state.select(Some(0));

                let lines = state.result.grouped[selected].lines.as_slice();
                UiState::Logs(LogsState {
                    lines,
                    table_state,
                    scroll_state: ScrollbarState::new(lines.len()),
                    previous: Box::new(state.into()),
                })
            }
            (UiState::Logs(state), UiEvent::Select) => {
                let selected = state.selected();
                let mut table_state = TableState::default();
                table_state.select(Some(0));

                let line = state.lines[selected];
                let log = &app.lines[line];
                let raw_line = app.get_line(log.index).unwrap();
                let full_line: FullLogLine = serde_json::from_str(raw_line).unwrap();
                let trace_len = if let Some(exception) = &full_line.exception {
                    exception.stack().map(|e| 1 + e.trace.len()).sum()
                } else {
                    0
                };
                UiState::Log(LogState {
                    log,
                    full_line,
                    trace_len,
                    table_state,
                    previous: Box::new(state.into()),
                })
            }
            (UiState::Logs(state), UiEvent::Copy) => {
                let selected = state.selected();
                let mut table_state = TableState::default();
                table_state.select(Some(0));

                let line = &app.lines[state.lines[selected]];
                let raw = app.get_line(line.index).unwrap_or_default();
                copy_osc(raw);
                UiState::Logs(state)
            }
            (UiState::Log(state), UiEvent::Copy) => {
                let raw = app.get_line(state.log.index).unwrap_or_default();
                copy_osc(raw);
                UiState::Log(state)
            }
            (
                UiState::Match(MatchState { previous, .. })
                | UiState::Logs(LogsState { previous, .. })
                | UiState::Log(LogState { previous, .. }),
                UiEvent::Back,
            ) => *previous,
            (state, _) => state,
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

mod table_state {
    use ratatui::widgets::TableState;

    pub trait TableStateExt {
        fn up(&mut self, count: usize, step: usize) -> usize;
        fn down(&mut self, count: usize, step: usize) -> usize;
    }

    impl TableStateExt for TableState {
        fn up(&mut self, count: usize, step: usize) -> usize {
            let current = self.selected().unwrap_or(0);
            let after = if step > current {
                if step == 1 {
                    count - 1
                } else {
                    0
                }
            } else {
                current - step
            };
            self.select(Some(after));
            after
        }

        fn down(&mut self, count: usize, step: usize) -> usize {
            let current = self.selected().unwrap_or(0);
            let after = if step >= count - current {
                if step == 1 {
                    0
                } else {
                    count - 1
                }
            } else {
                current + step
            };
            self.select(Some(after));
            after
        }
    }
}
