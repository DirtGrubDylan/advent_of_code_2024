use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

use super::location::Location;
use super::math;

#[derive(Debug, Default, Eq, PartialEq, PartialOrd, Hash, Copy, Clone, Ord)]
pub struct Point2d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Neg<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> Point2d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Neg<Output = T>
        + Rem<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Point2d { x, y }
    }

    #[must_use]
    pub fn reduce(&self) -> Self {
        let gcd = math::gcd(self.x, self.y);

        Point2d::new(self.x / gcd, self.y / gcd)
    }
}

impl<T> Add<Point2d<T>> for Point2d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Neg<Output = T>
        + Rem<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point2d::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign<Point2d<T>> for Point2d<T>
where
    T: Add<Output = T>
        + AddAssign<T>
        + Sub<Output = T>
        + SubAssign<T>
        + Mul<Output = T>
        + MulAssign<T>
        + Div<Output = T>
        + DivAssign<T>
        + Neg<Output = T>
        + Rem<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub<Point2d<T>> for Point2d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Neg<Output = T>
        + Rem<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2d::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> SubAssign<Point2d<T>> for Point2d<T>
where
    T: Add<Output = T>
        + AddAssign<T>
        + Sub<Output = T>
        + SubAssign<T>
        + Mul<Output = T>
        + MulAssign<T>
        + Div<Output = T>
        + DivAssign<T>
        + Neg<Output = T>
        + Rem<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Mul<Point2d<T>> for Point2d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Neg<Output = T>
        + Rem<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Point2d::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> Mul<T> for Point2d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Neg<Output = T>
        + Rem<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Point2d::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> MulAssign<Point2d<T>> for Point2d<T>
where
    T: Add<Output = T>
        + AddAssign<T>
        + Sub<Output = T>
        + SubAssign<T>
        + Mul<Output = T>
        + MulAssign<T>
        + Div<Output = T>
        + DivAssign<T>
        + Neg<Output = T>
        + Rem<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> MulAssign<T> for Point2d<T>
where
    T: Add<Output = T>
        + AddAssign<T>
        + Sub<Output = T>
        + SubAssign<T>
        + Mul<Output = T>
        + MulAssign<T>
        + Div<Output = T>
        + DivAssign<T>
        + Neg<Output = T>
        + Rem<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> Div<Point2d<T>> for Point2d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Neg<Output = T>
        + Rem<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Point2d::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T> DivAssign<Point2d<T>> for Point2d<T>
where
    T: Add<Output = T>
        + AddAssign<T>
        + Sub<Output = T>
        + SubAssign<T>
        + Mul<Output = T>
        + MulAssign<T>
        + Div<Output = T>
        + DivAssign<T>
        + Neg<Output = T>
        + Rem<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T> Neg for Point2d<T>
where
    T: Add<Output = T>
        + AddAssign<T>
        + Sub<Output = T>
        + SubAssign<T>
        + Mul<Output = T>
        + MulAssign<T>
        + Div<Output = T>
        + DivAssign<T>
        + Neg<Output = T>
        + Rem<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point2d::new(-self.x, -self.y)
    }
}

impl<T> From<(T, T)> for Point2d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Neg<Output = T>
        + Rem<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    fn from((x, y): (T, T)) -> Point2d<T> {
        Point2d { x, y }
    }
}

impl<T> Location for Point2d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Neg<Output = T>
        + Rem<Output = T>
        + Ord
        + From<u8>
        + Copy,
{
    type ValueOutput = T;

    fn manhattan_distance_to(&self, other: &Point2d<T>) -> T {
        let relative_x = other.x - self.x;
        let relative_y = other.y - self.y;

        relative_x + relative_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    const ORIGIN_POINT: Point2d<i32> = Point2d { x: 0, y: 0 };

    #[test]
    fn test_from() {
        let expected = Point2d::from((5, 5));

        let result = Point2d { x: 5, y: 5 };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_manhattan_distance_to() {
        let point = Point2d::new(5, 5);

        let expected = 10;

        let result = ORIGIN_POINT.manhattan_distance_to(&point);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_hash() {
        let mut map = HashMap::new();

        let first = Point2d::new(3, 4);
        let second = Point2d::new(3, 4);

        map.entry(first).or_insert(1);

        map.entry(second).and_modify(|x| *x += 1).or_insert(-666);

        assert_eq!(*map.get(&second).unwrap(), 2);
    }

    #[test]
    fn test_reduce() {
        let point_1 = Point2d::new(-3, 6);
        let point_2 = Point2d::new(3, -5);

        let expected_1 = Point2d::new(-1, 2);
        let expected_2 = Point2d::new(3, -5);

        let result_1 = point_1.reduce();
        let result_2 = point_2.reduce();

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }
}
