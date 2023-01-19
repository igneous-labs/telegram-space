use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(IntoPrimitive, TryFromPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum MessageType {
    Acknowledge = 0,
    PlayerState = 1,
    WorldState = 2,
}
