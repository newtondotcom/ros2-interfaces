use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PanoramaImg {
    pub header: crate::std_msgs::msg::Header,
    pub pano_id: ::std::string::String,
    pub latitude: f64,
    pub longitude: f64,
    pub heading: f64,
    pub geo_tag: ::std::string::String,
    pub image: crate::sensor_msgs::msg::Image,
}

impl Default for PanoramaImg {
    fn default() -> Self {
        PanoramaImg {
            header: crate::std_msgs::msg::Header::default(),
            pano_id: ::std::string::String::new(),
            latitude: 0.0,
            longitude: 0.0,
            heading: 0.0,
            geo_tag: ::std::string::String::new(),
            image: crate::sensor_msgs::msg::Image::default(),
        }
    }
}

impl ros2_client::Message for PanoramaImg {}
