use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridMapReq {
    pub frame_id: ::std::string::String,
    pub position_x: f64,
    pub position_y: f64,
    pub length_x: f64,
    pub length_y: f64,
    pub layers: Vec<::std::string::String>,
}

impl Default for GetGridMapReq {
    fn default() -> Self {
        GetGridMapReq {
            frame_id: ::std::string::String::new(),
            position_x: 0.0,
            position_y: 0.0,
            length_x: 0.0,
            length_y: 0.0,
            layers: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetGridMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridMapRes {
    pub map: crate::grid_map_msgs::msg::GridMap,
}

impl Default for GetGridMapRes {
    fn default() -> Self {
        GetGridMapRes {
            map: crate::grid_map_msgs::msg::GridMap::default(),
        }
    }
}

impl ros2_client::Message for GetGridMapRes {}


pub struct GetGridMap;
impl ros2_client::Service for GetGridMap {
    type Request = GetGridMapReq;
    type Response = GetGridMapRes;

    fn request_type_name(&self) -> &str { "GetGridMapReq" }
    fn response_type_name(&self) -> &str { "GetGridMapRes" }
}
