use crate::app::App;
use crate::ui::state::UiPage;
use ratatui::layout::Constraint;
use ratatui::prelude::Style;
use ratatui::style::palette::tailwind;
use ratatui::widgets::{Row, Table};

pub fn footer(app: &App, page: UiPage) -> Table {
    let footer_style = Style::default()
        .bg(tailwind::BLACK)
        .fg(tailwind::GREEN.c600);

    let widths = [
        Constraint::Percentage(100),
        Constraint::Min(25),
        Constraint::Min(20),
    ];

    Table::new(
        [Row::new([
            help(page).to_string(),
            format!("{} unmatched items", app.unmatched.len()),
            format!("{} parse errors", app.error_count),
        ])],
        widths,
    )
    .style(footer_style)
}

fn help(page: UiPage) -> &'static str {
    match page {
        UiPage::MatchList => "«Q» Exit - «Enter» Select",
        UiPage::Match | UiPage::All | UiPage::Unmatched => "«Q» Exit - «Esc» Back",
    }
}
