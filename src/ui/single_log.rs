use crate::app::App;
use crate::logline::{FullException, FullLogLine, LogLine, Trace};
use crate::ui::style::{TABLE_HEADER_STYLE, TIME_FORMAT};
use crate::ui::table::{ScrollbarTable, ScrollbarTableState};
use ratatui::prelude::*;
use ratatui::widgets::{Cell, Paragraph, Row, Wrap};
use std::iter::once;
use time::format_description::well_known::Iso8601;

pub fn single_log(app: &App, line: &LogLine) -> SingleLog {
    let raw_line = app.get_line(line.index).unwrap();
    let line: FullLogLine = serde_json::from_str(raw_line).unwrap();
    SingleLog { line }
}

pub struct SingleLog {
    line: FullLogLine,
}

impl StatefulWidget for SingleLog {
    type State = ScrollbarTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let par = Paragraph::new(format!(
            "{}\n\n  {} {}\n  {}\n\n  from {} by {} at {}",
            self.line.message,
            self.line.method,
            self.line.url,
            self.line.user_agent,
            self.line.remote_address,
            self.line.user,
            self.line.time.format(&Iso8601::<TIME_FORMAT>).unwrap()
        ))
        .wrap(Wrap::default());

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(7), Constraint::Percentage(100)])
            .split(area);

        par.render(layout[0], buf);

        if let Some(exception) = &self.line.exception {
            StatefulWidget::render(render_exception(exception), layout[1], buf, state);
        }
    }
}

pub fn render_exception(exception: &FullException) -> ScrollbarTable {
    let header = [
        Text::from("File"),
        Text::from("Line").alignment(Alignment::Right),
        Text::from("Function"),
    ]
    .into_iter()
    .map(Cell::from)
    .collect::<Row>()
    .style(TABLE_HEADER_STYLE)
    .height(1);

    let widths = [
        Constraint::Percentage(40),
        Constraint::Min(10),
        Constraint::Percentage(60),
    ];
    let rows = exception.stack().flat_map(exception_trace);
    ScrollbarTable::new(rows, widths).header(header)
}

fn exception_trace(exception: &FullException) -> impl Iterator<Item = Row> + '_ {
    let exception_row = Row::new([
        Text::from(""),
        Text::from(exception.line.to_string()).alignment(Alignment::Right),
        Text::from(exception.file.clone()),
    ])
    .style(TABLE_HEADER_STYLE);
    let trace_rows = exception.trace.iter().map(trace_line);
    once(exception_row).chain(trace_rows)
}

fn trace_line(trace: &Trace) -> Row {
    Row::new([
        Text::from(trace.file.clone()),
        Text::from(if trace.line > 0 {
            trace.line.to_string()
        } else {
            String::new()
        })
        .alignment(Alignment::Right),
        Text::from(trace.function().to_string()),
    ])
}
