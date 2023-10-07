use crate::errors::CustomContractError;
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::executes::{join_game, new_game, submit_choice};
use crate::queries::{query_game_state, query_who_won};

use ethabi::{decode, encode, ParamType, Token};
use prost::Message;
use serde_json_wasm::to_string;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, CustomContractError> {
    match msg {
        ExecuteMsg::SendMessageEvm {
            destination_chain,
            destination_address,
            message,
        } => send_message_evm(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
        ),
    }
}

pub fn send_message_evm(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination_chain: String,
    destination_address: String,
    message: String,
) -> Result<Response, ContractError> {
    // Message payload to be received by the destination
     let message_payload = encode(&vec![
         Token::String(message),
     ]);

    // // {info.funds} used to pay gas. Must only contain 1 token type.
    // // let coin: cosmwasm_std::Coin = cw_utils::one_coin(&info).unwrap();

     let coin = &info.funds[0];

     let my_coin = crate::ibc::Coin {
         denom: coin.denom.clone(),
         amount: coin.amount.clone().to_string(),
     };

    let gmp_message: GmpMessage = GmpMessage {
        destination_chain,
        destination_address,
        payload: message_payload.to_vec(),
        type_: 1,
        fee: Some(Fee {
            amount: coin.clone().to_string(), // Make sure to handle amounts accurately
            recipient: "axelar1aythygn6z5thymj6tmzfwekzh05ewg3l7d6y89".to_string(),
        }),
    };

    // // let send_msg = to_binary(&TransferMsg {
    // //     channel: "channel-3".to_string(),
    // //     remote_address: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5"
    // //         .to_string(),
    // //     timeout: 10 * 60,
    // //     memo: gmp_str,
    // // })?;
    // // let msg = secret_toolkit::snip20::send_msg_with_code_hash(
    // //     "secret1yxjmepvyl2c25vnt53cr2dpn8amknwausxee83".to_string(),
    // //     Some("2976a2577999168b89021ecb2e09c121737696f71c4342f9a922ce8654e98662".to_string()),
    // //     Uint128::new(0),
    // //     Some(send_msg), // msg goes here
    // //     None,
    // //     None,
    // //     1,
    // //     "638a3e1d50175fbcb8373cf801565283e3eb23d88a9b7b7f99fcc5eb1e6b561e".to_string(),
    // //     "secret1vcau4rkn7mvfwl8hf0dqa9p0jr59983e3qqe3z".to_string(), /* snip20 contract originates from axelar*/
    // // );

    let memo = to_string(&gmp_message).map_err(|e| {
        StdError::generic_err(format!("error generating Memo: {:?}", e))
    })?;

    let ibc_message = crate::ibc::MsgTransfer {
        source_port: "wasm.secret1vfht4c30h4st7e254ww86p6whwyy0uux2ns5ck".to_string(),
        source_channel: "channel-3".to_string(), // Testnet Osmosis to axelarnet: https://docs.axelar.dev/resources/testnet#ibc-channels
        token: Some(my_coin.into()),
        sender: env.contract.address.to_string(),
        receiver: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5"
            .to_string(),
        timeout_height: None,
        timeout_timestamp: env.block.time.plus_seconds(604_800u64).nanos(),
        memo: memo,
    };

    let cosmos_msg = cosmwasm_std::CosmosMsg::Stargate {
        type_url: "/ibc.applications.transfer.v1.MsgTransfer".to_string(),
        value: Binary(ibc_message.encode_to_vec()),
    };

    Ok(Response::new().add_message(cosmos_msg))
    // // Ok(
    // //     Response::new().add_message(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
    // //         contract_addr: (),
    // //         code_hash: (),
    // //         msg: (),
    // //         funds: (),
    // //     })),
    // // )
    //Ok(Response::new())
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::WhoWon { game } => to_binary(&query_who_won(deps, env, game)?),
        QueryMsg::GameState { game } => to_binary(&query_game_state(deps, env, game)?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::msg::GameStateResponse;
    use crate::state::{GameResult, GameStatus, RPS};
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{coins, OwnedDeps};

    fn _check_current_status(
        deps: &OwnedDeps<MockStorage, MockApi, MockQuerier>,
        env: Env,
        game_id: &String,
        expected: GameStatus,
    ) -> GameStateResponse {
        let value = query_game_state(deps.as_ref(), env, game_id.clone());

        if value.is_err() {
            panic!("Game not found in storage");
        }

        let unwrapped = value.unwrap();

        assert_eq!(&unwrapped.state, &expected);
        unwrapped
    }

    fn instantiate_contract(deps: DepsMut) -> MessageInfo {
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps, mock_env(), info.clone(), msg).unwrap();
        info
    }

    #[test]
    fn new_game() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let info = instantiate_contract(deps.as_mut());

        let msg_player1 = ExecuteMsg::NewGame {
            player_name: "alice".to_string(),
            bet: None,
        };

        // test new game returns a valid game ID
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg_player1).unwrap();
        assert_eq!(&res.events[0].attributes[0].value, "QTEBERJH");

        let game_id = res.events[0].attributes[0].value.clone();

        // it worked, let's query the state and check that we're waiting for the 2nd player to join
        let unwrapped = _check_current_status(&deps, env, &game_id, GameStatus::WaitingForPlayerToJoin);
        assert_eq!(unwrapped.game, game_id);
    }

    #[test]
    fn full_game() {
        let mut deps = mock_dependencies();
        let mut env = mock_env();

        let info = instantiate_contract(deps.as_mut());

        let msg_player1 = ExecuteMsg::NewGame {
            bet: None,
            player_name: "alice".to_string(),
        };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg_player1).unwrap();

        assert_eq!(&res.events[0].attributes[0].value, "QTEBERJH");

        let game_id = res.events[0].attributes[0].value.clone();

        let msg_player2 = ExecuteMsg::JoinGame {
            player_name: "bob".to_string(),
            game_code: game_id.clone(),
        };

        let info2 = mock_info("bob", &coins(2, "token"));
        let _res = execute(deps.as_mut(), env.clone(), info2.clone(), msg_player2).unwrap();

        let _ = _check_current_status(&deps, env.clone(), &game_id, GameStatus::Started);

        let msg_action_p1 = ExecuteMsg::SubmitChoice {
            game_code: game_id.clone(),
            choice: RPS::Rock,
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg_action_p1).unwrap();

        let _ = _check_current_status(&deps, env.clone(), &game_id, GameStatus::Got1stChoiceWaitingFor2nd);

        let msg_action_p2 = ExecuteMsg::SubmitChoice {
            game_code: game_id.clone(),
            choice: RPS::Paper,
        };
        let _res = execute(deps.as_mut(), env.clone(), info2.clone(), msg_action_p2).unwrap();

        let _ = _check_current_status(&deps, env.clone(), &game_id, GameStatus::WaitingForWinner);

        env.block.height += 1;

        let winner = query_who_won(deps.as_ref(), env, game_id);

        if winner.is_err() {
            panic!("Winner not available");
        }

        let unwrapped = winner.unwrap();

        assert_eq!(unwrapped.winner, GameResult::Player2);
        assert_eq!(
            unwrapped.address,
            Some(cosmwasm_std::Addr::unchecked("bob"))
        );
    }
}
