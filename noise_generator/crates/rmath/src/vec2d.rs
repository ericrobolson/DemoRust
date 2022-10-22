use core::ops::*;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec2d<N> {
    pub x: N,
    pub y: N,
}
impl<N> Vec2d<N>
where
    N: Mul<Output = N> + Copy + Add<Output = N>,
{
    /// Returns the dot product of two vectors
    pub fn dot(&self, rhs: Self) -> N {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn length(&self) -> N {
        let x2 = self.x * self.x;
        let y2 = self.y * self.y;

        todo!()
    }
}

impl<N> From<(N, N)> for Vec2d<N> {
    fn from((x, y): (N, N)) -> Self {
        Self { x, y }
    }
}
impl<N> From<[N; 2]> for Vec2d<N> {
    fn from([x, y]: [N; 2]) -> Self {
        Self { x, y }
    }
}

impl<N> Add for Vec2d<N>
where
    N: Add<Output = N>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<N> AddAssign for Vec2d<N>
where
    N: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<N> Div<N> for Vec2d<N>
where
    N: Div<Output = N> + Copy,
{
    type Output = Self;

    fn div(self, rhs: N) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<N> DivAssign<N> for Vec2d<N>
where
    N: DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: N) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<N> Div for Vec2d<N>
where
    N: Div<Output = N>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<N> DivAssign for Vec2d<N>
where
    N: DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<N> Mul<N> for Vec2d<N>
where
    N: Mul<Output = N> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<N> MulAssign<N> for Vec2d<N>
where
    N: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: N) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<N> Mul for Vec2d<N>
where
    N: Mul<Output = N>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<N> MulAssign for Vec2d<N>
where
    N: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<N> Sub for Vec2d<N>
where
    N: Sub<Output = N>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<N> SubAssign for Vec2d<N>
where
    N: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot() {
        let a: Vec2d<i32> = (10, 10).into();
        let b: Vec2d<i32> = (20, 30).into();

        let expected = a.x * b.x + a.y * b.y;

        let result = a.dot(b);
        assert_eq!(expected, result)
    }

    #[test]
    fn add() {
        let a: Vec2d<i32> = (10, 10).into();
        let b = (20, 30).into();

        let expected: Vec2d<i32> = (30, 40).into();

        let result = a + b;
        assert_eq!(expected, result)
    }

    #[test]
    fn add_assign() {
        let mut a: Vec2d<i32> = (10, 10).into();
        let b = (20, 30).into();

        let expected: Vec2d<i32> = (30, 40).into();

        a += b;
        assert_eq!(expected, a)
    }

    #[test]
    fn div() {
        let a: Vec2d<i32> = (200, 100).into();
        let b: Vec2d<i32> = (4, 5).into();

        let expected: Vec2d<i32> = (50, 20).into();

        let result = a / b;
        assert_eq!(expected, result)
    }

    #[test]
    fn div_assign() {
        let mut a: Vec2d<i32> = (10, 10).into();
        let b: Vec2d<i32> = (2, 5).into();

        let expected: Vec2d<i32> = (5, 2).into();

        a /= b;
        assert_eq!(expected, a)
    }

    #[test]
    fn div_n() {
        let a: Vec2d<i32> = (20, 30).into();

        let expected: Vec2d<i32> = (4, 6).into();

        let result = a / 5;
        assert_eq!(expected, result)
    }

    #[test]
    fn div_assign_n() {
        let mut a: Vec2d<i32> = (20, 30).into();

        let expected: Vec2d<i32> = (2, 3).into();

        a /= 10;
        assert_eq!(expected, a)
    }

    #[test]
    fn from_array() {
        let [x, y] = [10, 8];

        let expected = Vec2d { x, y };
        let result: Vec2d<i32> = [x, y].into();
        assert_eq!(expected, result)
    }

    #[test]
    fn from_tuple() {
        let (x, y) = (10, 8);

        let expected = Vec2d { x, y };
        let result: Vec2d<i32> = (x, y).into();
        assert_eq!(expected, result)
    }

    #[test]
    fn mul() {
        let a: Vec2d<i32> = (10, 10).into();
        let b: Vec2d<i32> = (20, 30).into();

        let expected: Vec2d<i32> = (200, 300).into();

        let result = a * b;
        assert_eq!(expected, result)
    }

    #[test]
    fn mul_assign() {
        let mut a: Vec2d<i32> = (10, 10).into();
        let b: Vec2d<i32> = (20, 30).into();

        let expected: Vec2d<i32> = (200, 300).into();

        a *= b;
        assert_eq!(expected, a)
    }

    #[test]
    fn mul_n() {
        let a: Vec2d<i32> = (20, 30).into();

        let expected: Vec2d<i32> = (200, 300).into();

        let result = a * 10;
        assert_eq!(expected, result)
    }

    #[test]
    fn mul_assign_n() {
        let mut a: Vec2d<i32> = (20, 30).into();

        let expected: Vec2d<i32> = (200, 300).into();

        a *= 10;
        assert_eq!(expected, a)
    }

    #[test]
    fn sub() {
        let a: Vec2d<i32> = (10, 10).into();
        let b = (20, 30).into();

        let expected: Vec2d<i32> = (-10, -20).into();

        let result = a - b;
        assert_eq!(expected, result)
    }

    #[test]
    fn sub_assign() {
        let mut a: Vec2d<i32> = (10, 10).into();
        let b = (20, 30).into();

        let expected: Vec2d<i32> = (-10, -20).into();

        a -= b;
        assert_eq!(expected, a)
    }
}
