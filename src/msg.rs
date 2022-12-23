use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub nonce: String,
    pub admin: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    KujiraDenomMsg(kujira::msg::DenomMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
