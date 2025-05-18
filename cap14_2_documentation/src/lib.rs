//! # Art
//!
//! The library for drawning and modeling artistic concepts.

pub use self::color::PrimaryColor;
pub use self::color::SecondColor;
pub use self::draw::mix;

pub mod color {
    pub enum PrimaryColor {
        Blue,
        Red,
        Green,
    }

    pub enum SecondColor {
        Yellow,
        Pink,
        Brown,
    }
}

pub mod draw {
    use crate::color::*;

    pub fn mix(color1: PrimaryColor, color2: SecondColor) {}
}

#[cfg(test)]
mod tests {
    use super::*;
}
