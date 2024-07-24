use ratatui::prelude::Style;
use ratatui::style::palette::tailwind;
use time::format_description::well_known::iso8601::{Config, EncodedConfig, TimePrecision};

pub const TABLE_HEADER_STYLE: Style = Style::new().bg(tailwind::BLACK).fg(tailwind::GREEN.c600);
pub const TABLE_SELECTED_STYLE: Style = Style::new().fg(tailwind::BLACK).bg(tailwind::GREEN.c600);
pub const TIME_FORMAT: EncodedConfig = Config::DEFAULT
    .set_time_precision(TimePrecision::Second {
        decimal_digits: None,
    })
    .encode();
