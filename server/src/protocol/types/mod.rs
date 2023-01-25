//! This module defines types used by godot.
//! # Godot Binary Serialization API
//! The packet is designed to be always padded to 4 bytes.
//! All values are little-endian-encoded. All packets have a 4-byte
//! header representing an integer, specifying the type of data.
//!
//! NOTE: Fucking Binary Serialization API documentation is outdated,
//!       and the magic numbers are fucking wrong. For example, when serialized,
//!       the magic number for Array type is 28, but on doc 28 is PackedColorArray,
//!       and the magic number for PackedColorArray is 37. Had to read the type
//!       definition enum in godot engine code in core/variant/type_info.h to get the
//!       correct magic number.
//!       Thanks Obama
//!
//! references:
//!  - https://docs.godotengine.org/en/latest/classes/class_%40globalscope.html#class-globalscope-method-var-to-bytes
//!  - https://docs.godotengine.org/en/latest/tutorials/io/binary_serialization_api.html

mod array;
mod packed_byte_array;
//mod vector2;

pub use array::*;
pub use packed_byte_array::*;
//pub use vector2::*;
