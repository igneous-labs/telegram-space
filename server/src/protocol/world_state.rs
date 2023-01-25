use super::{player_state::PlayerStateData, types::PackedByteArray, ClientId};

#[derive(Debug)]
pub struct WorldStateEntry {
    pub client_id: ClientId,
    pub player_state_data: PlayerStateData,
}

impl From<&WorldStateEntry> for Vec<u8> {
    fn from(data: &WorldStateEntry) -> Self {
        [
            data.client_id.to_le_bytes().to_vec(),
            (&data.player_state_data).into(),
        ]
        .concat()
        .to_vec()
    }
}

impl From<&WorldStateEntry> for PackedByteArray {
    fn from(data: &WorldStateEntry) -> Self {
        PackedByteArray(data.into())
    }
}

// TODO: test
