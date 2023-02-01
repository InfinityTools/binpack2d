//! `binpack2d` is a library for performing two-dimensional rectangle bin-packing,
//! based on Jukka Jylänki's C++ implementation of RectangleBinPack.

pub use crate::binpack::BinPacker;
pub use crate::binpack::BinType;
pub use crate::binpack::BinError;
pub use crate::binpack::bin_new;
pub use crate::binpack::bin_with_capacity;
pub use crate::binpack::pack_bins;

pub use crate::binpack::guillotine;
pub use crate::binpack::maxrects;

pub use crate::dimension::Dimension;
pub use crate::rectangle::Rectangle;

pub mod dimension;
pub mod rectangle;
pub mod binpack;
