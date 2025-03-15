use crate::app::{App, Filter};
use crate::logfile::logline::{format_time, LogLine};
use crate::ui::state::GroupedLogGrouping;
use crate::ui::style::TABLE_HEADER_STYLE;
use crate::ui::table::{ScrollbarTable, ScrollbarTableState};
use crate::ui::UI_HEADER_SIZE;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{StatefulWidget, Widget};
use ratatui::text::Text;
use ratatui::widgets::{Cell, Paragraph, Row, Wrap};

pub struct GroupedLogs<'a> {
    lines: &'a [usize],
    app: &'a App<'a>,
    filter: &'a Filter,
    grouping: GroupedLogGrouping,
}

pub fn grouped_logs<'a>(
    app: &'a App<'a>,
    lines: &'a [usize],
    filter: &'a Filter,
    grouping: GroupedLogGrouping,
) -> GroupedLogs<'a> {
    GroupedLogs {
        lines,
        app,
        filter,
        grouping,
    }
}

impl StatefulWidget for GroupedLogs<'_> {
    type State = ScrollbarTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let line = &self.app.lines[self.lines[state.selected()]];
        let lines = self.lines.iter().copied().map(|i| &self.app.lines[i]);

        let par = match self.grouping {
            GroupedLogGrouping::Message => Paragraph::new(format!(
                "{}{}{}\n\n{} from {} - Nextcloud {}",
                line.exception
                    .as_ref()
                    .map(|e| e.exception.as_ref())
                    .unwrap_or_default(),
                if line.exception.is_some() { ":\n" } else { "" },
                line.message,
                line.level.as_str(),
                line.app,
                line.version,
            ))
            .wrap(Wrap::default()),
            GroupedLogGrouping::Request => Paragraph::new(format!(
                "{} {}\n\n  {} from {} by {} - Nextcloud {}",
                line.method, line.url, line.request_id, line.remote, line.user, line.version,
            ))
            .wrap(Wrap::default()),
        };

        let header = match self.grouping {
            GroupedLogGrouping::Message => [
                Text::from("Remote"),
                Text::from("Method"),
                Text::from("Url"),
                Text::from("Request Id"),
                Text::from("Time").alignment(Alignment::Right),
            ],
            GroupedLogGrouping::Request => [
                Text::from("Level"),
                Text::from("App"),
                Text::from("Message"),
                Text::from(""),
                Text::from("Time").alignment(Alignment::Right),
            ],
        }
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(TABLE_HEADER_STYLE)
        .height(1);

        let widths = match self.grouping {
            GroupedLogGrouping::Message => [
                Constraint::Min(16),
                Constraint::Min(8),
                Constraint::Percentage(100),
                Constraint::Min(25),
                Constraint::Length(27),
            ],
            GroupedLogGrouping::Request => [
                Constraint::Min(16),
                Constraint::Min(8),
                Constraint::Percentage(100),
                Constraint::Length(0),
                Constraint::Length(27),
            ],
        };
        let table = ScrollbarTable::new(
            lines
                .filter(|line| line.matches(self.filter))
                .enumerate()
                .map(|(i, line)| log_row(line, self.grouping, i.abs_diff(state.selected()) < 100)),
            widths,
        )
        .header(header);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(UI_HEADER_SIZE),
                Constraint::Percentage(100),
            ])
            .split(area);

        par.render(layout[0], buf);
        table.render(layout[1], buf, state);
    }
}

fn log_row<'a>(line: &'a LogLine<'a>, grouping: GroupedLogGrouping, is_in_view: bool) -> Row<'a> {
    if is_in_view {
        match grouping {
            GroupedLogGrouping::Message => Row::new([
                Text::from(line.remote.as_str()),
                Text::from(line.method.as_str()),
                Text::from(line.url.as_ref()),
                Text::from(line.request_id.as_str()),
                Text::from(format_time(line.time)).alignment(Alignment::Right),
            ]),
            GroupedLogGrouping::Request => Row::new([
                Text::from(line.level.as_str()),
                Text::from(line.app.as_ref()),
                Text::from(line.message.as_ref()),
                Text::from(""),
                Text::from(format_time(line.time)).alignment(Alignment::Right),
            ]),
        }
    } else {
        Row::default()
    }
}
