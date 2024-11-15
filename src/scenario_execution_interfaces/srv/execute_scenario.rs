use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteScenarioReq {
    pub scenario: crate::scenario_execution_interfaces::msg::Scenario,
}

impl Default for ExecuteScenarioReq {
    fn default() -> Self {
        ExecuteScenarioReq {
            scenario: crate::scenario_execution_interfaces::msg::Scenario::default(),
        }
    }
}

impl ros2_client::Message for ExecuteScenarioReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteScenarioRes {
    pub result: bool,
}

impl Default for ExecuteScenarioRes {
    fn default() -> Self {
        ExecuteScenarioRes {
            result: false,
        }
    }
}

impl ros2_client::Message for ExecuteScenarioRes {}


pub struct ExecuteScenario;
impl ros2_client::Service for ExecuteScenario {
    type Request = ExecuteScenarioReq;
    type Response = ExecuteScenarioRes;

    fn request_type_name(&self) -> &str { "ExecuteScenarioReq" }
    fn response_type_name(&self) -> &str { "ExecuteScenarioRes" }
}
