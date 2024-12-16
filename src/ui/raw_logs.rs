use crate::app::App;
use crate::logline::{format_time, LogLine};
use crate::ui::style::TABLE_HEADER_STYLE;
use crate::ui::table::ScrollbarTable;
use ratatui::layout::{Alignment, Constraint};
use ratatui::text::Text;
use ratatui::widgets::{Cell, Row};

pub fn raw_logs<'a>(app: &'a App<'a>, lines: &[usize], filter: &str) -> ScrollbarTable<'a> {
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
    ScrollbarTable::new(
        lines.filter(|line| line.matches(filter)).map(log_row),
        widths,
    )
    .header(header)
}

fn log_row<'a>(line: &'a LogLine<'a>) -> Row<'a> {
    Row::new([
        Text::from(line.level.as_str()),
        Text::from(line.app.as_ref()),
        Text::from(line.display()),
        Text::from(format_time(line.time)).alignment(Alignment::Right),
    ])
}
