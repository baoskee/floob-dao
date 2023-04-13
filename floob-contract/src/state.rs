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
    pub content: Vec<String>,
    pub img_url: Option<String>,
}

// The DAO controlling the story-telling contract
pub const ADMIN: Item<Addr> = Item::new("admin");

pub const THREADS: Map<u64, Thread> = Map::new("posts");
pub const THREAD_COUNT: Item<u64> = Item::new("posts_count");
pub const THREAD_IMG_URL: Map<u64, String> = Map::new("post_img_url");
