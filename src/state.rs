use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define an Admin struct with the admin address
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Admin {
    pub addr: Addr,
}

// Create a storage Item for the Admin struct
pub const ADMIN: Item<Admin> = Item::new("admin");
