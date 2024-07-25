use crate::app::{App, LogMatch};
use ratatui::widgets::TableState;
use table_state::TableStateExt;

#[derive(Clone)]
pub enum UiState<'a> {
    MatchList {
        table_state: TableState,
    },
    Match {
        result: &'a LogMatch,
        table_state: TableState,
        previous: Box<UiState<'a>>,
    },
    Quit,
}

impl Default for UiState<'_> {
    fn default() -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        UiState::MatchList { table_state }
    }
}

impl<'a> UiState<'a> {
    pub fn page(&self) -> UiPage {
        match self {
            UiState::Quit | UiState::MatchList { .. } => UiPage::MatchList,
            UiState::Match { .. } => UiPage::Match,
        }
    }

    fn table_state(&mut self) -> Option<&mut TableState> {
        match self {
            UiState::MatchList { table_state } => Some(table_state),
            UiState::Match { table_state, .. } => Some(table_state),
            UiState::Quit => None,
        }
    }

    fn table_count(&self, app: &App) -> usize {
        match self {
            UiState::MatchList { .. } => app.match_lines(),
            UiState::Match { result, .. } => result.grouped.len(),
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
                    table_state.down(count, step)
                }
                state
            }
            (mut state, UiEvent::Up(step)) => {
                let count = state.table_count(app);
                if let Some(table_state) = state.table_state() {
                    table_state.up(count, step)
                }
                state
            }
            (mut prev @ UiState::MatchList { .. }, UiEvent::Select) => {
                let selected = prev.table_state().unwrap().selected().unwrap_or(0);
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
                    previous: Box::new(prev),
                }
            }
            (UiState::Match { previous, .. }, UiEvent::Back) => *previous,
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
}

mod table_state {
    use ratatui::widgets::TableState;

    pub trait TableStateExt {
        fn up(&mut self, count: usize, step: usize);
        fn down(&mut self, count: usize, step: usize);
    }

    impl TableStateExt for TableState {
        fn up(&mut self, count: usize, step: usize) {
            let current = self.selected().unwrap_or(0);
            let after = if step > current {
                count - 1
            } else {
                current - step
            };
            self.select(Some(after))
        }

        fn down(&mut self, count: usize, step: usize) {
            let current = self.selected().unwrap_or(0);
            let after = if step >= count - current {
                0
            } else {
                current + step
            };
            self.select(Some(after))
        }
    }
}
