use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

/**
 * This is more like a thread
 */
#[cw_serde]
pub struct Thread {
    pub title: String,
    pub description: String,
    pub author: Addr,
}

#[cw_serde]
pub struct ThreadElem {
    pub content: String,
    pub author: Addr,
}

/**
 * TODO(1): Is this a good way to store lists inside CosmWasm?
 */
pub const THREADS: Map<u64, Thread> = Map::new("posts");
pub const THREAD_COUNT: Item<u64> = Item::new("posts_count");

pub const THREAD_ELEM: Map<(u64, u64), ThreadElem> = Map::new("sub_thread");
pub const THREAD_ELEM_COUNT: Map<u64, u64> = Map::new("sub_thread_count");
