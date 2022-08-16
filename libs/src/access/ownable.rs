use contract::contract_api::{
    runtime,
    storage::{self},
};
use std::collections::BTreeMap;
use types::{account::AccountHash, CLType, EntryPoint, EntryPoints, Key, Parameter, URef};

use crate::{
    error::Error,
    utils::{contract_package_hash, endpoint, get_caller, get_optional_key, ret, set_key},
};

struct OwnableEntryPoints {}

impl OwnableEntryPoints {
    /// Returns the `owner` entry point.
    pub fn owner() -> EntryPoint {
        endpoint("owner", vec![], CLType::Key)
    }

    /// Returns the `transfer_ownership` entry point.
    pub fn transfer_ownership() -> EntryPoint {
        endpoint(
            "transfer_ownership",
            vec![Parameter::new("new_owner", CLType::Key)],
            CLType::Unit,
        )
    }

    /// Returns the `renounce_ownership` entry point.
    pub fn renounce_ownership() -> EntryPoint {
        endpoint("renounce_ownership", vec![], CLType::Unit)
    }
}
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

pub struct OwnableLib {}

pub fn set_entry_points(current_entry_points: &mut EntryPoints) -> &EntryPoints {
    current_entry_points.add_entry_point(OwnableEntryPoints::owner());
    current_entry_points.add_entry_point(OwnableEntryPoints::transfer_ownership());
    current_entry_points.add_entry_point(OwnableEntryPoints::renounce_ownership());

    current_entry_points
}

pub fn emit(ownable_event: &OwnableEvent) {
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

pub fn owner() -> Key {
    get_optional_key("owner").unwrap_or(Key::Account(AccountHash::default()))
}

pub fn ret_owner() {
    let owner = owner();
    ret(owner)
}

pub fn transfer_ownership() {
    let new_owner: Key = runtime::get_named_arg("new_owner");

    _transfer_ownership(new_owner, true);
}

pub fn renounce_ownership() {
    _transfer_ownership(Key::Account(AccountHash::default()), true);
}

fn _transfer_ownership(new_owner: Key, check_permission: bool) {
    let old_owner =
        get_optional_key::<Key>("owner").unwrap_or(Key::Account(AccountHash::default()));

    if check_permission && old_owner != get_caller() {
        runtime::revert(Error::CannotMintToZeroHash);
    }
    set_key("owner", new_owner);

    emit(&OwnableEvent::OwnershipTransferred {
        old_owner,
        new_owner,
    });
}
