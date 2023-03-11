use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::state::Thread;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateThread {
        title: String,
        description: String
    },
    CreateSubthread {
        thread_id: u64,
        content: String
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Thread)]     
    Thread { id: u64 } 
}
