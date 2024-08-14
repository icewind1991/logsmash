use crate::app::App;
use crate::ui::state::UiPage;
use ratatui::layout::Constraint;
use ratatui::prelude::Style;
use ratatui::style::palette::tailwind;
use ratatui::text::Text;
use ratatui::widgets::{Row, Table};

pub fn footer<'a>(app: &App<'a>, page: UiPage) -> Table<'a> {
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
            Text::from(help(page)),
            Text::from(format!("{} unmatched items", app.unmatched.lines.len())),
            Text::from(format!("{} parse errors", app.error_count)),
        ])],
        widths,
    )
    .style(footer_style)
}

fn help(page: UiPage) -> &'static str {
    match page {
        UiPage::MatchList => "«Q» Exit - «Enter» Select - «E» Show parse errors",
        UiPage::Match => "«Q» Exit - «Enter» Select - «Esc» Back",
        UiPage::Logs => "«Q» Exit - «Esc» Back - «C» Copy log line",
        UiPage::Log => "«Q» Exit - «Esc» Back - «R» Toggle raw - «C» Copy log line",
        UiPage::Errors => "«Q» Exit - «Esc» Back - «C» Copy log line",
    }
}
