use crate::logline::LogLine;
use crate::ui::style::{TABLE_HEADER_STYLE, TABLE_SELECTED_STYLE, TIME_FORMAT};
use ratatui::layout::Constraint;
use ratatui::widgets::{Cell, HighlightSpacing, Row, Table};
use time::format_description::well_known::Iso8601;

pub fn grouped_lines<'a, I: Iterator<Item = &'a LogLine> + 'a>(lines: I) -> Table<'a> {
    let header = ["Level", "App", "Message", "Date"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(TABLE_HEADER_STYLE)
        .height(1);

    let widths = [
        Constraint::Min(10),
        Constraint::Min(20),
        Constraint::Percentage(100),
        Constraint::Min(30),
    ];
    let table = Table::new(lines.map(|line| log_row(line)), widths)
        .header(header)
        .highlight_style(TABLE_SELECTED_STYLE)
        .highlight_spacing(HighlightSpacing::Always);
    table
}

fn log_row(line: &LogLine) -> Row {
    Row::new([
        line.level.as_str().to_string(),
        line.app.to_string(),
        line.message.clone(),
        line.time.format(&Iso8601::<TIME_FORMAT>).unwrap(),
    ])
}
