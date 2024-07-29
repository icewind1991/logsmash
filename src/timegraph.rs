use hdrhistogram::Histogram;
use time::OffsetDateTime;

pub struct TimeGraph {
    histogram: Histogram<u64>,
    start: u64,
    end: u64,
}

impl TimeGraph {
    pub fn new(start: OffsetDateTime, end: OffsetDateTime) -> Self {
        let histogram = Histogram::new_with_bounds(
            1,
            end.unix_timestamp() as u64 - start.unix_timestamp() as u64 + 1,
            3,
        )
        .unwrap();
        TimeGraph {
            histogram,
            start: start.unix_timestamp() as u64,
            end: end.unix_timestamp() as u64,
        }
    }

    pub fn add(&mut self, time: OffsetDateTime) {
        self.histogram
            .record(time.unix_timestamp() as u64 - self.start + 1)
            .unwrap()
    }

    pub fn counts(&self, buckets: usize) -> Vec<u64> {
        let step = (self.end - self.start + 1) / buckets as u64;

        self.histogram
            .iter_linear(step)
            .map(|val| val.count_since_last_iteration())
            .collect()
    }
}
