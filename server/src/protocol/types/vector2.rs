// NOTE: This won't be used for now
//
// NOTE: it turns out rust f32 and godot f32 not really compatible
//       so for now don't deserialize to Vector2 type
//

//! # Vector2: Two dimensional vector of f32 coordinates
//!
//! Magic number: 5
//!
//! | field | offset | length |
//! | x     | 4      | 4      |
//! | y     | 4      | 4      |
const VECTOR2_MAGIC_NUMBER: u32 = 5;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

// Deserialization from Godot's `var_to_bytes` format
impl TryFrom<&[u8]> for Vector2 {
    // TODO: define error in common error module
    type Error = String;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != 12 {
            return Err("given bytes is in wrong length".into());
        }
        if u32::from_le_bytes(data[0..=3].try_into().unwrap()) != VECTOR2_MAGIC_NUMBER {
            return Err("given bytes does not contain valid magic number".into());
        }
        Ok(Vector2 {
            x: f32::from_le_bytes(data[4..=7].try_into().unwrap()),
            y: f32::from_le_bytes(data[8..=11].try_into().unwrap()),
        })
    }
}

impl From<Vector2> for Vec<u8> {
    fn from(v: Vector2) -> Self {
        [
            VECTOR2_MAGIC_NUMBER.to_le_bytes(),
            v.x.to_le_bytes(),
            v.y.to_le_bytes(),
        ]
        .concat()
        .to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        // Vector2(390.9185, 347.6383)
        let payload = [5, 0, 0, 0, 144, 117, 195, 67, 180, 209, 173, 67];
        let v: Vector2 = (&payload[..]).try_into().unwrap();
        assert_eq!(v, Vector2 { x: 390.918, y: 347.638 });
    }
}
