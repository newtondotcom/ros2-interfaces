use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub sync_1: u8,
    pub sync_2: u8,
    pub crc: u16,
    pub id: u16,
    pub revision: u8,
    pub length: u16,
    pub tow: u32,
    pub wnc: u16,
}

impl Default for BlockHeader {
    fn default() -> Self {
        BlockHeader {
            sync_1: 0,
            sync_2: 0,
            crc: 0,
            id: 0,
            revision: 0,
            length: 0,
            tow: 0,
            wnc: 0,
        }
    }
}

impl ros2_client::Message for BlockHeader {}
