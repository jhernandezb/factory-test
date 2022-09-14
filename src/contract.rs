#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, ReplyOn, Response, StdError,
    StdResult, SubMsg, WasmMsg,
};
use cw_utils::parse_reply_instantiate_data;

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

const CONTRACT_NAME: &str = "crates.io:sg-factory-test";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // if not factory just return
    if !msg.factory {
        return Ok(Response::default());
    }
    let submsg = SubMsg {
        msg: WasmMsg::Instantiate {
            code_id: msg.code_id,
            msg: to_binary(&InstantiateMsg {
                factory: false,
                code_id: msg.code_id,
            })?,
            funds: info.funds,
            admin: Some(info.sender.to_string()),
            label: "contract".to_string(),
        }
        .into(),
        id: REPLY_ID,
        gas_limit: None,
        reply_on: ReplyOn::Always,
    };
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_submessage(submsg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    return Ok(Response::default());
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    return Ok(Binary::default());
}

// Reply callback triggered from contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id != REPLY_ID {
        return Err(StdError::generic_err("invalid reply id").into());
    }
    let reply = parse_reply_instantiate_data(msg);
    match reply {
        Ok(res) => Ok(Response::default()
            .add_attribute("action", "instantiate_sg721_reply")
            .add_attribute("address", res.contract_address)),

        Err(_) => Err(StdError::generic_err("error instantianting child contract").into()),
    }
}

#[cfg(test)]
mod tests {}
