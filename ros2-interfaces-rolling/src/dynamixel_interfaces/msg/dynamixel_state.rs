use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamixelState {
    pub header: crate::std_msgs::msg::Header,
    pub comm_state: i32,
    pub id: Vec<i32>,
    pub torque_state: Vec<bool>,
    pub dxl_hw_state: Vec<i32>,
}

impl DynamixelState {
    pub const COMM_STATE_OK: i32 = 0;
    pub const COMM_STATE_CANNOT_FIND_CONTROL_ITEM: i32 = -1;
    pub const COMM_STATE_OPEN_PORT_FAIL: i32 = -2;
    pub const COMM_STATE_INDIRECT_ADDR_FAIL: i32 = -3;
    pub const COMM_STATE_ITEM_WRITE_FAIL: i32 = -4;
    pub const COMM_STATE_ITEM_READ_FAIL: i32 = -5;
    pub const COMM_STATE_SYNC_WRITE_FAIL: i32 = -6;
    pub const COMM_STATE_SYNC_READ_FAIL: i32 = -7;
    pub const COMM_STATE_SET_SYNC_WRITE_FAIL: i32 = -8;
    pub const COMM_STATE_SET_SYNC_READ_FAIL: i32 = -9;
    pub const COMM_STATE_BULK_WRITE_FAIL: i32 = -10;
    pub const COMM_STATE_BULK_READ_FAIL: i32 = -11;
    pub const COMM_STATE_SET_BULK_WRITE_FAIL: i32 = -12;
    pub const COMM_STATE_SET_BULK_READ_FAIL: i32 = -13;
    pub const COMM_STATE_SET_READ_ITEM_FAIL: i32 = -14;
    pub const COMM_STATE_SET_WRITE_ITEM_FAIL: i32 = -15;
    pub const COMM_STATE_DXL_HARDWARE_ERROR: i32 = -16;
    pub const COMM_STATE_DXL_REBOOT_FAIL: i32 = -17;
}

impl Default for DynamixelState {
    fn default() -> Self {
        DynamixelState {
            header: crate::std_msgs::msg::Header::default(),
            comm_state: 0,
            id: Vec::new(),
            torque_state: Vec::new(),
            dxl_hw_state: Vec::new(),
        }
    }
}

impl ros2_client::Message for DynamixelState {}
