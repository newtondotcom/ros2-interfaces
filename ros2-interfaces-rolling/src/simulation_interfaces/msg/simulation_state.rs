use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationState {
    pub state: u8,
}

impl SimulationState {
    pub const STATE_STOPPED: u8 = 0;
    pub const STATE_PLAYING: u8 = 1;
    pub const STATE_PAUSED: u8 = 2;
    pub const STATE_QUITTING: u8 = 3;
    pub const STATE_NO_WORLD: u8 = 4;
    pub const STATE_LOADING_WORLD: u8 = 5;
}

impl Default for SimulationState {
    fn default() -> Self {
        SimulationState {
            state: 0,
        }
    }
}

impl ros2_client::Message for SimulationState {}
