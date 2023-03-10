use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::{
    errors::ProtocolErrors,
    player_state::PlayerStateData,
    types::{Array, PackedByteArray},
    world_state::WorldStateEntry,
    ClientId, InstanceId, LevelId,
};

#[derive(IntoPrimitive, TryFromPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum IngressMessageType {
    PlayerState = 1,
    RequestLevel = 3,
    PlayerInstance = 5,
    PlayerChatUserId = 7,
}

#[derive(Debug)]
#[repr(u8)]
pub enum IngressMessage {
    PlayerState(PlayerStateData),
    RequestLevel(LevelId),
    PlayerInstance(InstanceId),
    PlayerChatUserId(Vec<u8>),
}

#[derive(IntoPrimitive, TryFromPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum EgressMessageType {
    Acknowledge = 0,
    WorldState = 2,
    LevelData = 4,
    PlayerInstanceAcknowledge = 6,
    PlayerChatUserIdAcknowledge = 8,
}

#[derive(Debug)]
#[repr(u8)]
pub enum EgressMessage {
    Acknowledge(ClientId),
    WorldState(Vec<WorldStateEntry>, Vec<(ClientId, Vec<u8>)>),
    LevelData(LevelId, Vec<u8>),
    PlayerInstanceAcknowledge(InstanceId),
    PlayerChatUserIdAcknowledge,
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
            IngressMessageType::RequestLevel => {
                if data.len() != 9 {
                    return Err(ProtocolErrors::DeserializationError(
                        "given bytes is in wrong length".into(),
                    ));
                }
                Self::RequestLevel(u64::from_le_bytes(data[1..=8].try_into().unwrap()))
            }
            IngressMessageType::PlayerInstance => {
                if data.len() != 9 {
                    return Err(ProtocolErrors::DeserializationError(
                        "given bytes is in wrong length".into(),
                    ));
                }
                Self::PlayerInstance(u64::from_le_bytes(data[1..=8].try_into().unwrap()))
            }
            IngressMessageType::PlayerChatUserId => {
                // variable size data
                Self::PlayerChatUserId(data[1..].try_into().unwrap())
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
            EgressMessage::WorldState(world_state_data, instance_chat_user_ids) => {
                let world_state_data: Vec<u8> = PackedByteArray(
                    Array(world_state_data.iter().map(PackedByteArray::from).collect()).into(),
                )
                .into();
                let instance_chat_user_id: Vec<u8> = PackedByteArray(
                    Array(
                        instance_chat_user_ids
                            .iter()
                            .map(|(client_id, chat_user_id)| {
                                let inner: Vec<u8> = PackedByteArray(
                                    [client_id.to_le_bytes().to_vec(), chat_user_id.to_owned()]
                                        .concat()
                                        .to_vec(),
                                )
                                .into();
                                inner
                            })
                            .collect(),
                    )
                    .into(),
                )
                .into();
                [
                    u8::from(EgressMessageType::WorldState)
                        .to_le_bytes()
                        .to_vec(),
                    Array(vec![world_state_data, instance_chat_user_id]).into(),
                ]
                .concat()
                .to_vec()
            }
            EgressMessage::LevelData(level_id, compressed_level_data) => [
                (u8::from(EgressMessageType::LevelData))
                    .to_le_bytes()
                    .to_vec(),
                level_id.to_le_bytes().to_vec(),
                compressed_level_data.to_vec(),
            ]
            .concat()
            .to_vec(),
            EgressMessage::PlayerInstanceAcknowledge(instance_id) => [
                (u8::from(EgressMessageType::PlayerInstanceAcknowledge))
                    .to_le_bytes()
                    .to_vec(),
                instance_id.to_le_bytes().to_vec(),
            ]
            .concat()
            .to_vec(),
            EgressMessage::PlayerChatUserIdAcknowledge => {
                [(u8::from(EgressMessageType::PlayerChatUserIdAcknowledge))
                    .to_le_bytes()
                    .to_vec()]
                .concat()
                .to_vec()
            }
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
