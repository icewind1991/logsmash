use crate::app::{App, LogMatch};
use crate::ui::histogram::sparkline;
use crate::ui::style::{TABLE_HEADER_STYLE, TABLE_SELECTED_STYLE};
use itertools::Either;
use ratatui::prelude::*;
use ratatui::widgets::{Cell, HighlightSpacing, Row, Table};
use std::fmt::Write;
use std::iter::{empty, once};

pub fn match_list(app: &App) -> Table {
    let header = [
        Text::from("Statement"),
        Text::from("File"),
        Text::from("Line").alignment(Alignment::Right),
        Text::from("Time"),
        Text::from("Count"),
    ]
    .into_iter()
    .map(Cell::from)
    .collect::<Row>()
    .style(TABLE_HEADER_STYLE)
    .height(1);

    let widths = [
        Constraint::Percentage(70),
        Constraint::Percentage(30),
        Constraint::Min(6),
        Constraint::Length(10),
        Constraint::Min(10),
    ];

    let all = Row::new([
        Text::from("All lines"),
        Text::from(String::new()),
        Text::from(String::new()).alignment(Alignment::Right),
        Text::from(sparkline(&app.histogram.counts(10))),
        Text::from(app.lines.len().to_string()),
    ]);
    let unmatched = if app.unmatched.is_empty() {
        Either::Right(empty())
    } else {
        Either::Left(once(Row::new([
            Text::from("Unmatched lines"),
            Text::from(String::new()),
            Text::from(String::new()).alignment(Alignment::Right),
            Text::from(sparkline(&app.unmatched_histogram.counts(10))),
            Text::from(app.unmatched.len().to_string()),
        ])))
    };

    Table::new(
        once(all)
            .chain(app.matches.iter().map(|result| log_row(result, app)))
            .chain(unmatched),
        widths,
    )
    .header(header)
    .highlight_style(TABLE_SELECTED_STYLE)
    .highlight_spacing(HighlightSpacing::Always)
}

fn log_row<'a>(result: &LogMatch, app: &'a App) -> Row<'a> {
    let mut message = String::new();
    let mut paths = String::new();
    let mut lines = String::new();
    for index in result.result.iter() {
        let statement = app.log_statements.get(index).expect("invalid match index");
        writeln!(&mut message, "{}", statement.message()).unwrap();
        writeln!(&mut paths, "{}", statement.path()).unwrap();
        writeln!(&mut lines, "{}", statement.line).unwrap();
    }
    Row::new([
        Text::from(message),
        Text::from(paths),
        Text::from(lines).alignment(Alignment::Right),
        Text::from(sparkline(&result.histogram.counts(10))),
        Text::from(result.count().to_string()),
    ])
    .height(result.result.len() as u16)
}
