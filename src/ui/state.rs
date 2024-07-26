use crate::app::{App, LogMatch};
use ratatui::widgets::{ScrollbarState, TableState};
use table_state::TableStateExt;

#[derive(Clone)]
pub enum UiState<'a> {
    MatchList {
        table_state: TableState,
        scroll_state: ScrollbarState,
    },
    Match {
        result: &'a LogMatch,
        table_state: TableState,
        scroll_state: ScrollbarState,
        previous: Box<UiState<'a>>,
    },
    Logs {
        lines: &'a [usize],
        table_state: TableState,
        scroll_state: ScrollbarState,
        previous: Box<UiState<'a>>,
    },
    Quit,
}

impl<'a> UiState<'a> {
    pub fn new(app: &App) -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        UiState::MatchList {
            table_state,
            scroll_state: ScrollbarState::new(app.match_lines()),
        }
    }

    pub fn page(&self) -> UiPage {
        match self {
            UiState::Quit | UiState::MatchList { .. } => UiPage::MatchList,
            UiState::Match { .. } => UiPage::Match,
            UiState::Logs { .. } => UiPage::Logs,
        }
    }

    fn table_state(&mut self) -> Option<&mut TableState> {
        match self {
            UiState::MatchList { table_state, .. } => Some(table_state),
            UiState::Match { table_state, .. } => Some(table_state),
            UiState::Logs { table_state, .. } => Some(table_state),
            UiState::Quit => None,
        }
    }

    fn scroll_state(&mut self) -> Option<&mut ScrollbarState> {
        match self {
            UiState::MatchList { scroll_state, .. } => Some(scroll_state),
            UiState::Match { scroll_state, .. } => Some(scroll_state),
            UiState::Logs { scroll_state, .. } => Some(scroll_state),
            UiState::Quit => None,
        }
    }

    fn table_count(&self, app: &App) -> usize {
        match self {
            UiState::MatchList { .. } => app.match_lines(),
            UiState::Match { result, .. } => result.grouped.len(),
            UiState::Logs { lines, .. } => lines.len(),
            UiState::Quit => 0,
        }
    }

    pub fn process(self, event: UiEvent, app: &'a App) -> UiState {
        match (self, event) {
            (UiState::Quit, _) => UiState::Quit,
            (_, UiEvent::Quit) => UiState::Quit,
            (UiState::MatchList { .. }, UiEvent::Back) => UiState::Quit,
            (mut state, UiEvent::Down(step)) => {
                let count = state.table_count(app);
                if let Some(table_state) = state.table_state() {
                    let pos = table_state.down(count, step);
                    let scroll_state = state.scroll_state().unwrap();
                    *scroll_state = scroll_state.position(pos);
                }
                state
            }
            (mut state, UiEvent::Up(step)) => {
                let count = state.table_count(app);
                if let Some(table_state) = state.table_state() {
                    let pos = table_state.up(count, step);
                    let scroll_state = state.scroll_state().unwrap();
                    *scroll_state = scroll_state.position(pos);
                }
                state
            }
            (
                UiState::MatchList {
                    table_state: prev_state,
                    scroll_state: prev_scroll,
                },
                UiEvent::Select,
            ) => {
                let selected = prev_state.selected().unwrap_or(0);
                let mut table_state = TableState::default();
                table_state.select(Some(0));

                let result = if selected == 0 {
                    &app.all
                } else if selected == app.match_lines() - 1 {
                    &app.unmatched
                } else {
                    &app.matches[selected - 1]
                };
                UiState::Match {
                    result,
                    table_state,
                    scroll_state: ScrollbarState::new(result.count()),
                    previous: Box::new(UiState::MatchList {
                        table_state: prev_state,
                        scroll_state: prev_scroll,
                    }),
                }
            }
            (
                UiState::Match {
                    table_state: prev_state,
                    scroll_state: prev_scroll,
                    previous,
                    result,
                },
                UiEvent::Select,
            ) => {
                let selected = prev_state.selected().unwrap_or(0);
                let mut table_state = TableState::default();
                table_state.select(Some(0));

                let lines = result.grouped[selected].lines.as_slice();
                UiState::Logs {
                    lines,
                    table_state,
                    scroll_state: ScrollbarState::new(lines.len()),
                    previous: Box::new(UiState::Match {
                        table_state: prev_state,
                        scroll_state: prev_scroll,
                        previous,
                        result,
                    }),
                }
            }
            (UiState::Match { previous, .. } | UiState::Logs { previous, .. }, UiEvent::Back) => {
                *previous
            }
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
}

pub enum UiPage {
    MatchList,
    Match,
    Logs,
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
