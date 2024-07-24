use crate::app::{App, LogMatch};
use crate::ui::style::{TABLE_HEADER_STYLE, TABLE_SELECTED_STYLE};
use ratatui::prelude::*;
use ratatui::widgets::{Cell, HighlightSpacing, Row, Table};
use std::fmt::Write;

pub fn match_list(app: &App) -> Table {
    let header = ["Statement", "File", "Line", "Count"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(TABLE_HEADER_STYLE)
        .height(1);

    let widths = [
        Constraint::Percentage(60),
        Constraint::Percentage(40),
        Constraint::Min(10),
        Constraint::Min(10),
    ];
    let table = Table::new(
        app.matches.iter().map(|result| log_row(result, app)),
        widths,
    )
    .header(header)
    .highlight_style(TABLE_SELECTED_STYLE)
    .highlight_spacing(HighlightSpacing::Always);
    table
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
    Row::new([message, paths, lines, result.count().to_string()]).height(result.result.len() as u16)
}
