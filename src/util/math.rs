pub trait SaturatingAdd<T> {
    fn sat_add(self, rhs: T) -> Self;
    fn sat_add_assign(&mut self, rhs: T);
}

impl SaturatingAdd<i32> for u32 {
    fn sat_add(self, rhs: i32) -> Self {
        if rhs < 0 {
            self.saturating_sub(rhs.abs() as u32)
        } else {
            self.saturating_add(rhs as u32)
        }
    }
    fn sat_add_assign(&mut self, rhs: i32) {
        if rhs < 0 {
            *self = self.saturating_sub(rhs.abs() as u32)
        } else {
            *self = self.saturating_add(rhs as u32)
        }
    }
}

impl SaturatingAdd<i32> for usize {
    fn sat_add(self, rhs: i32) -> Self {
        if rhs < 0 {
            self.saturating_sub(rhs.abs() as usize)
        } else {
            self.saturating_add(rhs as usize)
        }
    }
    fn sat_add_assign(&mut self, rhs: i32) {
        if rhs < 0 {
            *self = self.saturating_sub(rhs.abs() as usize)
        } else {
            *self = self.saturating_add(rhs as usize)
        }
    }
}

impl SaturatingAdd<i32> for u64 {
    fn sat_add(self, rhs: i32) -> Self {
        if rhs < 0 {
            self.saturating_sub(rhs.abs() as u64)
        } else {
            self.saturating_add(rhs.abs() as u64)
        }
    }
    fn sat_add_assign(&mut self, rhs: i32) {
        if rhs < 0 {
            *self = self.saturating_sub(rhs.abs() as u64)
        } else {
            *self = self.saturating_add(rhs.abs() as u64)
        }
    }
}