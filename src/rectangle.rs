//! A structure that represents the placement of a single object in a bin.

use std::fmt::{Display, Formatter};
use super::dimension::{self, Dimension};

/// `Rectangle` specifies an area in a coordinate space that is defined an upper-left point,
/// as defined by `x` and `y`, and the dimensions, defined by the [`Dimension`] object.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rectangle {
    x: i32,
    y: i32,
    dim: Dimension,
}

impl Rectangle {
    /// Creates a new `Rect` whose upper-left corner is defined by `x` and `y`, and whose `width`
    /// and `height` are defined by the [`Dimension`] type.
    pub fn new(x: i32, y: i32, dim: Dimension) -> Rectangle {
        Rectangle { x, y, dim }
    }

    /// Returns the x coordinate of the bounding `Rectangle`.
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Returns the x coordinate of the bounding `Rectangle`, including padding.
    pub(crate) fn x_total(&self) -> i32 {
        self.x - self.dim.padding()
    }

    /// Returns the y coordinate of the bounding `Rectangle`.
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Returns the y coordinate of the bounding `Rectangle`, including padding.
    pub(crate) fn y_total(&self) -> i32 {
        self.y - self.dim.padding()
    }

    /// Moves this `Rectangle` horizontally to the location specified by x.
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    /// Moves this `Rectangle` horizontally to the location specified by x.
    ///
    /// Includes padding in the calculation.
    pub(crate) fn set_x_total(&mut self, x: i32) {
        self.x = x + self.dim.padding();
    }

    /// Moves this `Rectangle` vertically to the location specified by y.
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    /// Moves this `Rectangle` vertically to the location specified by y.
    ///
    /// Includes padding in the calculation.
    pub(crate) fn set_y_total(&mut self, y: i32) {
        self.y = y + self.dim.padding();
    }

    /// Moves this `Rectangle` to the location specified by x and y.
    pub fn set_location(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// Moves this `Rectangle` to the location specified by x and y.
    ///
    /// Includes padding in the calculation.
    pub(crate) fn set_location_total(&mut self, x: i32, y: i32) {
        self.x = x + self.dim.padding();
        self.y = y + self.dim.padding();
    }

    /// Translates this `Rectangle` the indicated distance, to the right along the X axis,
    /// and downward along the Y axis.
    ///
    /// **Note:** Underflow and overflow are bound by [`i32::MIN`] and [`i32::MAX`] respectively.
    pub fn translate(&mut self, dx: i32, dy: i32) {
        if dx != 0 {
            self.x = self.x.saturating_add(dx);
        }
        if dy != 0 {
            self.y = self.y.saturating_add(dy);
        }
    }

    /// Returns the identifier associated with the `Rectangle`.
    pub fn id(&self) -> isize {
        self.dim.id()
    }

    /// Returns the width of the bounding `Rectangle` without padding.
    pub fn width(&self) -> i32 {
        self.dim.width()
    }

    /// Returns the width of the bounding `Rectangle` with padding.
    pub(crate) fn width_total(&self) -> i32 {
        self.dim.width_total()
    }

    /// Returns the height of the bounding `Rectangle` without padding.
    pub fn height(&self) -> i32 {
        self.dim.height()
    }

    /// Returns the height of the bounding `Rectangle` with padding.
    pub(crate) fn height_total(&self) -> i32 {
        self.dim.height_total()
    }

    /// Returns an immutable reference to the associated [`Dimension`] object.
    pub fn dim(&self) -> &Dimension {
        &self.dim
    }

    /// Returns a mutable reference to the associated [`Dimension`] object.
    pub fn dim_mut(&mut self) -> &mut Dimension {
        &mut self.dim
    }

    /// Returns `true` if `width` or `height` of the `Rectangle` is 0, and `false` otherwise.
    ///
    /// Padding is not included in the check.
    pub fn is_empty(&self) -> bool {
        self.dim.is_empty()
    }

    /// Checks whether or not this `Rectangle` entirely contains the specified `Rectangle`.
    ///
    /// Padding is not included in the check.
    pub fn contains(&self, rect: &Rectangle) -> bool {
        rect.x >= self.x
            && rect.y >= self.y
            && rect.x + rect.width() <= self.x + self.width()
            && rect.y + rect.height() <= self.y + self.height()
    }

    /// Checks whether or not this `Rectangle` entirely contains the specified `Rectangle`.
    ///
    /// Padding is included in the check.
    pub(crate) fn contains_total(&self, rect: &Rectangle) -> bool {
        rect.x_total() >= self.x_total()
            && rect.y_total() >= self.y_total()
            && rect.x_total() + rect.width_total() <= self.x_total() + self.width_total()
            && rect.y_total() + rect.height_total() <= self.y_total() + self.height_total()
    }

    /// Checks whether or not this `Rectangle` and the specified `Rectangle` intersect.
    ///
    /// Padding is not included in the check.
    pub fn intersects(&self, rect: &Rectangle) -> bool {
        let mut tw = self.width();
        let mut th = self.height();
        let mut rw = rect.width();
        let mut rh = rect.height();
        if rw == 0 || rh == 0 || tw == 0 || th == 0 {
            return false;
        }

        let tx = self.x;
        let ty = self.y;
        let rx = rect.x;
        let ry = rect.y;
        rw += rx;
        rh += ry;
        tw += tx;
        th += ty;
        // overflow || intersect
        (rw < rx || rw > tx) && (rh < ry || rh > ty) && (tw < tx || tw > rx) && (th < ty || th > ry)
    }

    /// Computes the union of this `Rectangle` with the specified `Rectangle`.
    ///
    /// `rect` specifies the second rectangle to use for the union.
    ///
    /// `id` will be used as new identifier for the dimension included the returned `Rectangle`.
    /// An identifier is autogenerated if if `None` is specified.
    ///
    /// The greater padding of the source `Rectangle`s is applied to the returned `Rectangle`.
    ///
    /// Returns a new `Rectangle` that represents the union of the two rectangles.
    pub fn union(&self, rect: &Rectangle, id: Option<isize>) -> Self {
        let min_x = self.x.min(rect.x);
        let min_y = self.y.min(rect.y);

        let max_x = (self.x + self.width()).max(rect.x + rect.width());
        let width = max_x - min_x;

        let max_y = (self.y + self.height()).max(rect.y + rect.height());
        let height = max_y - min_y;

        let id = id.unwrap_or_else(dimension::get_unique_id);

        let padding = self.dim.padding().max(rect.dim.padding());

        Self {
            x: min_x,
            y: min_y,
            dim: Dimension::with_id(id, width, height, padding),
        }
    }
}

impl From<Dimension> for Rectangle {
    fn from(value: Dimension) -> Self {
        Rectangle::new(0, 0, value)
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle(x: {}, y: {}, dim: {})", self.x, self.y, self.dim)
    }
}


#[cfg(test)]
mod tests;
