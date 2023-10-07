use crate::state::{GameResult, RPS};
use cosmwasm_std::{CosmosMsg, StdError, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SendMessageEvm {
        destination_chain: String,
        destination_address: String,
        message: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetStoredMessage {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct GetStoredMessageResp {
    pub sender: String,
    pub message: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Fee {
    pub amount: String,
    pub recipient: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct GmpMessage {
    pub destination_chain: String,
    pub destination_address: String,
    pub payload: Vec<u8>,
    #[serde(rename = "type")]
    pub type_: i64,
    pub fee: Option<Fee>,
}

