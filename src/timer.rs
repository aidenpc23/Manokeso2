use std::time::{Duration, Instant};

pub struct Timer {
    start: Option<Instant>,
    durs: Vec<Duration>,
    i: usize,
    enabled: bool,
}

pub struct CounterTimer {
    start: Option<Instant>,
    durs: Vec<Duration>,
    i: usize,
    enabled: bool,
}

impl Timer {
    pub fn new(size: usize) -> Self {
        Self {
            start: None,
            durs: vec![Duration::ZERO; size],
            i: 0,
            enabled: true,
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
    pub fn end(&mut self) {
        if self.enabled {
            let end = Instant::now();
            if let Some(start) = self.start {
                self.start = None;
                self.push(end - start);
            }
        }
    }
    pub fn ready(&self) -> bool {
        !self.durs.last().unwrap_or(&Duration::ZERO).is_zero()
    }
    pub fn avg(&self) -> Duration {
        self.durs.iter().sum::<Duration>() / self.durs.len() as u32
    }
}
