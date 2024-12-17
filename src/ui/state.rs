use crate::app::{App, Filter, LogMatch, EMPTY_FILTER};
use crate::logline::{FullLogLine, LogLine};
use crate::ui::footer::FooterParams;
use crate::ui::table::ScrollbarTableState;
use crate::ui::UI_HEADER_SIZE;
use crate::{copy_osc, parse_line_full};
use derive_more::From;
use ratatui::widgets::TableState;
use std::iter::once;

#[derive(Clone, From, PartialEq)]
pub enum UiState<'a> {
    MatchList(MatchListState<'a>),
    Match(MatchState<'a>),
    GroupedLogs(GroupedLogsState<'a>),
    Log(LogState<'a>),
    Errors(ErrorState<'a>),
    Quit,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    Normal,
    FilterInput,
}

#[derive(Clone)]
pub struct MatchListState<'a> {
    app: &'a App<'a>,
    pub table_state: ScrollbarTableState,
    pub filter: Filter,
    mode: Mode,
}

impl<'a> MatchListState<'a> {
    fn selected(&self) -> usize {
        self.table_state.selected()
    }

    fn enter(self, selected: usize, app: &'a App) -> UiState<'a> {
        let result = if selected == 0 {
            &app.all
        } else if selected <= app.matches.len() {
            if self.filter.is_empty() {
                &app.matches[selected - 1]
            } else {
                app.matches
                    .iter()
                    .filter(|log_match| log_match.matches(app, &self.filter))
                    .nth(selected - 1)
                    .expect("filtered select out of bounds")
            }
        } else {
            &app.unmatched
        };
        let table_state = ScrollbarTableState::new(result.grouped.len() + 1);
        UiState::Match(MatchState {
            result,
            table_state,
            previous: Box::new(self.into()),
            filter: Filter::default(),
            mode: Mode::Normal,
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
    pub filter: Filter,
    mode: Mode,
}

impl<'a> MatchState<'a> {
    fn selected(&self) -> usize {
        self.table_state.selected()
    }

    fn enter(self, selected: usize, app: &'a App) -> UiState<'a> {
        let mut table_state = TableState::default();
        table_state.select(Some(0));

        let selected_line = if selected == 0 {
            &self.result.all
        } else if self.filter.is_empty() {
            &self.result.grouped[selected - 1]
        } else {
            self.result
                .grouped
                .iter()
                .filter(|grouped| grouped.matches(app, &self.filter))
                .nth(selected - 1)
                .expect("filtered select out of bounds")
        };
        let lines = selected_line.lines.as_slice();
        let table_state = ScrollbarTableState::new(lines.len());
        UiState::GroupedLogs(GroupedLogsState {
            lines,
            table_state,
            previous: Box::new(self.into()),
            filter: Filter::default(),
            mode: Mode::Normal,
        })
    }
}

impl PartialEq for MatchState<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.result.result == other.result.result
    }
}

#[derive(Clone)]
pub struct GroupedLogsState<'a> {
    pub lines: &'a [usize],
    pub table_state: ScrollbarTableState,
    pub previous: Box<UiState<'a>>,
    pub filter: Filter,
    mode: Mode,
}

impl<'a> GroupedLogsState<'a> {
    fn selected(&self) -> usize {
        self.table_state.selected()
    }

    fn enter(self, selected: usize, app: &'a App<'a>) -> UiState<'a> {
        let log = if self.filter.is_empty() {
            let line = self.lines[selected];
            &app.lines[line]
        } else {
            self.lines
                .iter()
                .map(|index| &app.lines[*index])
                .filter(|line| line.matches(&self.filter))
                .nth(selected)
                .expect("filtered select out of bounds")
        };
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
            table_state,
            previous: Box::new(self.into()),
        })
    }
}

impl PartialEq for GroupedLogsState<'_> {
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
            filter: Filter::default(),
            mode: Mode::Normal,
        })
    }

    pub fn page(&self) -> UiPage {
        match self {
            UiState::Quit | UiState::MatchList(_) => UiPage::MatchList,
            UiState::Match(_) => UiPage::Match,
            UiState::GroupedLogs(_) => UiPage::Logs,
            UiState::Log(_) => UiPage::Log,
            UiState::Errors(_) => UiPage::Errors,
        }
    }

    pub fn mode(&self) -> Mode {
        match self {
            UiState::MatchList(state) => state.mode,
            UiState::Match(state) => state.mode,
            UiState::GroupedLogs(state) => state.mode,
            _ => Mode::Normal,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        match self {
            UiState::MatchList(state) => state.mode = mode,
            UiState::Match(state) => state.mode = mode,
            UiState::GroupedLogs(state) => state.mode = mode,
            _ => {}
        }
    }

    pub fn filter(&self) -> Option<&Filter> {
        match self {
            UiState::MatchList(state) => Some(&state.filter),
            UiState::Match(state) => Some(&state.filter),
            UiState::GroupedLogs(state) => Some(&state.filter),
            _ => None,
        }
    }

    pub fn filter_mut(&mut self) -> Option<&mut Filter> {
        match self {
            UiState::MatchList(state) => Some(&mut state.filter),
            UiState::Match(state) => Some(&mut state.filter),
            UiState::GroupedLogs(state) => Some(&mut state.filter),
            _ => None,
        }
    }

    fn table_state(&self) -> Option<&ScrollbarTableState> {
        match self {
            UiState::MatchList(state) => Some(&state.table_state),
            UiState::Match(state) => Some(&state.table_state),
            UiState::GroupedLogs(state) => Some(&state.table_state),
            UiState::Log(state) => Some(&state.table_state),
            UiState::Errors(state) => Some(&state.table_state),
            _ => None,
        }
    }

    fn table_state_mut(&mut self) -> Option<&mut ScrollbarTableState> {
        match self {
            UiState::MatchList(state) => Some(&mut state.table_state),
            UiState::Match(state) => Some(&mut state.table_state),
            UiState::GroupedLogs(state) => Some(&mut state.table_state),
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
            UiState::MatchList(MatchListState { app, filter, .. }) => {
                let mut total_height = 0;
                let match_row_counts = app
                    .matches
                    .iter()
                    .filter(|m| m.matches(app, filter))
                    .map(|m| m.row_count());
                for (index, row_count) in once(1)
                    .chain(match_row_counts)
                    .chain(once(1))
                    .enumerate()
                    .skip(self.scroll_offset())
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
            UiState::GroupedLogs(_) => UI_HEADER_SIZE + 1,
            UiState::Log(_) => 0,
            UiState::Errors(_) => 0,
            UiState::Quit => 0,
        }
    }

    pub fn process(self, event: UiEvent, app: &'a App<'a>) -> (bool, UiState<'a>) {
        match (self, event) {
            (UiState::Quit, _) => (true, UiState::Quit),
            (_, UiEvent::Quit) => (true, UiState::Quit),
            (
                UiState::MatchList(MatchListState {
                    mode: Mode::Normal, ..
                }),
                UiEvent::Back,
            ) => (true, UiState::Quit),
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
            (mut state, UiEvent::Scroll(step)) => {
                if let Some(table_state) = state.table_state_mut() {
                    table_state.scroll(step);
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
                (true, state.enter(selected, app))
            }
            (UiState::Match(state), UiEvent::Enter(selected)) => (true, state.enter(selected, app)),
            (UiState::GroupedLogs(state), UiEvent::Select) => {
                let selected = state.selected();
                (true, state.enter(selected, app))
            }
            (UiState::GroupedLogs(state), UiEvent::Enter(selected)) => {
                (true, state.enter(selected, app))
            }
            (UiState::GroupedLogs(state), UiEvent::Copy) => {
                let selected = state.selected();
                let mut table_state = TableState::default();
                table_state.select(Some(0));

                let line = &app.lines[state.lines[selected]];
                let raw = app.get_line(line.index).unwrap_or_default();
                copy_osc(raw);
                (false, UiState::GroupedLogs(state))
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
            (mut ui, UiEvent::EnterFilterMode) if ui.mode() != Mode::FilterInput => {
                ui.set_mode(Mode::FilterInput);
                (true, ui)
            }
            (mut ui, UiEvent::Text(c)) if ui.mode() == Mode::FilterInput => {
                if let Some(filter) = ui.filter_mut() {
                    filter.push(c);
                }
                (true, ui)
            }
            (mut ui, UiEvent::PopText(pop_mode)) if ui.mode() == Mode::FilterInput => {
                if let Some(filter) = ui.filter_mut() {
                    match pop_mode {
                        PopMode::Character => filter.pop(),
                        PopMode::Word => filter.pop_word(),
                    }
                }
                (true, ui)
            }
            (mut ui, UiEvent::ClearFilter) if ui.mode() != Mode::Normal => {
                if let Some(filter) = ui.filter_mut() {
                    filter.clear();
                }
                ui.set_mode(Mode::Normal);
                (true, ui)
            }
            (
                mut ui @ UiState::MatchList(MatchListState {
                    mode: Mode::FilterInput,
                    ..
                }),
                UiEvent::Back,
            ) => {
                if let Some(filter) = ui.filter_mut() {
                    filter.clear();
                }
                ui.set_mode(Mode::Normal);
                (true, ui)
            }

            (
                UiState::Match(MatchState { previous, .. })
                | UiState::GroupedLogs(GroupedLogsState { previous, .. })
                | UiState::Log(LogState { previous, .. })
                | UiState::Errors(ErrorState { previous, .. }),
                UiEvent::Back,
            ) => (true, *previous),
            (state, _) => (false, state),
        }
    }

    pub fn footer_params(&self) -> FooterParams {
        match self.mode() {
            Mode::Normal => FooterParams::Normal { page: self.page() },
            Mode::FilterInput => FooterParams::FilterInput {
                filter: self.filter().unwrap_or(&EMPTY_FILTER),
                page: self.page(),
            },
        }
    }
}

pub enum UiEvent {
    Quit,
    Back,
    Up(usize, bool),
    Down(usize, bool),
    Scroll(isize),
    Errors,
    Select,
    #[allow(dead_code)]
    SelectAt(usize),
    Enter(usize),
    Copy,
    EnterFilterMode,
    ClearFilter,
    Text(char),
    PopText(PopMode),
}

pub enum PopMode {
    Character,
    Word,
}

#[derive(PartialEq)]
pub enum UiPage {
    MatchList,
    Match,
    Logs,
    Log,
    Errors,
}
