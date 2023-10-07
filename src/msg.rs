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

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    WhoWon { game: String },
    GameState { game: String },
}

/// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CheckWinner {
    pub winner: GameResult,
    pub address: Option<Addr>,
}

/// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GameStateResponse {
    pub game: String,
    pub state: crate::state::GameStatus,
}
