use crate::app::{App, GroupedLines, LogMatch};
use crate::ui::histogram::sparkline;
use crate::ui::style::TABLE_HEADER_STYLE;
use crate::ui::table::ScrollbarTable;
use ratatui::layout::Constraint;
use ratatui::text::Text;
use ratatui::widgets::{Cell, Row};

pub fn grouped_lines(app: &App, log_match: &LogMatch) -> ScrollbarTable<'static> {
    let grouped = &log_match.grouped;
    let header = [
        Text::from("Level"),
        Text::from("App"),
        Text::from("Message"),
        Text::from("Time"),
        Text::from("Count"),
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
        Constraint::Length(10),
        Constraint::Min(10),
    ];
    ScrollbarTable::new(grouped.iter().map(|group| group_row(app, group)), widths).header(header)
}

fn group_row(app: &App, group: &GroupedLines) -> Row<'static> {
    let line = &app.lines[group.lines[0]];

    Row::new([
        line.level.as_str().to_string(),
        line.app.to_string(),
        line.display(),
        sparkline(&group.histogram.counts(10)),
        group.len().to_string(),
    ])
}
