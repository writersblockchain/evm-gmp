use crate::errors::CustomContractError;
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,CosmosMsg, StdError, Uint128
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, Fee};

use ethabi::{encode, decode, ParamType,Token};
use prost::Message;
use serde_json_wasm::to_string;

use crate::msg::GmpMessage;

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
        ExecuteMsg::ReceiveMessageEvm {
            source_chain,
            source_address,
            payload,
        } => receive_message_evm(
            deps, 
            env,
            info,
            source_chain, 
            source_address, 
            payload),
    }
}

pub fn send_message_evm(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination_chain: String,
    destination_address: String,
    message: String,
) -> Result<Response, CustomContractError> {
    // Message payload to be received by the destination
     let message_payload = encode(&vec![
         Token::String(message),
     ]);

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
            amount: coin.amount.clone().to_string(), // Make sure to handle amounts accurately
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
        source_port: "transfer".to_string(),
        source_channel: "channel-20".to_string(), // Testnet Osmosis to axelarnet: https://docs.axelar.dev/resources/testnet#ibc-channels
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
}

pub fn receive_message_evm(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    source_chain: String,
    source_address: String,
    payload: Binary,
) -> Result<Response, ContractError> {
    // decode the payload
    // executeMsgPayload: [sender, message]
    let decoded = decode(
        &vec![ParamType::String, ParamType::String],
        payload.as_slice(),
    );

    execute(deps,env,info,decoded[1].to_string())
    
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStoredMessage {  } => todo!(),
   }
}
