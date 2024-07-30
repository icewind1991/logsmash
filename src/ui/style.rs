use ratatui::prelude::Style;
use ratatui::style::palette::tailwind;

pub const TABLE_HEADER_STYLE: Style = Style::new().bg(tailwind::BLACK).fg(tailwind::GREEN.c600);
pub const TABLE_SELECTED_STYLE: Style = Style::new().fg(tailwind::BLACK).bg(tailwind::GREEN.c600);
