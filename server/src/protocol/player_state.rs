#[derive(Copy, Clone, Debug)]
pub struct PlayerStateData {
    pub position: [u8; 12],
    pub direction: u8,
    pub status: u8,
}

impl From<&PlayerStateData> for Vec<u8> {
    fn from(data: &PlayerStateData) -> Self {
        [
            data.position.to_vec(),
            data.direction.to_le_bytes().to_vec(),
            data.status.to_le_bytes().to_vec(),
        ]
        .concat()
        .to_vec()
    }
}
