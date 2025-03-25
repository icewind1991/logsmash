use crate::app::{App, Filter, LogMatch};
use crate::ui::style::TABLE_HEADER_STYLE;
use crate::ui::table::ScrollbarTable;
use itertools::Either;
use ratatui::prelude::*;
use ratatui::widgets::{Cell, Row};
use std::fmt::Write;
use std::iter::{empty, once};

pub fn match_list<'a>(app: &'a App<'a>, filter: &Filter) -> ScrollbarTable<'a> {
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

    let all = log_row(&app.all, app, "All lines");
    let unmatched = if app.unmatched.count() == 0 {
        Either::Right(empty())
    } else {
        Either::Left(once(log_row(&app.unmatched, app, "Unmatched lines")))
    };

    ScrollbarTable::new(
        once(all)
            .chain(
                app.matches
                    .iter()
                    .filter(|result| result.matches(app, filter))
                    .map(|result| log_row(result, app, "")),
            )
            .chain(unmatched),
        widths,
    )
    .header(header)
}

fn log_row<'a>(result: &'a LogMatch, app: &'a App, name: &'static str) -> Row<'a> {
    if let Some(match_result) = &result.result {
        let mut message = String::new();
        let mut paths = String::new();
        let mut lines = String::new();
        for index in match_result.iter() {
            let statement = app.log_statements.get(index).expect("invalid match index");
            writeln!(&mut message, "{}", statement.message()).ok();
            writeln!(&mut paths, "{}", statement.path()).ok();
            writeln!(&mut lines, "{}", statement.line).ok();
        }
        Row::new([
            Text::from(message),
            Text::from(paths),
            Text::from(lines).alignment(Alignment::Right),
            Text::from(result.sparkline.as_str()),
            Text::from(result.count().to_string()),
        ])
        .height(match_result.len() as u16)
    } else {
        Row::new([
            Text::from(name),
            Text::from(""),
            Text::from(""),
            Text::from(result.sparkline.as_str()),
            Text::from(result.count().to_string()),
        ])
    }
}
