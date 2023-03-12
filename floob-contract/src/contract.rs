#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, OverflowError, Response, StdError,
    StdResult, Storage,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Thread, ThreadElem, THREADS, THREAD_COUNT, THREAD_ELEM, THREAD_ELEM_COUNT};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:floob-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateThread { title, description } => {
            let id = advance_posts_count(deps.storage)?;
            THREADS.save(
                deps.storage,
                id,
                &Thread {
                    title,
                    description,
                    author: info.sender,
                },
            )?;

            Ok(Response::default()
                .add_attribute("action", "create_thread")
                .add_attribute("id", id.to_string()))
        }
        ExecuteMsg::CreateThreadElem { thread_id, content } => {
            let subthread_id = advance_subthread_count(deps.storage, thread_id)?;
            THREAD_ELEM.save(
                deps.storage,
                (thread_id, subthread_id),
                &ThreadElem {
                    content,
                    author: info.sender,
                },
            )?;
            Ok(Response::default()
                .add_attribute("action", "create_thread_elem")
                .add_attribute("thread_id", thread_id.to_string())
                .add_attribute("subthread_id", subthread_id.to_string()))
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
        QueryMsg::ThreadElem { thread_id, elem_id } => {
            let elem = THREAD_ELEM.load(deps.storage, (thread_id, elem_id))?;
            Ok(to_binary(&elem)?)
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

fn advance_subthread_count(store: &mut dyn Storage, thread_id: u64) -> StdResult<u64> {
    let lhs = THREAD_ELEM_COUNT
        .may_load(store, thread_id)?
        .unwrap_or_default();
    let res = lhs.checked_add(1);
    match res {
        Some(id) => {
            THREAD_ELEM_COUNT.save(store, thread_id, &id)?;
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

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_create_thread() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = ExecuteMsg::CreateThread {
            title: "Hello".to_string(),
            description: "World".to_string(),
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
        assert_eq!("creator", value.author);

        // Create sub-thread
        let msg = ExecuteMsg::CreateThreadElem {
            thread_id: 0,
            content: "Hello World".to_string(),
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        // Check message attributes
        assert_eq!(
            res.attributes,
            vec![
                attr("action", "create_thread_elem"),
                attr("thread_id", "0"),
                attr("subthread_id", "0"),
            ]
        );
        // Check query
        let msg = QueryMsg::ThreadElem {
            thread_id: 0,
            elem_id: 0,
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let value: ThreadElem = from_binary(&res).unwrap();
        assert_eq!("Hello World", value.content);

        // Create second thread element
        let msg = ExecuteMsg::CreateThreadElem {
            thread_id: 0,
            content: "Second thread element. Hope this works".to_string(),
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        // Check message attributes
        assert_eq!(
            res.attributes,
            vec![
                attr("action", "create_thread_elem"),
                attr("thread_id", "0"),
                attr("subthread_id", "1"),
            ]
        );
        // Check query
        let msg = QueryMsg::ThreadElem {
            thread_id: 0,
            elem_id: 1,
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let value: ThreadElem = from_binary(&res).unwrap();
        assert_eq!("Second thread element. Hope this works", value.content);
    }
}
