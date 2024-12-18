use crate::app::{App, Filter};
use crate::ui::state::UiPage;
use ratatui::layout::Constraint;
use ratatui::prelude::Style;
use ratatui::style::palette::tailwind;
use ratatui::text::Text;
use ratatui::widgets::{Row, Table};

pub enum FooterParams<'a> {
    Normal { page: UiPage },
    FilterInput { page: UiPage, filter: &'a Filter },
}

pub fn footer<'a>(app: &App<'a>, params: FooterParams<'a>) -> Table<'a> {
    let footer_style = Style::default()
        .bg(tailwind::BLACK)
        .fg(tailwind::GREEN.c600);

    match params {
        FooterParams::Normal { page } => {
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
        FooterParams::FilterInput { filter, page } => {
            let help = filter_help(page);
            let widths = [
                Constraint::Min(u16::try_from(help.chars().count()).unwrap()),
                Constraint::Percentage(100),
            ];

            Table::new(
                [Row::new([
                    Text::from(help),
                    Text::from(format!("- Filter: {}█", filter)),
                ])],
                widths,
            )
            .style(footer_style)
        }
    }
}

fn help(page: UiPage) -> &'static str {
    match page {
        UiPage::MatchList => "«Q» Exit - «Enter» Select - «F» Filter - «E» Show parse errors",
        UiPage::Match => "«Q» Exit - «Enter» Select - «F» Filter - «Esc» Back",
        UiPage::Logs => {
            "«Q» Exit - «F» Filter - «Esc» Back - «C» Copy log line - «R» Show logs for request"
        }
        UiPage::Log => {
            "«Q» Exit - «Esc» Back - «R» Toggle raw - «C» Copy log line - «R» Show logs for request"
        }
        UiPage::Errors => "«Q» Exit - «Esc» Back - «C» Copy log line",
    }
}

fn filter_help(page: UiPage) -> &'static str {
    match page {
        UiPage::MatchList => "«Esc» Clear",
        _ => "«Esc» Clear - «Left» Back",
    }
}
