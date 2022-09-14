#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, ContractInfoResponse, Deps, DepsMut, Empty, Env, MessageInfo, Querier,
    QuerierWrapper, Reply, ReplyOn, Response, StdError, StdResult, SubMsg, WasmMsg, WasmQuery,
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
    // if not factory  check the caller is a contract from the code_id
    // and return
    if !msg.factory {
        let req = WasmQuery::ContractInfo {
            contract_addr: info.sender.into(),
        }
        .into();
        let res: ContractInfoResponse = deps.querier.query(&req)?;
        if res.code_id != msg.code_id {
            let err_msg = format!(
                "contract is not from allowed code id {}-{}",
                res.code_id, msg.code_id
            );
            return Err(StdError::generic_err(err_msg).into());
        }
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
        reply_on: ReplyOn::Success,
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

pub fn query_contract_info<Q: Querier, T: Into<String>>(
    querier: &Q,
    contract_addr: T,
) -> StdResult<ContractInfoResponse> {
    let req = WasmQuery::ContractInfo {
        contract_addr: contract_addr.into(),
    }
    .into();
    QuerierWrapper::<Empty>::new(querier).query(&req)
}
