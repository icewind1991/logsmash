use crate::app::App;
use crate::logline::{format_time, FullException, FullLogLine, LogLine, Trace};
use crate::parse_line_full;
use crate::ui::style::TABLE_HEADER_STYLE;
use crate::ui::table::{ScrollbarTable, ScrollbarTableState};
use ratatui::prelude::*;
use ratatui::widgets::{Cell, Paragraph, Row, Wrap};
use std::iter::once;

pub fn single_log(app: &App, line: &LogLine) -> SingleLog {
    let raw_line = app.get_line(line.index);
    let line = raw_line.and_then(|raw_line| parse_line_full(raw_line).ok());
    SingleLog::new(line)
}

pub struct SingleLog {
    line: Option<FullLogLine>,
    path_prefix_length: usize,
}

impl SingleLog {
    pub fn new(line: Option<FullLogLine>) -> Self {
        let path_prefix_length = line
            .as_ref()
            .and_then(|line| line.exception.as_ref())
            .map(|ex| find_path_prefix_length(ex.trace.iter().map(|t| t.file.as_str())))
            .unwrap_or_default();
        SingleLog {
            line,
            path_prefix_length,
        }
    }
}

impl StatefulWidget for SingleLog {
    type State = ScrollbarTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        if let Some(line) = self.line {
            let par = Paragraph::new(format!(
                "{}\n\n  {} {}\n  {}\n\n  {} from {} by {} at {} - Nextcloud {}",
                line.message,
                line.method,
                line.url,
                line.user_agent,
                line.request_id,
                line.remote_address,
                line.user,
                format_time(line.time),
                line.version,
            ))
            .wrap(Wrap::default());

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Min(7),
                    Constraint::Min(5),
                    Constraint::Percentage(100),
                ])
                .split(area);

            par.render(layout[0], buf);

            if let Some(exception) = &line.exception {
                if line.message.contains(&exception.message) {
                    StatefulWidget::render(
                        render_exception(exception, self.path_prefix_length),
                        layout[1].union(layout[2]),
                        buf,
                        state,
                    );
                } else {
                    let ex_par = Paragraph::new(format!(
                        "\n{}:\n  {}",
                        exception.exception, exception.message
                    ))
                    .wrap(Wrap::default());
                    ex_par.render(layout[1], buf);
                    StatefulWidget::render(
                        render_exception(exception, self.path_prefix_length),
                        layout[2],
                        buf,
                        state,
                    );
                }
            }
        } else {
            let par = Paragraph::new("Failed to parse log line").wrap(Wrap::default());
            par.render(area, buf);
        }
    }
}

pub fn render_exception(exception: &FullException, path_prefix_length: usize) -> ScrollbarTable {
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
    let rows = exception
        .stack()
        .flat_map(move |e| exception_trace(e, path_prefix_length));
    ScrollbarTable::new(rows, widths).header(header)
}

fn exception_trace(
    exception: &FullException,
    path_prefix_length: usize,
) -> impl Iterator<Item = Row> + '_ {
    let exception_row = Row::new([
        Text::from(
            exception
                .file
                .as_str()
                .get(path_prefix_length..)
                .unwrap_or_default(),
        ),
        Text::from(exception.line.to_string()).alignment(Alignment::Right),
        Text::from(""),
    ])
    .style(TABLE_HEADER_STYLE);
    let trace_rows = exception
        .trace
        .iter()
        .map(move |t| trace_line(t, path_prefix_length));
    once(exception_row).chain(trace_rows)
}

fn trace_line(trace: &Trace, path_prefix_length: usize) -> Row {
    Row::new([
        Text::from(
            trace
                .file
                .as_str()
                .get(path_prefix_length..)
                .unwrap_or_default(),
        ),
        Text::from(if trace.line > 0 {
            trace.line.to_string()
        } else {
            String::new()
        })
        .alignment(Alignment::Right),
        Text::from(trace.function().to_string()),
    ])
}

fn find_path_prefix_length<'a, I: Iterator<Item = &'a str>>(paths: I) -> usize {
    let patterns = [
        "/3rdparty/",
        "/apps/",
        "/lib/private",
        "/remote.php",
        "/public.php",
        "/index.php",
    ];
    for path in paths {
        for pattern in patterns {
            if let Some(offset) = path.find(pattern) {
                return offset + 1;
            }
        }
    }
    0
}
