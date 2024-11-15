use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMap2Req {
    pub global_map: bool,
    pub optimized: bool,
    pub with_images: bool,
    pub with_scans: bool,
    pub with_user_data: bool,
    pub with_grids: bool,
    pub with_words: bool,
    pub with_global_descriptors: bool,
}

impl Default for GetMap2Req {
    fn default() -> Self {
        GetMap2Req {
            global_map: false,
            optimized: false,
            with_images: false,
            with_scans: false,
            with_user_data: false,
            with_grids: false,
            with_words: false,
            with_global_descriptors: false,
        }
    }
}

impl ros2_client::Message for GetMap2Req {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMap2Res {
    pub data: crate::rtabmap_msgs::msg::MapData,
}

impl Default for GetMap2Res {
    fn default() -> Self {
        GetMap2Res {
            data: crate::rtabmap_msgs::msg::MapData::default(),
        }
    }
}

impl ros2_client::Message for GetMap2Res {}


pub struct GetMap2;
impl ros2_client::Service for GetMap2 {
    type Request = GetMap2Req;
    type Response = GetMap2Res;

    fn request_type_name(&self) -> &str { "GetMap2Req" }
    fn response_type_name(&self) -> &str { "GetMap2Res" }
}
