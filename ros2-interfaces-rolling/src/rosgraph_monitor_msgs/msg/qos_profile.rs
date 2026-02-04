use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QosProfile {
    pub depth: u32,
    pub deadline: crate::builtin_interfaces::msg::Duration,
    pub lifespan: crate::builtin_interfaces::msg::Duration,
    pub history: u8,
    pub reliability: u8,
    pub durability: u8,
    pub liveliness: u8,
    pub liveliness_lease_duration: crate::builtin_interfaces::msg::Duration,
}

impl QosProfile {
    pub const HISTORY_SYSTEM_DEFAULT: u8 = 0;
    pub const HISTORY_KEEP_LAST: u8 = 1;
    pub const HISTORY_KEEP_ALL: u8 = 2;
    pub const HISTORY_UNKNOWN: u8 = 3;
    pub const RELIABILITY_SYSTEM_DEFAULT: u8 = 0;
    pub const RELIABILITY_RELIABLE: u8 = 1;
    pub const RELIABILITY_BEST_EFFORT: u8 = 2;
    pub const RELIABILITY_UNKNOWN: u8 = 3;
    pub const DURABILITY_SYSTEM_DEFAULT: u8 = 0;
    pub const DURABILITY_TRANSIENT_LOCAL: u8 = 1;
    pub const DURABILITY_VOLATILE: u8 = 2;
    pub const DURABILITY_UNKNOWN: u8 = 3;
    pub const LIVELINESS_SYSTEM_DEFAULT: u8 = 0;
    pub const LIVELINESS_AUTOMATIC: u8 = 1;
    pub const LIVELINESS_MANUAL_BY_TOPIC: u8 = 3;
    pub const LIVELINESS_UNKNOWN: u8 = 4;
}

impl Default for QosProfile {
    fn default() -> Self {
        QosProfile {
            depth: 0,
            deadline: crate::builtin_interfaces::msg::Duration::default(),
            lifespan: crate::builtin_interfaces::msg::Duration::default(),
            history: 0,
            reliability: 0,
            durability: 0,
            liveliness: 0,
            liveliness_lease_duration: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for QosProfile {}
