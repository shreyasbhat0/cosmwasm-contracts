use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Task;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub descripton: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    AddTask { name: String, description: String },
    UpdateTask { name: String, description: String },
    DeleteTask { name: String },
    Reset {},
    Completed { name: String, completed: bool },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TaskListResponse)]
    GetTasks {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TaskListResponse {
    pub tasks: Vec<Task>,
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
