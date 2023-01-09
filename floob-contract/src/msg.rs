use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::state::Post;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreatePost {
        title: String,
        description: String
    }     
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Post)]     
    Post { id: u64 } 
}
