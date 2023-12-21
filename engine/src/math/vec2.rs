use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub type Vec2f = Vec2<f32>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Default for Vec2<T>
where
    T: Default,
{
    fn default() -> Self {
        Vec2 {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T> Display for Vec2<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        self.x.fmt(f)?;
        f.write_str(", ")?;
        self.y.fmt(f)?;
        f.write_str(")")
    }
}

impl<T> Vec2<T> {}

impl<T> Add<Vec2<T>> for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign<Vec2<T>> for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub<Vec2<T>> for Vec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign<Vec2<T>> for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<V, T> Mul<T> for Vec2<V>
where
    V: Mul<T, Output = V>,
    T: Copy,
{
    type Output = Vec2<V>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<V: MulAssign<T>, T: Copy> MulAssign<T> for Vec2<V> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
