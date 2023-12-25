//! # Art
//!
//! A library for modeling artistic concepts.
//!
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

/// Returns sum of 2 numbers
///
/// # Examples
///
/// ```
/// let sum = my_crate::sum_of_number(2, 3);
///
/// println!("{}", sum);
/// assert_eq!(sum, 5);
/// ```
pub fn sum_of_number(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
