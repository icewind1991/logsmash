use crate::app::App;
use crate::logline::LogLine;
use crate::ui::style::{TABLE_HEADER_STYLE, TIME_FORMAT};
use crate::ui::table::ScrollbarTable;
use ratatui::layout::{Alignment, Constraint};
use ratatui::text::Text;
use ratatui::widgets::{Cell, Row};
use time::format_description::well_known::Iso8601;

pub fn raw_logs<'a>(app: &'a App, lines: &[usize]) -> ScrollbarTable<'a> {
    let lines = lines.iter().copied().map(|i| &app.lines[i]);
    let header = [
        Text::from("Level"),
        Text::from("App"),
        Text::from("Message"),
        Text::from("Time").alignment(Alignment::Right),
    ]
    .into_iter()
    .map(Cell::from)
    .collect::<Row>()
    .style(TABLE_HEADER_STYLE)
    .height(1);

    let widths = [
        Constraint::Min(10),
        Constraint::Min(20),
        Constraint::Percentage(100),
        Constraint::Length(27),
    ];
    ScrollbarTable::new(lines.map(log_row), widths).header(header)
}

fn log_row(line: &LogLine) -> Row {
    Row::new([
        Text::from(line.level.as_str()),
        Text::from(line.app.as_str()),
        Text::from(line.display()),
        Text::from(line.time.format(&Iso8601::<TIME_FORMAT>).unwrap()).alignment(Alignment::Right),
    ])
}
