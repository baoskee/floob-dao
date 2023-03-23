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
        ExecuteMsg::EditThread {
            id,
            title,
            description,
            content,
        } => {
            let count = THREAD_COUNT.may_load(deps.storage)?.unwrap_or_default();
            // Watch overflow here
            if id + 1 > count {
                return Err(ContractError::ThreadNotFound {});
            }
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
                .add_attribute("action", "edit_thread")
                .add_attribute("id", id.to_string()))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetThread { id } => {
            let post = THREADS.load(deps.storage, id)?;
            Ok(to_binary(&post)?)
        },
        QueryMsg::GetThreadsCreated { start, end } => {
            let count = THREAD_COUNT.may_load(deps.storage)?.unwrap_or_default();
            let start = start.unwrap_or(0);
            let end = end.unwrap_or(count);
            let mut threads = vec![];
            for i in start..end {
                threads.push(THREADS.load(deps.storage, i)?);
            }
            Ok(to_binary(&threads)?)
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

        let msg = QueryMsg::GetThread { id: 0 };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let value: Thread = from_binary(&res).unwrap();
        assert_eq!("Hello", value.title);
        assert_eq!("World", value.description);
        assert_eq!(vec!["Hello World".to_string()], value.content);
    }

    #[test]
    fn test_edit_thread() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            admin: "creator".to_string(),
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Test edit fails on non-existent thread
        let msg = ExecuteMsg::EditThread {
            id: 0,
            title: "Hello".to_string(),
            description: "World".to_string(),
            content: vec!["Hello World".to_string()],
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Err(ContractError::ThreadNotFound {}) => {}
            _ => panic!("Expected ThreadNotFound error"),
        }
        // Create new thread
        let msg = ExecuteMsg::CreateThread {
            title: "Hello".to_string(),
            description: "World".to_string(),

            // Content is a vector of long paragraphs
            content: vec!["Hello World".to_string()],
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        // Edit thread to new long paragraphs
        let msg = ExecuteMsg::EditThread {
            id: 0,
            title: "Hello".to_string(),
            description: "World".to_string(),
            content: vec![
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec dictum aliquam eros, eu molestie sapien dignissim eget. Nullam tincidunt orci in dolor fermentum suscipit. Sed non lectus non massa lobortis blandit. Sed eget purus quam. Nam euismod arcu eu ex aliquet hendrerit. Aliquam erat volutpat. Sed at nisi sit amet mauris fringilla tristique.",
                "Praesent efficitur urna nec magna bibendum, eu iaculis nibh volutpat. Nam ac tellus et augue malesuada ullamcorper. Integer tincidunt auctor elit, ac dapibus ante. Nulla malesuada bibendum lectus vel tristique. Maecenas in faucibus turpis. Proin sed quam sapien. Duis rhoncus tincidunt dui, ac consectetur justo pellentesque vel.",
                "Suspendisse potenti. Nulla facilisi. Suspendisse suscipit varius felis, vel tempor dolor consequat a. Sed eget lorem eu urna malesuada semper. Maecenas venenatis magna at nibh molestie luctus. Nam vel urna diam. Nunc in turpis id augue posuere commodo. Suspendisse auctor, massa quis malesuada tincidunt, magna sapien dignissim metus, sit amet finibus ipsum metus sit amet velit.",
                "Vivamus a est libero. Nulla sed sapien eu nisl venenatis vestibulum non vel ipsum. Morbi sagittis turpis id massa tincidunt hendrerit. Ut pellentesque sapien vel leo cursus, at cursus massa eleifend. Fusce vel ante sed ipsum pellentesque lacinia vitae vel tellus. Integer quis urna eu justo finibus egestas.",
                "Duis sit amet lorem ex. Pellentesque nec suscipit massa. Sed at nunc at sem facilisis hendrerit. Sed faucibus, nibh vel malesuada molestie, elit mi rhoncus sapien, ac facilisis est lorem vel ipsum. Nullam sit amet nulla nisl. Etiam suscipit consectetur lectus, sit amet maximus neque dapibus at. Sed at eleifend quam. Fusce pellentesque mauris enim, vel ultricies nisl fermentum in.",
            ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        // Assert content is updated
        let msg = QueryMsg::GetThread { id: 0 };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let value: Thread = from_binary(&res).unwrap();
        assert_eq!(
            vec![
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec dictum aliquam eros, eu molestie sapien dignissim eget. Nullam tincidunt orci in dolor fermentum suscipit. Sed non lectus non massa lobortis blandit. Sed eget purus quam. Nam euismod arcu eu ex aliquet hendrerit. Aliquam erat volutpat. Sed at nisi sit amet mauris fringilla tristique.",
                "Praesent efficitur urna nec magna bibendum, eu iaculis nibh volutpat. Nam ac tellus et augue malesuada ullamcorper. Integer tincidunt auctor elit, ac dapibus ante. Nulla malesuada bibendum lectus vel tristique. Maecenas in faucibus turpis. Proin sed quam sapien. Duis rhoncus tincidunt dui, ac consectetur justo pellentesque vel.",
                "Suspendisse potenti. Nulla facilisi. Suspendisse suscipit varius felis, vel tempor dolor consequat a. Sed eget lorem eu urna malesuada semper. Maecenas venenatis magna at nibh molestie luctus. Nam vel urna diam. Nunc in turpis id augue posuere commodo. Suspendisse auctor, massa quis malesuada tincidunt, magna sapien dignissim metus, sit amet finibus ipsum metus sit amet velit.",
                "Vivamus a est libero. Nulla sed sapien eu nisl venenatis vestibulum non vel ipsum. Morbi sagittis turpis id massa tincidunt hendrerit. Ut pellentesque sapien vel leo cursus, at cursus massa eleifend. Fusce vel ante sed ipsum pellentesque lacinia vitae vel tellus. Integer quis urna eu justo finibus egestas.",
                "Duis sit amet lorem ex. Pellentesque nec suscipit massa. Sed at nunc at sem facilisis hendrerit. Sed faucibus, nibh vel malesuada molestie, elit mi rhoncus sapien, ac facilisis est lorem vel ipsum. Nullam sit amet nulla nisl. Etiam suscipit consectetur lectus, sit amet maximus neque dapibus at. Sed at eleifend quam. Fusce pellentesque mauris enim, vel ultricies nisl fermentum in.",
            ]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            value.content
        );
    }

    #[test]
    fn test_get_threads_created() {
        // Instantiate contract
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            admin: "creator".to_string(),
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        // Create multiple threads
        let msg = ExecuteMsg::CreateThread {
            title: "Hello".to_string(),
            description: "World".to_string(),
            content: vec!["Hello World".to_string()],
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        let msg = ExecuteMsg::CreateThread {
            title: "Hello".to_string(),
            description: "World".to_string(),
            content: vec!["Hello World 2".to_string()],
        }; 
        let info = mock_info("creator", &coins(1000, "earth"));
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        let msg = ExecuteMsg::CreateThread {
            title: "Hello".to_string(),
            description: "World".to_string(),
            content: vec!["Hello World 3".to_string()],
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        // Assert content is updated
        let msg = QueryMsg::GetThreadsCreated { start: None, end: None  };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();        
        let value: Vec<Thread> = from_binary(&res).unwrap();
        assert_eq!(3, value.len());
        assert_eq!("Hello World", value[0].content[0]);
        assert_eq!("Hello World 2", value[1].content[0]);
        assert_eq!("Hello World 3", value[2].content[0]);
    }
}
