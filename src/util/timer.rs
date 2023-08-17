use std::time::{Duration, Instant};

pub struct Timer {
    start: Option<Instant>,
    durs: Vec<Duration>,
    i: usize,
    enabled: bool,
    duration: Duration,
}

impl Timer {
    /// Creates a new timer that can store up to `size` durations.
    /// The `avg` function will report the average time taken roughly over the provided duration.
    /// The size should be at least as many times as you expect to run the timer within the
    /// provided duration.
    pub fn new(duration: Duration, size: usize) -> Self {
        Self {
            start: None,
            durs: vec![Duration::ZERO; size],
            i: 0,
            enabled: true,
            duration,
        }
    }
    pub fn push(&mut self, dur: Duration) {
        self.i = (self.i + 1) % self.durs.len();
        self.durs[self.i] = dur;
    }
    pub fn start(&mut self) {
        if self.enabled {
            self.start = Some(Instant::now());
        }
    }
    pub fn stop(&mut self) {
        if self.enabled {
            let end = Instant::now();
            if let Some(start) = self.start {
                self.start = None;
                self.push(end - start);
            }
        }
    }
    pub fn avg(&self) -> Duration {
        let mut sum = Duration::ZERO;
        let mut count = 0;
        for i in 0..self.durs.len() {
            let dur = self.durs[(self.i + i) % self.durs.len()];
            if dur == Duration::ZERO {
                break;
            }
            sum += dur;
            count += 1;
            if sum > self.duration {
                break;
            }
        }
        sum.checked_div(count).unwrap_or(Duration::ZERO)
    }
}
