use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Map, Item};

#[cw_serde]
pub struct Post {
    pub title: String, 
    pub description: String 
} 

pub const POSTS: Map<u64, Post> = Map::new("posts");

pub const POST_COUNT: Item<u64> = Item::new("posts_count");
