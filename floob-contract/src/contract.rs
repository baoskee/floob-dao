#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, OverflowError, Response, StdError,
    StdResult, Storage,
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Post, POSTS, POST_COUNT};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:floob-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePost { title, description } => {
            let id = advance_posts_count(deps.storage)?;
            POSTS.save(deps.storage, id, &Post { title, description })?;

            Ok(Response::default())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Post { id } => {
            let post = POSTS.load(deps.storage, id)?;
            Ok(to_binary(&post)?)
        }
    }
}

/// MARK: Helpers

fn advance_posts_count(store: &mut dyn Storage) -> StdResult<u64> {
    let lhs = POST_COUNT.may_load(store)?.unwrap_or_default();
    let res = lhs.checked_add(1);
    match res {
        Some(id) => {
            POST_COUNT.save(store, &id)?;
            Ok(id)
        }
        None => Err(StdError::Overflow {
            source: OverflowError {
                operation: cosmwasm_std::OverflowOperation::Add,
                operand1: lhs.to_string(),
                operand2: 1.to_string(),
            },
        }),
    }
}

#[cfg(test)]
mod tests {}
