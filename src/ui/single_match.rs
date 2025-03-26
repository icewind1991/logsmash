use crate::app::{App, Filter, GroupedLines, LogMatch};
use crate::ui::style::TABLE_HEADER_STYLE;
use crate::ui::table::{ScrollbarTable, ScrollbarTableState};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::StatefulWidget;
use ratatui::text::Text;
use ratatui::widgets::{Cell, Row};
use std::iter::once;

pub fn grouped_lines<'a>(
    app: &'a App<'a>,
    log_match: &'a LogMatch,
    filter: &'a Filter,
) -> SingleMatchTable<'a> {
    SingleMatchTable {
        app,
        log_match,
        filter,
    }
}

pub struct SingleMatchTable<'a> {
    app: &'a App<'a>,
    log_match: &'a LogMatch,
    filter: &'a Filter,
}

impl StatefulWidget for SingleMatchTable<'_> {
    type State = ScrollbarTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let grouped = &self.log_match.grouped;
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
        let table = ScrollbarTable::new(
            once(Row::new([
                Text::from("All lines"),
                Text::from(""),
                Text::from(""),
                Text::from(self.log_match.sparkline(self.app)),
                Text::from(self.log_match.count().to_string()),
            ]))
            .chain(
                grouped
                    .iter()
                    .filter(|group| group.matches(self.app, self.filter))
                    .enumerate()
                    .map(|(i, group)| {
                        group_row(self.app, group, i.abs_diff(state.selected()) < 100)
                    }),
            ),
            widths,
        )
        .header(header);
        table.render(area, buf, state);
    }
}

fn group_row<'a>(app: &'a App, group: &'a GroupedLines, is_in_view: bool) -> Row<'a> {
    if is_in_view {
        let line = &app.lines[group.lines[0]];

        Row::new([
            Text::from(line.level.as_str()),
            Text::from(line.app.as_ref()),
            Text::from(line.display()),
            Text::from(group.sparkline(app)),
            Text::from(group.len().to_string()),
        ])
    } else {
        Row::default()
    }
}
