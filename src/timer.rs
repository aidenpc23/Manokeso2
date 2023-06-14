use std::time::{Duration, Instant};

pub struct Timer {
    start: Option<Instant>,
    durs: Vec<Duration>,
    i: usize,
}

impl Timer {
    pub fn new(size: usize) -> Self {
        Self {
            start: None,
            durs: vec![Duration::ZERO; size],
            i: 0,
        }
    }
    pub fn push(&mut self, dur: Duration) {
        self.i = (self.i + 1) % self.durs.len();
        self.durs[self.i] = dur;
    }
    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }
    pub fn end(&mut self) {
        let end = Instant::now();
        if let Some(start) = self.start {
            self.start = None;
            self.push(end - start);
        }
    }
    pub fn avg(&self) -> Option<Duration> {
        if self.durs.last().unwrap_or(&Duration::ZERO).is_zero() {
            None
        } else {
            Some(self.durs.iter().sum::<Duration>() / self.durs.len() as u32)
        }
    }
}
