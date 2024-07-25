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
        let values = self.data.counts(area.width as usize);
        let sparkline = Sparkline::default().data(&values);
        sparkline.render(area, buf)
    }
}

const SPARKS: &[char] = &[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

pub fn sparkline(values: &[u64]) -> String {
    let max = values.iter().copied().max().unwrap() as f64;
    let len = SPARKS.len() as f64 - 1.0;
    values
        .iter()
        .copied()
        .map(|val| {
            let rel = val as f64 / max;
            SPARKS[(rel * len) as usize]
        })
        .collect()
}

#[test]
fn test_sparkline() {
    assert_eq!(" ▇█", sparkline(&[0, 900, 1000]));
}
