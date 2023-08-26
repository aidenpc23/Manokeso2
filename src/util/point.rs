use std::ops::{Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq, Default)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Point<f32> {
    pub const CARDINAL_DIRECTIONS: [Self; 4] = [
        Self { x: 1.0, y: 0.0 },
        Self { x: 0.0, y: 1.0 },
        Self { x: -1.0, y: 0.0 },
        Self { x: 0.0, y: -1.0 },
    ];

    pub const CORNERS: [Self; 4] = [
        Self { x: 1.0, y: 1.0 },
        Self { x: -1.0, y: 1.0 },
        Self { x: -1.0, y: -1.0 },
        Self { x: 1.0, y: -1.0 },
    ];

    pub fn dist(&self, other: Point<f32>) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }

    pub fn norm(self) -> Point<f32> {
        self / self.mag()
    }

    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl<T: Default> Point<T> {
    pub fn zero() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Point<T> {
    pub fn index(&self, width: T) -> T {
        self.y * width + self.x
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T: Mul<Output = T>> Mul for Point<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Div<Output = T>> Div for Point<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: Div<Output = T> + Copy> DivAssign for Point<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Point<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Point<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Mul<Output = T> + Copy> MulAssign<T> for Point<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl Into<Point<f32>> for Point<usize> {
    fn into(self) -> Point<f32> {
        return Point {
            x: self.x as f32,
            y: self.y as f32,
        };
    }
}

impl Into<Point<i32>> for Point<f32> {
    fn into(self) -> Point<i32> {
        return Point {
            x: self.x as i32,
            y: self.y as i32,
        };
    }
}

impl Into<Point<u32>> for Point<i32> {
    fn into(self) -> Point<u32> {
        return Point {
            x: self.x as u32,
            y: self.y as u32,
        };
    }
}

impl Into<Point<usize>> for Point<i32> {
    fn into(self) -> Point<usize> {
        return Point {
            x: self.x as usize,
            y: self.y as usize,
        };
    }
}

impl Into<Point<usize>> for Point<f32> {
    fn into(self) -> Point<usize> {
        return Point {
            x: self.x as usize,
            y: self.y as usize,
        };
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for Point<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub<T> for Point<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T: BitAnd<Output = T> + Copy> BitAnd<T> for Point<T> {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        Self {
            x: self.x & rhs,
            y: self.y & rhs,
        }
    }
}

impl Point<i32> {
    pub fn clamp_usize(&self, max: Point<usize>) -> Point<usize> {
        return Point {
            x: (self.x.max(0) as usize).min(max.x),
            y: (self.y.max(0) as usize).min(max.y),
        };
    }
}

impl<T: Neg<Output = T>> Neg for Point<T> {
    type Output = Point<T>;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
