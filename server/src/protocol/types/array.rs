//! # Array of PackedArray
//!
//! We won't be covering the entirety of the Godot's Binary Serialization API for Array.
//! Instead, this implementation of Array type will assume the given array is array of PackedByteArray.
//!
//! This is used to sync the world state which contains an aray PackedByteArray of length n where
//! n is the number of clients currently connected to the server, and each element of type
//! PackedByteArray is a tuple of client_id and serialized PlayerState.
//!
//! Since this type will only be used to send data from server to clients, only serialization procedure
//! is implemented.
//!
//! Magic number: 28
//!
//! TODO: Write description
const ARRAY_MAGIC_NUMBER: u32 = 28;
const ARRAY_PADDING: usize = 4;

#[derive(Debug)]
pub struct Array<T>(pub Vec<T>)
where
    T: Into<Vec<u8>>;

impl<T> From<Array<T>> for Vec<u8>
where
    T: Into<Vec<u8>>,
{
    fn from(array: Array<T>) -> Self {
        let len: u32 = array
            .0
            .len()
            .try_into()
            .expect("array size exceeded u32 max");
        let mut res = Vec::new();
        res.extend(ARRAY_MAGIC_NUMBER.to_le_bytes());
        res.extend(len.to_le_bytes());
        for element in array.0 {
            let mut bytes: Vec<u8> = element.into();
            let padding = (ARRAY_PADDING - (bytes.len() % ARRAY_PADDING)) % ARRAY_PADDING;
            bytes.resize(bytes.len() + padding, 0u8);
            res.extend::<Vec<_>>(bytes);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::{super::PackedByteArray, *};

    // [
    //  28, 0, 0, 0, // This is an arry
    //  3, 0, 0, 0,  // of size 3
    //      29, 0, 0, 0, // This is a packed byte array
    //      15, 0, 0, 0, // of size 15 (padded to 4 bytes, so 16 bytes per element)
    //          1, 5, 0, 0, 0, 0, 0, 200, 66, 118, 12, 206, 66, 1, 0, 0, // data
    //      29, 0, 0, 0,
    //      15, 0, 0, 0,
    //          1, 5, 0, 0, 0, 0, 0, 200, 66, 118, 12, 206, 66, 1, 0, 0,
    //      29, 0, 0, 0,
    //      15, 0, 0, 0,
    //          1, 5, 0, 0, 0, 0, 0, 200, 66, 118, 12, 206, 66, 1, 0, 0
    //  ]
    #[test]
    fn it_serializes_array_of_packed_byte_array() {
        let arr = Array(vec![
            PackedByteArray(vec![1, 5, 0, 0, 0, 0, 0, 200, 66, 118, 12, 206, 66, 1, 0]),
            PackedByteArray(vec![1, 5, 0, 0, 0, 0, 0, 200, 66, 118, 12, 206, 66, 1, 0]),
            PackedByteArray(vec![1, 5, 0, 0, 0, 0, 0, 200, 66, 118, 12, 206, 66, 1, 0]),
        ]);
        let res: Vec<_> = arr.into();
        assert_eq!(
            vec![
                28, 0, 0, 0, 3, 0, 0, 0, 29, 0, 0, 0, 15, 0, 0, 0, 1, 5, 0, 0, 0, 0, 0, 200, 66,
                118, 12, 206, 66, 1, 0, 0, 29, 0, 0, 0, 15, 0, 0, 0, 1, 5, 0, 0, 0, 0, 0, 200, 66,
                118, 12, 206, 66, 1, 0, 0, 29, 0, 0, 0, 15, 0, 0, 0, 1, 5, 0, 0, 0, 0, 0, 200, 66,
                118, 12, 206, 66, 1, 0, 0
            ],
            res
        );
    }

    #[test]
    fn it_serializes_array_of_client_chat_user_id() {
        let client_id = 1u16;
        let chat_user_id: Vec<u8> = vec![1, 2, 3, 4, 5];
        let elements: Vec<Vec<u8>> = vec![[client_id.to_le_bytes().to_vec(), chat_user_id]
            .concat()
            .to_vec()];
        let arr = Array(elements);
        let res: Vec<u8> = arr.into();
        assert_eq!(vec![28, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 2, 3, 4, 5, 0], res);
    }
}
