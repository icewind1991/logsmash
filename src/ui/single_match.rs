use crate::app::{App, Filter, GroupedLines, LogMatch};
use crate::ui::style::TABLE_HEADER_STYLE;
use crate::ui::table::ScrollbarTable;
use ratatui::layout::Constraint;
use ratatui::text::Text;
use ratatui::widgets::{Cell, Row};
use std::iter::once;

pub fn grouped_lines<'a>(
    app: &'a App<'a>,
    log_match: &'a LogMatch,
    filter: &Filter,
) -> ScrollbarTable<'a> {
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
    ScrollbarTable::new(
        once(Row::new([
            Text::from("All lines"),
            Text::from(""),
            Text::from(""),
            Text::from(log_match.sparkline.as_str()),
            Text::from(log_match.lines.len().to_string()),
        ]))
        .chain(
            grouped
                .iter()
                .filter(|group| group.matches(app, filter))
                .map(|group| group_row(app, group)),
        ),
        widths,
    )
    .header(header)
}

fn group_row<'a>(app: &'a App, group: &'a GroupedLines) -> Row<'a> {
    let line = &app.lines[group.lines[0]];

    Row::new([
        Text::from(line.level.as_str()),
        Text::from(line.app.as_ref()),
        Text::from(line.display()),
        Text::from(group.sparkline.as_str()),
        Text::from(group.len().to_string()),
    ])
}
