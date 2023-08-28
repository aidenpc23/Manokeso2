pub trait SaturatingAdd<T> {
    fn sat_add(self, rhs: T) -> Self;
    fn sat_add_assign(&mut self, rhs: T);
}

impl SaturatingAdd<i32> for u32 {
    fn sat_add(self, rhs: i32) -> Self {
        (self as i32 + rhs).max(0) as u32
    }
    fn sat_add_assign(&mut self, rhs: i32) {
        *self = (*self as i32 + rhs).max(0) as u32
    }
}

impl SaturatingAdd<i32> for usize {
    fn sat_add(self, rhs: i32) -> Self {
        (self as i32 + rhs).max(0) as usize
    }
    fn sat_add_assign(&mut self, rhs: i32) {
        *self = (*self as i32 + rhs).max(0) as usize
    }
}

impl SaturatingAdd<i32> for u64 {
    fn sat_add(self, rhs: i32) -> Self {
        (self as i32 + rhs).max(0) as u64
    }
    fn sat_add_assign(&mut self, rhs: i32) {
        *self = (*self as i32 + rhs).max(0) as u64
    }
}