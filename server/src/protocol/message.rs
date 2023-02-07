use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::{
    errors::ProtocolErrors, player_state::PlayerStateData, world_state::WorldStateEntry, ClientId,
};

// TODO: consider moving types module into protocol
use super::types::{Array, PackedByteArray};

#[derive(IntoPrimitive, TryFromPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum IngressMessageType {
    PlayerState = 1,
}

#[derive(Debug)]
#[repr(u8)]
pub enum IngressMessage {
    PlayerState(PlayerStateData),
}

#[derive(IntoPrimitive, TryFromPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum EgressMessageType {
    Acknowledge = 0,
    WorldState = 2,
    LevelData = 3,
}

#[derive(Debug)]
#[repr(u8)]
pub enum EgressMessage {
    Acknowledge(ClientId),
    WorldState(Vec<WorldStateEntry>),
    LevelData,
}

// Deserialization
impl TryFrom<&[u8]> for IngressMessage {
    type Error = ProtocolErrors;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < 1 {
            return Err(ProtocolErrors::DeserializationError(
                "given bytes is in wrong length".into(),
            ));
        }
        // TODO: map error to DeserializationError
        let msg_type: IngressMessageType = data[0].try_into().map_err(|_| {
            ProtocolErrors::DeserializationError("wrong ingress message type".into())
        })?;
        let msg = match msg_type {
            IngressMessageType::PlayerState => {
                if data.len() != 15 {
                    return Err(ProtocolErrors::DeserializationError(
                        "given bytes is in wrong length".into(),
                    ));
                }
                Self::PlayerState(PlayerStateData {
                    position: data[1..=12].try_into().unwrap(),
                    direction: data[13],
                    status: data[14],
                })
            }
        };
        Ok(msg)
    }
}

// Serialization
impl From<&EgressMessage> for Vec<u8> {
    fn from(msg: &EgressMessage) -> Self {
        match msg {
            EgressMessage::Acknowledge(client_id) => [
                u8::from(EgressMessageType::Acknowledge)
                    .to_le_bytes()
                    .to_vec(),
                client_id.to_le_bytes().to_vec(),
            ]
            .concat()
            .to_vec(),
            EgressMessage::WorldState(world_state_data) => [
                u8::from(EgressMessageType::WorldState)
                    .to_le_bytes()
                    .to_vec(),
                Array(world_state_data.iter().map(PackedByteArray::from).collect()).into(),
            ]
            .concat()
            .to_vec(),
            EgressMessage::LevelData => [
                (u8::from(EgressMessageType::LevelData))
                    .to_le_bytes()
                    .to_vec(),
            ]
            .concat()
            .to_vec(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const MSG: IngressMessage = IngressMessage::PlayerState(PlayerStateData {
        position: [5, 0, 0, 0, 0, 0, 200, 66, 118, 12, 206, 66],
        direction: 1u8,
        status: 0u8,
    });
    const BIN: [u8; 15] = [1, 5, 0, 0, 0, 0, 0, 200, 66, 118, 12, 206, 66, 1, 0];

    #[test]
    fn it_deserializes() {
        let msg = (&BIN[..]).try_into().unwrap();
        assert!(matches!(msg, IngressMessage::PlayerState(_)));
        // TODO: check inner data
    }
}
