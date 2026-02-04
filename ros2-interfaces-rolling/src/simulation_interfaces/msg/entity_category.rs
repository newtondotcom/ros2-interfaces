use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityCategory {
    pub category: u8,
}

impl EntityCategory {
    pub const CATEGORY_OBJECT: u8 = 0;
    pub const CATEGORY_ROBOT: u8 = 1;
    pub const CATEGORY_HUMAN: u8 = 2;
    pub const CATEGORY_DYNAMIC_OBJECT: u8 = 4;
    pub const CATEGORY_STATIC_OBJECT: u8 = 5;
}

impl Default for EntityCategory {
    fn default() -> Self {
        EntityCategory {
            category: 0,
        }
    }
}

impl ros2_client::Message for EntityCategory {}
