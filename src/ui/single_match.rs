use crate::app::{App, GroupedLines, LogMatch};
use crate::ui::histogram::sparkline;
use crate::ui::style::{TABLE_HEADER_STYLE, TABLE_SELECTED_STYLE};
use ratatui::layout::Constraint;
use ratatui::text::Text;
use ratatui::widgets::{Cell, HighlightSpacing, Row, Table};

pub fn grouped_lines(app: &App, log_match: &LogMatch) -> Table<'static> {
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
    let table = Table::new(grouped.iter().map(|group| group_row(app, group)), widths)
        .header(header)
        .highlight_style(TABLE_SELECTED_STYLE)
        .highlight_spacing(HighlightSpacing::Always);
    table
}

fn group_row(app: &App, group: &GroupedLines) -> Row<'static> {
    let line = &app.lines[group.lines[0]];

    Row::new([
        line.level.as_str().to_string(),
        line.app.to_string(),
        line.message.clone(),
        sparkline(&group.histogram.counts(10)),
        group.len().to_string(),
    ])
}
