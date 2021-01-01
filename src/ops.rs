use {super::f8, std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign}};

impl Add for f8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<&Self> for f8 {
    type Output = <f8 as Add>::Output;

    fn add(self, rhs: &Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<'a> Add<f8> for &'a f8 {
    type Output = <f8 as Add>::Output;

    fn add(self, rhs: f8) -> Self::Output {
        f8(self.0 + rhs.0)
    }
}

impl<'a> Add<&'a f8> for &'a f8 {
    type Output = <f8 as Add>::Output;

    fn add(self, rhs: Self) -> Self::Output {
        f8(self.0 + rhs.0)
    }
}

impl AddAssign for f8 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0);
    }
}

impl AddAssign<&Self> for f8 {
    fn add_assign(&mut self, rhs: &Self) {
        *self = Self(self.0 + rhs.0);
    }
}

impl Div for f8 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Div<&Self> for f8 {
    type Output = <f8 as Div>::Output;

    fn div(self, rhs: &Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl<'a> Div<f8> for &'a f8 {
    type Output = <f8 as Div>::Output;

    fn div(self, rhs: f8) -> Self::Output {
        f8(self.0 / rhs.0)
    }
}

impl<'a> Div<&'a f8> for &'a f8 {
    type Output = <f8 as Div>::Output;

    fn div(self, rhs: Self) -> Self::Output {
        f8(self.0 / rhs.0)
    }
}

impl DivAssign for f8 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self(self.0 / rhs.0);
    }
}

impl DivAssign<&Self> for f8 {
    fn div_assign(&mut self, rhs: &Self) {
        *self = Self(self.0 / rhs.0);
    }
}

impl Mul for f8 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<&Self> for f8 {
    type Output = <f8 as Mul>::Output;

    fn mul(self, rhs: &Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<'a> Mul<f8> for &'a f8 {
    type Output = <f8 as Mul>::Output;

    fn mul(self, rhs: f8) -> Self::Output {
        f8(self.0 * rhs.0)
    }
}

impl<'a> Mul<&'a f8> for &'a f8 {
    type Output = <f8 as Mul>::Output;

    fn mul(self, rhs: Self) -> Self::Output {
        f8(self.0 * rhs.0)
    }
}

impl MulAssign for f8 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self(self.0 * rhs.0);
    }
}

impl MulAssign<&Self> for f8 {
    fn mul_assign(&mut self, rhs: &Self) {
        *self = Self(self.0 * rhs.0);
    }
}

impl Sub for f8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub<&Self> for f8 {
    type Output = <f8 as Sub>::Output;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<'a> Sub<f8> for &'a f8 {
    type Output = <f8 as Sub>::Output;

    fn sub(self, rhs: f8) -> Self::Output {
        f8(self.0 - rhs.0)
    }
}

impl<'a> Sub<&'a f8> for &'a f8 {
    type Output = <f8 as Sub>::Output;

    fn sub(self, rhs: Self) -> Self::Output {
        f8(self.0 - rhs.0)
    }
}

impl SubAssign for f8 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0);
    }
}

impl SubAssign<&Self> for f8 {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = Self(self.0 - rhs.0);
    }
}
