use std::convert::TryFrom;
use std::ops::{Index, IndexMut};

/// Type of color in Ethereum blockies
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
#[repr(u8)]
pub enum ColorClass {
    /// Color 1
    BgColor = 0,
    /// Color 2
    Color = 1,
    /// Color 3
    SpotColor = 2,
}

/// Array for [`ColorClass`], which can be accessed like a map
///
/// # Example
/// ```
/// use eth_blockies::*;
///
/// let colorclass_map: ColorClassArrayMap<&str> = ["first_elem", "second_elem", "third_elem"];
///
/// assert_eq!(colorclass_map[ColorClass::BgColor], "first_elem");
/// assert_eq!(colorclass_map[ColorClass::Color], "second_elem");
/// assert_eq!(colorclass_map[ColorClass::SpotColor], "third_elem");
/// ```
pub type ColorClassArrayMap<T> = [T; 3];

impl TryFrom<u8> for ColorClass {
    type Error = ();

    fn try_from(raw_value: u8) -> Result<Self, Self::Error> {
        match raw_value {
            0..=2 => Ok(unsafe { std::mem::transmute::<_, ColorClass>(raw_value) }),
            _ => Err(()),
        }
    }
}

impl<T> Index<ColorClass> for ColorClassArrayMap<T> {
    type Output = T;
    fn index(&self, colorclass: ColorClass) -> &Self::Output {
        unsafe { self.get_unchecked(colorclass as usize) }
    }
}
impl<T> IndexMut<ColorClass> for ColorClassArrayMap<T> {
    fn index_mut(&mut self, colorclass: ColorClass) -> &mut Self::Output {
        unsafe { self.get_unchecked_mut(colorclass as usize) }
    }
}

impl<T> Index<&ColorClass> for ColorClassArrayMap<T> {
    type Output = T;
    fn index(&self, colorclass: &ColorClass) -> &Self::Output {
        unsafe { self.get_unchecked(*colorclass as usize) }
    }
}
impl<T> IndexMut<&ColorClass> for ColorClassArrayMap<T> {
    fn index_mut(&mut self, colorclass: &ColorClass) -> &mut Self::Output {
        unsafe { self.get_unchecked_mut(*colorclass as usize) }
    }
}
