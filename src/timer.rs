use std::time::Duration;

pub struct Timer {
    durs: Vec<Duration>,
    i: usize,
}

impl Timer {
    pub fn new(size: usize) -> Self {
        Self {
            durs: vec![Duration::new(u64::MAX, 0); size],
            i: 0,
        }
    }
    pub fn push(&mut self, dur: Duration) {
        self.i = (self.i + 1) % self.durs.len();
        self.durs[self.i] = dur;
    }
    pub fn avg(&self) -> Duration {
        self.durs.iter().sum::<Duration>() / self.durs.len() as u32
    }
}
