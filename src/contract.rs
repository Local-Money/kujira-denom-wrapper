#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg};
use kujira::msg::KujiraMsg;
use kujira::query::KujiraQuery;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Admin, ADMIN};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<KujiraQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    // Save the sender as the admin
    let config = Admin {
        addr: msg.admin.clone(),
    };
    ADMIN.save(deps.storage, &config)?;

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
    deps: DepsMut<KujiraQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    // Check if the sender is the admin
    let config = ADMIN.load(deps.storage)?;
    if config.addr != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    // Execute the message
    match msg {
        ExecuteMsg::KujiraDenomMsg(kujira_msg) => execute_kujira_denom_msg(kujira_msg),
        ExecuteMsg::UpdateAdmin { admin } => update_admin(deps, admin),
    }
}

pub fn execute_kujira_denom_msg(
    kujira_msg: kujira::msg::DenomMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    let res = Response::new()
        .add_attribute("method", "execute")
        .add_submessage(SubMsg::new(kujira_msg));
    Ok(res)
}

pub fn update_admin(
    deps: DepsMut<KujiraQuery>,
    admin: Option<Addr>,
) -> Result<Response<KujiraMsg>, ContractError> {
    let mut res = Response::new().add_attribute("method", "update_admin");
    if let Some(addr) = admin {
        let new_admin = Admin { addr };
        ADMIN.save(deps.storage, &new_admin)?;
        res = res.add_attribute("admin", new_admin.addr.to_string());
    } else {
        ADMIN.remove(deps.storage);
        res = res.add_attribute("admin", "removed");
    }
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}
