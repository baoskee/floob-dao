use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Map, Item};
use cosmwasm_std::{Addr};

/**
 * This is more like a thread
 */
#[cw_serde]
pub struct Thread {
    pub title: String, 
    pub description: String,
    pub author: Addr 
} 

#[cw_serde]
pub struct SubThread {
    pub content: String,
    pub author: Addr 
}

/**
 * TODO: Is this a good way to store lists inside CosmWasm? 
 */
pub const THREADS: Map<u64, Thread> = Map::new("posts");
pub const THREAD_COUNT: Item<u64> = Item::new("posts_count");

pub const SUB_THREAD: Map<(u64, u64), SubThread> = Map::new("sub_thread"); 
pub const SUB_THREAD_COUNT: Map<u64, u64> = Map::new("sub_thread_count");
