#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::BTreeMap;

use contract::contract_api::storage;
use types::{ContractPackageHash, Key, URef, U256};

use crate::get_key;

pub enum AccessControlEvent {
    RoleAdminChanged { role: U256, previous_admin_role: U256, new_admin_role: U256 },
    RoleGranted { role: U256, account: Key, sender: Key },
    RoleRevoked { role: U256, account: Key, sender: Key },
}

impl AccessControlEvent {
    pub fn type_name(&self) -> String {
        match self {
            AccessControlEvent::RoleAdminChanged {
                role: _,
                previous_admin_role: _,
                new_admin_role: _,
            } => "role_admin_changed",
            AccessControlEvent::RoleGranted {
                role: _,
                account: _,
                sender: _,
            } => "role_granted",
            AccessControlEvent::RoleRevoked {
                role: _,
                account: _,
                sender: _,
            } => "role_revoked",
        }
        .to_string()
    }
}

pub fn contract_package_hash() -> ContractPackageHash {
    get_key::<ContractPackageHash>("contract_package_hash")
}

pub(crate) fn emit(event: &AccessControlEvent) {
    let mut events = Vec::new();
    let package = contract_package_hash();
    match event {
        AccessControlEvent::RoleAdminChanged {
            role,
            previous_admin_role,
            new_admin_role
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", event.type_name());
            event.insert("role", role.to_string());
            event.insert("previous_admin_role", previous_admin_role.to_string());
            event.insert("new_admin_role", new_admin_role.to_string());
            events.push(event);
        },
        AccessControlEvent::RoleGranted {
            role,
            account,
            sender
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", event.type_name());
            event.insert("role", role.to_string());
            event.insert("account", account.to_string());
            event.insert("sender", sender.to_string());
            events.push(event);
        },
        AccessControlEvent::RoleRevoked {
            role,
            account,
            sender
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", event.type_name());
            event.insert("role", role.to_string());
            event.insert("account", account.to_string());
            event.insert("sender", sender.to_string());
            events.push(event);
        }
    };
    for event in events {
        let _: URef = storage::new_uref(event);
    }
}
