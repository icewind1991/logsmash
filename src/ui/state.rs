use crate::app::App;
use ratatui::widgets::TableState;
use table_state::TableStateExt;

#[derive(Clone, Debug)]
pub enum UiState {
    MatchList {
        table_state: TableState,
    },
    Match {
        selected: usize,
        table_state: TableState,
    },
    All {
        table_state: TableState,
    },
    Unmatched {
        table_state: TableState,
    },
    Quit,
}

impl Default for UiState {
    fn default() -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        UiState::MatchList { table_state }
    }
}

impl UiState {
    pub fn page(&self) -> UiPage {
        match self {
            UiState::Quit | UiState::MatchList { .. } => UiPage::MatchList,
            UiState::Match { .. } => UiPage::Match,
            UiState::All { .. } => UiPage::All,
            UiState::Unmatched { .. } => UiPage::Unmatched,
        }
    }

    fn table_state(&mut self) -> Option<&mut TableState> {
        match self {
            UiState::MatchList { table_state } => Some(table_state),
            UiState::Match { table_state, .. } => Some(table_state),
            UiState::All { table_state } => Some(table_state),
            UiState::Unmatched { table_state } => Some(table_state),
            UiState::Quit => None,
        }
    }

    fn table_count(&self, app: &App) -> usize {
        match self {
            UiState::MatchList { .. } => app.match_lines(),
            UiState::Match { selected, .. } => app.matches[*selected].grouped.len(),
            UiState::All { .. } => app.all.grouped.len(),
            UiState::Unmatched { .. } => app.unmatched.grouped.len(),
            UiState::Quit => 0,
        }
    }

    pub fn process(self, event: UiEvent, app: &App) -> UiState {
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
            (UiState::MatchList { table_state }, UiEvent::Select) => {
                let selected = table_state.selected().unwrap_or(0);
                let mut table_state = TableState::default();
                table_state.select(Some(0));
                if selected == 0 {
                    UiState::All { table_state }
                } else if selected == app.match_lines() - 1 {
                    UiState::Unmatched { table_state }
                } else {
                    UiState::Match {
                        selected: selected - 1,
                        table_state,
                    }
                }
            }
            (
                UiState::Match {
                    selected: index, ..
                },
                UiEvent::Back,
            ) => {
                let mut table_state = TableState::default();
                table_state.select(Some(index + 1));
                UiState::MatchList { table_state }
            }
            (UiState::All { .. }, UiEvent::Back) => {
                let mut table_state = TableState::default();
                table_state.select(Some(0));
                UiState::MatchList { table_state }
            }
            (UiState::Unmatched { .. }, UiEvent::Back) => {
                let mut table_state = TableState::default();
                table_state.select(Some(app.match_lines() - 1));
                UiState::MatchList { table_state }
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
    All,
    Unmatched,
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
