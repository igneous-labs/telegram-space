pub type LevelId = u64;
pub type DecompressedSize = u32;

#[derive(Debug, Clone)]
pub struct CompressedLevelData {
    pub decompressed_size: DecompressedSize,
    pub data: Vec<u8>,
}

impl From<&CompressedLevelData> for Vec<u8> {
    fn from(level: &CompressedLevelData) -> Self {
        [
            level.decompressed_size.to_le_bytes().to_vec(),
            level.data.clone(),
        ]
        .concat()
        .to_vec()
    }
}
