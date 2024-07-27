use crate::app::App;
use crate::logline::LogLine;
use ratatui::widgets::{Paragraph, Wrap};

pub fn single_log<'a>(_app: &App, line: &'a LogLine) -> Paragraph<'a> {
    Paragraph::new(line.display()).wrap(Wrap::default())
}
