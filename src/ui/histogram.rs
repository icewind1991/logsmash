use crate::timegraph::TimeGraph;
use ratatui::prelude::*;
use ratatui::widgets::Sparkline;

pub struct UiHistogram<'a> {
    data: &'a TimeGraph,
}

impl<'a> UiHistogram<'a> {
    pub fn new(data: &'a TimeGraph) -> Self {
        UiHistogram { data }
    }
}

impl Widget for UiHistogram<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let values: Vec<_> = self.data.counts(area.width as usize).collect();
        let sparkline = Sparkline::default().data(&values);
        sparkline.render(area, buf)
    }
}
