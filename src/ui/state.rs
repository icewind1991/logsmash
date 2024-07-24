use crate::app::App;
use ratatui::widgets::TableState;
use table_state::TableStateExt;

#[derive(Clone, Debug)]
pub enum UiState {
    MatchList { table_state: TableState },
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
        }
    }
}

pub enum UiEvent {
    Quit,
    Back,
    Up,
    Down,
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
