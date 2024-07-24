use crate::app::{App, LogMatch};
use ratatui::prelude::*;
use ratatui::style::palette::tailwind;
use ratatui::widgets::{Cell, HighlightSpacing, Row, Table};
use std::fmt::Write;

pub fn match_list(app: &App) -> Table {
    let header_style = Style::default()
        .bg(tailwind::BLACK)
        .fg(tailwind::GREEN.c600);
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .bg(tailwind::BLACK)
        .fg(tailwind::GREEN.c600);

    let header = ["Statement", "File", "Line", "Count"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
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
    .highlight_style(selected_style)
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
    Row::new([message, paths, lines, result.count.to_string()]).height(result.result.len() as u16)
}
