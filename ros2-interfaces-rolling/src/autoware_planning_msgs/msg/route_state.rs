use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteState {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub state: u8,
}

impl RouteState {
    pub const UNKNOWN: u8 = 0;
    pub const INITIALIZING: u8 = 1;
    pub const UNSET: u8 = 2;
    pub const ROUTING: u8 = 3;
    pub const SET: u8 = 4;
    pub const REROUTING: u8 = 5;
    pub const ARRIVED: u8 = 6;
    pub const ABORTED: u8 = 7;
    pub const INTERRUPTED: u8 = 8;
}

impl Default for RouteState {
    fn default() -> Self {
        RouteState {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            state: 0,
        }
    }
}

impl ros2_client::Message for RouteState {}
