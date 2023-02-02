//! A structure representing the dimension of an object to be placed in a bin.
//!
//! The `Dimension` structure is primarily used to request placement of an object in a bin.

use std::fmt::{Display, Formatter};
use std::mem;
use std::sync::atomic::{AtomicIsize, Ordering};

/// Internally used to provide a unique identifier for each instance
static UNIQUE_ID: AtomicIsize = AtomicIsize::new(1);

/// A thread-safe function that returns a new identifier value for each call.
pub(crate) fn get_unique_id() -> isize {
    UNIQUE_ID.fetch_add(1, Ordering::Relaxed)
}

/// The `Dimension` struct stores information about width, height and optional padding of an object.
///
/// Padding can be used to ensure that rectangles are placed into bins with spacing to each other.
///
/// An identifier is provided for custom identification purposes.
///
/// [`id`]: Dimension::id
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dimension {
    id: isize,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) padding: i32,
}

impl Dimension {
    /// Creates a new `Dimension` object with the specified dimensions, an auto-generated
    /// identifier and no padding.
    ///
    /// Negative values for width and height are capped at 0.
    ///
    /// **Note:** Auto-generated identifiers can be considered unique, as long as they are not
    /// duplicated by [`with_id`], [`set_id`] or cloning.
    ///
    /// [`with_id`]: Self::with_id
    /// [`set_id`]: Self::set_id
    pub fn new(width: i32, height: i32) -> Self {
        Self::with_id(get_unique_id(), width, height, 0)
    }

    /// Creates a new `Dimension` object with the specified dimensions, an auto-generated
    /// identifier and optional padding.
    ///
    /// Negative values for width, height and padding are capped at 0.
    ///
    /// **Note:** Auto-generated identifiers can be considered unique, as long as they are not
    /// duplicated by [`with_id`], [`set_id`] or cloning.
    ///
    /// [`with_id`]: Self::with_id
    /// [`set_id`]: Self::set_id
    pub fn with_padding(width: i32, height: i32, padding: i32) -> Self {
        Self::with_id(get_unique_id(), width, height, padding)
    }

    /// Creates a new `Dimension` object with the specified dimension, identifier and
    /// optional padding.
    ///
    /// Negative values for width, height and padding are capped at 0.
    pub fn with_id(id: isize, width: i32, height: i32, padding: i32) -> Self {
        Self {
            id,
            width: width.max(0),
            height: height.max(0),
            padding: padding.max(0),
        }
    }

    /// Returns the identifier of the `Dimension`.
    pub fn id(&self) -> isize {
        self.id
    }

    /// Returns width of the `Dimension`.
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Returns width of the `Dimension` with padding.
    pub(crate) fn width_total(&self) -> i32 {
        self.width + (self.padding << 1)
    }

    /// Returns height of the `Dimension`.
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Returns height of the `Dimension` with padding.
    pub(crate) fn height_total(&self) -> i32 {
        self.height + (self.padding << 1)
    }

    /// Returns the padding of the `Dimension`.
    pub fn padding(&self) -> i32 {
        self.padding
    }

    /// Assigns a new identifier to `Dimension`.
    pub fn set_id(&mut self, value: isize) {
        self.id = value;
    }

    /// Sets width of the `Dimension` to the specified `value`.
    ///
    /// A negative value for width is capped at 0.
    pub fn set_width(&mut self, value: i32) {
        self.width = value.max(0);
    }

    /// Sets height of the `Dimension` to the specified `value`.
    ///
    /// A negative value for height is capped at 0.
    pub fn set_height(&mut self, value: i32) {
        self.height = value.max(0);
    }

    /// Sets both `width` and `height` of the `Dimension`.
    ///
    /// Negative values for width and height are capped at 0.
    pub fn set_dimension(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    /// Sets padding of the `Dimension` to the specified `value`.
    ///
    /// A negative value for padding is capped at 0.
    pub fn set_padding(&mut self, value: i32) {
        self.padding = value.max(0);
    }

    /// Flips the `Dimension` by 90 degrees.
    pub fn flip(&mut self) {
        mem::swap(&mut self.width, &mut self.height);
    }

    /// Creates a new `Dimension` from the current instance, which is flipped by 90 degrees.
    pub fn to_flipped(&self) -> Self {
        Self::with_id(self.id, self.height, self.width, self.padding)
    }

    /// Returns `true` if `width` or `height` of the `Dimension` is 0, and `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Returns `true` if `width` or `height` of the `Dimension` is 0, and `false` otherwise.
    ///
    /// Padding is included in the check.
    pub(crate) fn is_empty_total(&self) -> bool {
        (self.width + self.padding) == 0 || (self.height + self.padding) == 0
    }

    /// Computes the area of this `Dimension`.
    pub fn area(&self) -> i64 {
        self.width as i64 * self.height as i64
    }

    /// Computes the area of this `Dimension`.
    ///
    /// Padding is included in the calculation.
    pub(crate) fn area_total(&self) -> i64 {
        self.width_total() as i64 * self.height_total() as i64
    }
}

impl Default for Dimension {
    fn default() -> Self {
        Dimension::new(0, 0)
    }
}

impl Display for Dimension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dimension(id: {}, width: {}, height: {}, padding: {})",
               self.id, self.width, self.height, self.padding)
    }
}


#[cfg(test)]
mod tests;
