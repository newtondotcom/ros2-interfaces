use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulatorFeatures {
    pub features: Vec<u16>,
    pub spawn_formats: Vec<::std::string::String>,
    pub custom_info: ::std::string::String,
}

impl SimulatorFeatures {
    pub const SPAWNING: u8 = 0;
    pub const DELETING: u8 = 1;
    pub const NAMED_POSES: u8 = 2;
    pub const POSE_BOUNDS: u8 = 3;
    pub const ENTITY_TAGS: u8 = 4;
    pub const ENTITY_BOUNDS: u8 = 5;
    pub const ENTITY_BOUNDS_BOX: u8 = 6;
    pub const ENTITY_BOUNDS_CONVEX: u8 = 7;
    pub const ENTITY_CATEGORIES: u8 = 8;
    pub const SPAWNING_RESOURCE_STRING: u8 = 9;
    pub const ENTITY_STATE_GETTING: u8 = 10;
    pub const ENTITY_STATE_SETTING: u8 = 11;
    pub const ENTITY_INFO_GETTING: u8 = 12;
    pub const ENTITY_INFO_SETTING: u8 = 13;
    pub const SPAWNABLES: u8 = 14;
    pub const SIMULATION_RESET: u8 = 20;
    pub const SIMULATION_RESET_TIME: u8 = 21;
    pub const SIMULATION_RESET_STATE: u8 = 22;
    pub const SIMULATION_RESET_SPAWNED: u8 = 23;
    pub const SIMULATION_STATE_GETTING: u8 = 24;
    pub const SIMULATION_STATE_SETTING: u8 = 25;
    pub const SIMULATION_STATE_PAUSE: u8 = 26;
    pub const STEP_SIMULATION_SINGLE: u8 = 31;
    pub const STEP_SIMULATION_MULTIPLE: u8 = 32;
    pub const STEP_SIMULATION_ACTION: u8 = 33;
    pub const WORLD_LOADING: u8 = 40;
    pub const WORLD_RESOURCE_STRING: u8 = 41;
    pub const WORLD_TAGS: u8 = 42;
    pub const WORLD_UNLOADING: u8 = 43;
    pub const WORLD_INFO_GETTING: u8 = 44;
    pub const AVAILABLE_WORLDS: u8 = 45;
}

impl Default for SimulatorFeatures {
    fn default() -> Self {
        SimulatorFeatures {
            features: Vec::new(),
            spawn_formats: Vec::new(),
            custom_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SimulatorFeatures {}
