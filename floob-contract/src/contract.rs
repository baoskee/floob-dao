#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, OverflowError, Response, StdError,
    StdResult, Storage,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Thread, ADMIN, THREADS, THREAD_COUNT};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:floob-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let addr = deps.api.addr_validate(&msg.admin)?;
    ADMIN.save(deps.storage, &addr)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }
    match msg {
        ExecuteMsg::CreateThread {
            title,
            description,
            content,
        } => {
            let id = advance_posts_count(deps.storage)?;
            THREADS.save(
                deps.storage,
                id,
                &Thread {
                    title,
                    description,
                    content,
                },
            )?;

            Ok(Response::default()
                .add_attribute("action", "create_thread")
                .add_attribute("id", id.to_string()))
        }
    }
}

/**
 * TODO(1): cw-paginate might be a good idea here
 */
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Thread { id } => {
            let post = THREADS.load(deps.storage, id)?;
            Ok(to_binary(&post)?)
        }
    }
}

/// MARK: Helpers

fn advance_posts_count(store: &mut dyn Storage) -> StdResult<u64> {
    let lhs = THREAD_COUNT.may_load(store)?.unwrap_or_default();
    let res = lhs.checked_add(1);
    match res {
        Some(id) => {
            THREAD_COUNT.save(store, &id)?;
            Ok(lhs)
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
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, coins, from_binary};

    #[test]
    fn test_proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            admin: "admin".to_string(),
        };
        let info = mock_info("admin", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_create_thread() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            admin: "creator".to_string(),
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = ExecuteMsg::CreateThread {
            title: "Hello".to_string(),
            description: "World".to_string(),
            content: vec!["Hello World".to_string()],
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        // Check message attributes
        assert_eq!(
            res.attributes,
            vec![attr("action", "create_thread"), attr("id", "0"),]
        );

        let msg = QueryMsg::Thread { id: 0 };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let value: Thread = from_binary(&res).unwrap();
        assert_eq!("Hello", value.title);
        assert_eq!("World", value.description); 
        assert_eq!(vec!["Hello World".to_string()], value.content);
    }
}
