#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::BTreeMap;

use contract::contract_api::storage;
use types::{ContractPackageHash, Key, URef};

use crate::get_key;

pub enum OwnableEvent {
    OwnershipTransferred { old_owner: Key, new_owner: Key },
}

impl OwnableEvent {
    pub fn type_name(&self) -> String {
        match self {
            OwnableEvent::OwnershipTransferred {
                old_owner: _,
                new_owner: _,
            } => "ownership_transferred",
        }
        .to_string()
    }
}

pub fn contract_package_hash() -> ContractPackageHash {
    get_key::<ContractPackageHash>("contract_package_hash")
}

pub(crate) fn emit(ownable_event: &OwnableEvent) {
    let mut events = Vec::new();
    let package = contract_package_hash();
    match ownable_event {
        OwnableEvent::OwnershipTransferred {
            old_owner,
            new_owner,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", ownable_event.type_name());
            event.insert("old_owner", old_owner.to_string());
            event.insert("new_owner", new_owner.to_string());
            events.push(event);
        }
    };
    for event in events {
        let _: URef = storage::new_uref(event);
    }
}
