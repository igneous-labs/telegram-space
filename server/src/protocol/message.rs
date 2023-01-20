use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::{
    ClientId,
    player_state::PlayerStateData,
};

#[derive(IntoPrimitive, TryFromPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum MessageType {
    Acknowledge = 0,
    PlayerState = 1,
    WorldState = 2,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Message {
    Acknowledge(ClientId) = 0,
    PlayerState(PlayerStateData) = 1,
    WorldState(Vec<(ClientId, PlayerStateData)>) = 2,
}

impl TryFrom<&[u8]> for Message {
    // TODO: define error in common error module
    type Error = String;

    // TODO: refactor unwraps
    fn try_from(data: &[u8]) -> Result<Self, String> {
        if data.len() < 1 {
            return Err("given bytes is in wrong length".into());
        }
        let msg_type: MessageType = data[0].try_into().unwrap();
        let msg = match msg_type {
            MessageType::PlayerState => {
                if data.len() != 15 {
                    return Err("given bytes is in wrong length".into());
                }
                Self::PlayerState(PlayerStateData {
                    position: data[1..=12].try_into().unwrap(),
                    direction: data[13],
                    status: data[14],
                })
            }
            // NOTE: Other messages won't ever be deserialized, will add later if we need them
            _ => {
                return Err("given payload was in a wrong type".to_string());
            }
        };
        Ok(msg)
    }
}

impl From<&Message> for Vec<u8> {
    fn from(msg: &Message) -> Self {
        match msg {
            Message::Acknowledge(client_id) => [
                u8::from(MessageType::Acknowledge).to_le_bytes().to_vec(),
                client_id.to_le_bytes().to_vec(),
            ]
            .concat()
            .to_vec(),
            Message::PlayerState(player_state_data) => [
                u8::from(MessageType::PlayerState).to_le_bytes().to_vec(),
                player_state_data.into(),
            ]
            .concat()
            .to_vec(),
            Message::WorldState(world_state_data) => {
                [u8::from(MessageType::WorldState).to_le_bytes().to_vec()]
                    .concat()
                    .to_vec()
                // // TODO: Now, this is a problem ....
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const MSG: Message = Message::PlayerState(PlayerStateData {
        position: [5, 0, 0, 0, 0, 0, 200, 66, 118, 12, 206, 66],
        direction: 1u8,
        status: 0u8,
    });
    const BIN: [u8; 15] = [1, 5, 0, 0, 0, 0, 0, 200, 66, 118, 12, 206, 66, 1, 0];

    #[test]
    fn it_deserializes() {
        let msg = (&BIN[..]).try_into().unwrap();
        assert!(matches!(msg, Message::PlayerState(_)));
    }

    #[test]
    fn it_serializes() {
        let bin: Vec<u8> = (&MSG).into();
        assert_eq!(bin, BIN)
    }
}
