//! # PackedByteArray
//!
//! Array of u8 bytes.
//! Similar to Array type, 4 byte magic number, 4 byte length followed by the content bytes.
//!
//! Magic number: 29
const PACKED_BYTE_ARRAY_MAGIC_NUMBER: u32 = 29;

#[derive(Debug)]
pub struct PackedByteArray(pub Vec<u8>);

impl From<PackedByteArray> for Vec<u8> {
    fn from(array: PackedByteArray) -> Self {
        let len: u32 = array.0.len().try_into().expect("too big");
        [
            PACKED_BYTE_ARRAY_MAGIC_NUMBER.to_le_bytes().to_vec(),
            len.to_le_bytes().to_vec(),
            array.0,
        ]
        .concat()
        .to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_serializes_packed_byte_array() {
        let arr = PackedByteArray(vec![1, 5, 0, 0, 0, 0, 0, 200, 66, 118, 12, 206, 66, 1, 0]);
        let res: Vec<_> = arr.into();
        assert_eq!(
            vec![29, 0, 0, 0, 15, 0, 0, 0, 1, 5, 0, 0, 0, 0, 0, 200, 66, 118, 12, 206, 66, 1, 0],
            res
        );
    }
}
