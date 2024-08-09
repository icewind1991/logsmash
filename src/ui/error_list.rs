use crate::app::App;
use crate::ui::style::TABLE_HEADER_STYLE;
use crate::ui::table::ScrollbarTable;
use ratatui::layout::Constraint;
use ratatui::text::Text;
use ratatui::widgets::{Cell, Row};

pub fn error_list(app: &App) -> ScrollbarTable {
    let header = [Text::from("Error"), Text::from("Line")]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(TABLE_HEADER_STYLE)
        .height(1);

    let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];
    ScrollbarTable::new(app.error_lines.iter().map(error_row), widths).header(header)
}

fn error_row((line, err): &(String, serde_json::Error)) -> Row {
    Row::new([Text::from(format!("{err}")), Text::from(line.as_str())])
}
