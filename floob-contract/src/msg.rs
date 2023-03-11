use crate::state::{Thread, ThreadElem};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateThread { title: String, description: String },
    CreateThreadElem { thread_id: u64, content: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Thread)]
    Thread { id: u64 },
    #[returns(ThreadElem)]
    ThreadElem { thread_id: u64, elem_id: u64 },
}
