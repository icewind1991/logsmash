use crate::ui::style::TABLE_SELECTED_STYLE;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Margin, Rect};
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, Row, Scrollbar, ScrollbarOrientation, ScrollbarState,
    StatefulWidget, Table, TableState,
};

pub struct ScrollbarTable<'a> {
    table: Table<'a>,
    scrollbar: Scrollbar<'a>,
}

impl<'a> ScrollbarTable<'a> {
    pub fn new<R, C>(rows: R, widths: C) -> Self
    where
        R: IntoIterator,
        R::Item: Into<Row<'a>>,
        C: IntoIterator,
        C::Item: Into<Constraint>,
    {
        let rows: Vec<_> = rows.into_iter().collect();
        ScrollbarTable {
            table: Table::new(rows, widths)
                .block(Block::new().borders(Borders::RIGHT))
                .highlight_style(TABLE_SELECTED_STYLE)
                .highlight_spacing(HighlightSpacing::Always),
            scrollbar: Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
        }
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn header(mut self, header: Row<'a>) -> Self {
        self.table = self.table.header(header);
        self
    }
}

#[derive(Clone)]
pub struct ScrollbarTableState {
    count: usize,
    table: TableState,
    scrollbar: ScrollbarState,
}

impl ScrollbarTableState {
    pub fn new(count: usize) -> Self {
        let mut table = TableState::new();
        table.select(Some(0));
        ScrollbarTableState {
            count,
            table,
            scrollbar: ScrollbarState::new(count),
        }
    }

    pub fn selected(&self) -> usize {
        self.table.selected().unwrap_or_default()
    }

    pub fn offset(&self) -> usize {
        self.table.offset()
    }

    pub fn row_count(&self) -> usize {
        self.count
    }

    pub fn up(&mut self, step: usize, rollover: bool) -> usize {
        let current = self.table.selected().unwrap_or(0);
        let after = if step > current {
            if rollover {
                self.count - 1
            } else {
                0
            }
        } else {
            current - step
        };
        self.table.select(Some(after));
        self.scrollbar = self.scrollbar.position(after);
        after
    }

    pub fn down(&mut self, step: usize, rollover: bool) -> usize {
        let current = self.table.selected().unwrap_or(0);
        let after = if step >= self.count - current {
            if rollover {
                0
            } else {
                self.count - 1
            }
        } else {
            current + step
        };
        self.table.select(Some(after));
        self.scrollbar = self.scrollbar.position(after);
        after
    }

    pub fn scroll(&mut self, step: isize) {
        *self.table.offset_mut() = self.table.offset().saturating_add_signed(step);
        let selected = self
            .selected()
            .saturating_add_signed(step)
            .min(self.count - 1);
        self.table.select(Some(selected));
        self.scrollbar = self.scrollbar.position(selected);
    }

    pub fn select(&mut self, selected: usize) {
        self.table.select(Some(selected));
        self.scrollbar = self.scrollbar.position(selected);
    }
}

impl<'a> StatefulWidget for ScrollbarTable<'a> {
    type State = ScrollbarTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidget::render(self.table, area, buf, &mut state.table);
        StatefulWidget::render(
            self.scrollbar,
            area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            buf,
            &mut state.scrollbar,
        );
    }
}
