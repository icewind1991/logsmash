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
        }
    }

    pub fn process(self, event: UiEvent, app: &App) -> UiState {
        match (self, event) {
            (UiState::Quit, _) => UiState::Quit,
            (_, UiEvent::Quit) => UiState::Quit,
            (UiState::MatchList { .. }, UiEvent::Back) => UiState::Quit,
            (UiState::MatchList { mut table_state }, UiEvent::Down) => {
                table_state.down(app.matches.len());
                UiState::MatchList { table_state }
            }
            (UiState::MatchList { mut table_state }, UiEvent::Up) => {
                table_state.up(app.matches.len());
                UiState::MatchList { table_state }
            }
            (UiState::MatchList { table_state }, UiEvent::Select) => {
                let selected = table_state.selected().unwrap_or(0);
                let mut table_state = TableState::default();
                table_state.select(Some(0));
                UiState::Match {
                    selected,
                    table_state,
                }
            }
            (
                UiState::Match {
                    selected: index, ..
                },
                UiEvent::Back,
            ) => {
                let mut table_state = TableState::default();
                table_state.select(Some(index));
                UiState::MatchList { table_state }
            }
            (
                UiState::Match {
                    mut table_state,
                    selected,
                },
                UiEvent::Down,
            ) => {
                table_state.down(app.matches[selected].count());
                UiState::Match {
                    table_state,
                    selected,
                }
            }
            (
                UiState::Match {
                    mut table_state,
                    selected,
                },
                UiEvent::Up,
            ) => {
                table_state.up(app.matches[selected].count());
                UiState::Match {
                    table_state,
                    selected,
                }
            }
            (state @ UiState::Match { .. }, _) => state,
        }
    }
}

pub enum UiEvent {
    Quit,
    Back,
    Up,
    Down,
    Select,
}

pub enum UiPage {
    MatchList,
    Match,
}

mod table_state {
    use ratatui::widgets::TableState;

    pub trait TableStateExt {
        fn up(&mut self, count: usize);
        fn down(&mut self, count: usize);
    }

    impl TableStateExt for TableState {
        fn up(&mut self, count: usize) {
            let current = self.selected().unwrap_or(0);
            self.select(Some(if current == 0 { count - 1 } else { current - 1 }))
        }

        fn down(&mut self, count: usize) {
            let current = self.selected().unwrap_or(0);
            self.select(Some((current + 1).rem_euclid(count)))
        }
    }
}
