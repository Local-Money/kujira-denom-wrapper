#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg};
use kujira::msg::KujiraMsg;
use kujira::query::KujiraQuery;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:kujira-denom-minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<KujiraQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    // Save the sender as the admin
    let config = Config {
        admin: msg.admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;

    // Create SubMsg Creating a new Denom with the given nonce
    let create_denom_msg = KujiraMsg::Denom(kujira::msg::DenomMsg::Create {
        subdenom: msg.nonce.into(),
    });

    let res = Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", msg.admin)
        .add_submessage(SubMsg::new(create_denom_msg));
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut<KujiraQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    // Check if the sender is the admin
    let config = CONFIG.load(_deps.storage)?;
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    // Execute the message
    match msg {
        ExecuteMsg::KujiraDenomMsg(kujira_msg) => {
            let res = Response::new()
                .add_attribute("method", "execute")
                .add_submessage(SubMsg::new(kujira_msg));
            Ok(res)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
