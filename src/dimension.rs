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

/// The `Dimension` struct stores information about `width` and `height` of an object.
///
/// An identifier is provided for custom identification purposes.
///
/// [`id`]: Dimension::id
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dimension {
    id: isize,
    width: u32,
    height: u32,
}

impl Dimension {
    /// Creates a new `Dimension` object with the specified dimensions and an auto-generated
    /// unique identifier.
    ///
    /// **Note:** Uniqueness of the identifier is only ensured if this method is exclusively used
    /// to create new `Dimension` objects.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            id: get_unique_id(),
            width,
            height,
        }
    }

    /// Creates a new `Dimension` object with the specified identifier and dimension.
    pub fn with_id(id: isize, width: u32, height: u32) -> Self {
        Self { id, width, height }
    }

    /// Returns the identifier of the `Dimension`.
    pub fn id(&self) -> isize {
        self.id
    }

    /// Returns width of the `Dimension`.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns height of the `Dimension`.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Assigns a new identifier to `Dimension`.
    pub fn set_id(&mut self, value: isize) {
        self.id = value;
    }

    /// Sets width of the `Dimension` to the specified `value`.
    pub fn set_width(&mut self, value: u32) {
        self.width = value;
    }

    /// Sets height of the `Dimension` to the specified `value`.
    pub fn set_height(&mut self, value: u32) {
        self.height = value;
    }

    /// Sets both `width` and `height` of the `Dimension`.
    pub fn set_dimension(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Creates a new `Dimension` from the current instance, which is flipped by 90 degrees.
    pub fn to_flipped(&self) -> Self {
        Self::with_id(self.id, self.height, self.width)
    }

    /// Flips the `Dimension` by 90 degrees.
    pub fn flip(&mut self) {
        mem::swap(&mut self.width, &mut self.height);
    }

    /// Returns `true` if `width` or `height` of the `Dimension` is 0, and `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Calculates the area of this `Dimension`.
    pub fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }
}

impl Default for Dimension {
    fn default() -> Self {
        Dimension::new(0, 0)
    }
}

impl Display for Dimension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dimension(id: {}, width: {}, height: {})", self.id, self.width, self.height)
    }
}


#[cfg(test)]
mod tests;
