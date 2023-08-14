pub struct BoardView {
    pub bx: f32,
    pub by: f32,
    pub xs: usize,
    pub xe: usize,
    pub ys: usize,
    pub ye: usize,
}

impl Default for BoardView {
    fn default() -> Self {
        return Self {
            bx: 0.0,
            by: 0.0,
            xs: 0,
            xe: 0,
            ys: 0,
            ye: 0,
        };
    }
}

impl PartialEq for BoardView {
    fn eq(&self, other: &Self) -> bool {
        self.bx == other.bx
            && self.by == other.by
            && self.xs == other.xs
            && self.xe == other.xe
            && self.ys == other.ys
            && self.ye == other.ye
    }
}
