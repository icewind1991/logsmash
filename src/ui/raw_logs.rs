use crate::app::App;
use crate::logline::LogLine;
use crate::ui::style::{TABLE_HEADER_STYLE, TABLE_SELECTED_STYLE, TIME_FORMAT};
use ratatui::layout::{Alignment, Constraint};
use ratatui::text::Text;
use ratatui::widgets::{Cell, HighlightSpacing, Row, Table};
use time::format_description::well_known::Iso8601;

pub fn raw_logs(app: &App, lines: &[usize]) -> Table<'static> {
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
    let table = Table::new(lines.map(log_row), widths)
        .header(header)
        .highlight_style(TABLE_SELECTED_STYLE)
        .highlight_spacing(HighlightSpacing::Always);
    table
}

fn log_row(line: &LogLine) -> Row<'static> {
    Row::new([
        Text::from(line.level.as_str().to_string()),
        Text::from(line.app.to_string()),
        Text::from(line.message.clone()),
        Text::from(line.time.format(&Iso8601::<TIME_FORMAT>).unwrap()).alignment(Alignment::Right),
    ])
}
