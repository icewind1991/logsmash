use crate::ui::find_hit_row;
use crate::ui::state::{Mode, UiPage, UiState};
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyModifiers, MouseButton, MouseEventKind};
use std::io;
use std::time::Duration;

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

pub fn handle_events(page: UiPage, ui_state: &UiState) -> io::Result<Option<UiEvent>> {
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key) if key.kind == event::KeyEventKind::Press => {
                return Ok(match (ui_state.mode(), key.code) {
                    (_, KeyCode::Char('c')) if key.modifiers == KeyModifiers::CONTROL => {
                        Some(UiEvent::Quit)
                    }
                    (Mode::Normal, KeyCode::Esc) => Some(UiEvent::Back),

                    (Mode::Normal, KeyCode::Char('q')) => Some(UiEvent::Quit),
                    (Mode::Normal, KeyCode::Char('e')) if page == UiPage::MatchList => {
                        Some(UiEvent::Errors)
                    }
                    (_, KeyCode::Left) if page != UiPage::MatchList => Some(UiEvent::Back),
                    (_, KeyCode::Down) => Some(UiEvent::Down(1, true)),
                    (_, KeyCode::Up) => Some(UiEvent::Up(1, true)),
                    (_, KeyCode::PageDown) => Some(UiEvent::Down(10, false)),
                    (_, KeyCode::PageUp) => Some(UiEvent::Up(10, false)),
                    (_, KeyCode::End) => Some(UiEvent::Down(usize::MAX, false)),
                    (_, KeyCode::Home) => Some(UiEvent::Up(usize::MAX, false)),
                    (_, KeyCode::Enter | KeyCode::Right) => Some(UiEvent::Select),
                    (Mode::Normal, KeyCode::Char('c')) => Some(UiEvent::Copy),
                    (Mode::Normal, KeyCode::F(4) | KeyCode::Char('f')) => {
                        Some(UiEvent::EnterFilterMode)
                    }

                    (Mode::FilterInput, KeyCode::Esc) => Some(UiEvent::ClearFilter),
                    (Mode::FilterInput, KeyCode::F(4)) => Some(UiEvent::Back),
                    (Mode::FilterInput, KeyCode::Backspace) => {
                        Some(UiEvent::PopText(PopMode::Character))
                    }
                    (Mode::FilterInput, KeyCode::Char('w'))
                        if key.modifiers == KeyModifiers::CONTROL =>
                    {
                        Some(UiEvent::PopText(PopMode::Word))
                    }
                    (Mode::FilterInput, KeyCode::Char(c)) => Some(UiEvent::Text(c)),
                    _ => None,
                });
            }
            Event::Mouse(mouse) => {
                return Ok(match mouse.kind {
                    MouseEventKind::ScrollUp => Some(UiEvent::Scroll(-1)),
                    MouseEventKind::ScrollDown => Some(UiEvent::Scroll(1)),
                    MouseEventKind::Down(MouseButton::Left) => {
                        find_hit_row(mouse.row, ui_state).map(UiEvent::Enter)
                    }
                    _ => None,
                })
            }
            _ => {}
        }
    }
    Ok(None)
}
