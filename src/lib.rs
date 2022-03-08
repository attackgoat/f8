//! A no-standard crate which provides an f8 type.

#![no_std]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

const RECIP: f32 = 1.0 / ((u8::MAX) as f32 + 1.0);

/// An 8-bit floating point number constrained to a value within the inclusive range of [0, 1].
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct f8(u8);

impl f8 {
    /// Returns this `f8` as a `u8`.
    #[inline]
    pub fn byte(self) -> u8 {
        self.0
    }

    /// Returns this `f8` as a `f32`.
    #[inline]
    pub fn float(self) -> f32 {
        self.into()
    }
}

impl From<u8> for f8 {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val)
    }
}

impl From<f32> for f8 {
    #[inline]
    fn from(val: f32) -> Self {
        Self((val / RECIP) as _)
    }
}

impl From<f8> for u8 {
    #[inline]
    fn from(val: f8) -> Self {
        val.0
    }
}

impl From<f8> for f32 {
    #[inline]
    fn from(val: f8) -> Self {
        val.0 as f32 / u8::MAX as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_conversions() {
        assert_eq![0.0, f8::from(0).float()];
        assert_eq![1.0, f8::from(255).float()];

        assert_eq![0, f8::from(0.0).byte()];
        assert_eq![1, f8::from(RECIP).byte()];
        assert_eq![255, f8::from(1.0).byte()];
    }
}
