use hdrhistogram::Histogram;
use std::cmp::max;
use time::OffsetDateTime;

#[derive(Clone)]
pub struct TimeGraph {
    histogram: Histogram<u64>,
    start: u64,
    end: u64,
}

impl TimeGraph {
    pub fn new(start: OffsetDateTime, end: OffsetDateTime) -> Self {
        let histogram = Histogram::new_with_bounds(
            1,
            max(
                (end.unix_timestamp() as u64).saturating_sub(start.unix_timestamp() as u64) + 1,
                4,
            ),
            3,
        )
        .expect("Failed to build histogram");
        TimeGraph {
            histogram,
            start: start.unix_timestamp() as u64,
            end: end.unix_timestamp() as u64,
        }
    }

    pub fn add(&mut self, time: OffsetDateTime) {
        self.histogram
            .record(time.unix_timestamp() as u64 - self.start + 1)
            .ok();
    }

    pub fn counts(&self, buckets: usize) -> impl Iterator<Item = u64> + '_ {
        let step = (self.end - self.start + 1) / buckets as u64;

        self.histogram
            .iter_linear(max(1, step))
            .map(|val| val.count_since_last_iteration())
    }

    pub fn sparkline<const N: usize>(&self) -> String {
        let mut values = [0; N];
        for (value, count) in values.iter_mut().zip(self.counts(N)) {
            *value = count;
        }
        let max = values.iter().copied().max().unwrap_or_default() as f64;
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
}

const SPARKS: &[char] = &[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
