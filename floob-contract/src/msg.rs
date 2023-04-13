use crate::state::Thread;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateThread {
        title: String,
        description: String,
        content: Vec<String>,
        img_url: Option<String>,
    },
    EditThread {
        id: u64,
        title: String,
        description: String,
        content: Vec<String>,
        img_url: Option<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Thread)]
    GetThread { id: u64 },
    #[returns(Vec<Thread>)]
    GetThreadsCreated {
        start: Option<u64>,
        end: Option<u64>,
    },
}

#[cw_serde]
pub enum MigrateMsg {}
