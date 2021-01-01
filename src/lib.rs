#![no_std]

#[cfg(feature="serde")]
#[macro_use]
extern crate serde;

#[allow(non_camel_case_types)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct f8(u8);

impl From<u8> for f8 {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

impl From<f32> for f8 {
    fn from(val: f32) -> Self {
        Self((val / 255.0) as _)
    }
}
