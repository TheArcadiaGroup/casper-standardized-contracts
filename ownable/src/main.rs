#![allow(unused_parens)]
#![allow(non_snake_case)]
#![no_main]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::convert::TryInto;

use contract::{
    contract_api::{
        runtime,
        storage::{self, create_contract_package_at_hash},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use libs::error::Error;
use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::NamedKeys,
    system::CallStackElement,
    ApiError, CLTyped, CLValue, Key, URef, U256,
};
pub mod entry_points;
pub mod events;
use events::OwnableEvent;

/// # Purpose
/// * Returns the `owner` property.
#[no_mangle]
pub extern "C" fn owner() {
    let owner: String = get_key("owner");
    ret(owner)
}

/// # Purpose
/// * Transfers the ownership of the contract to the given address.
/// # Arguments
/// * `new_owner` - `Key` -> Address of the new owner.
#[no_mangle]
pub extern "C" fn transfer_ownership() {
    let new_owner: Key = runtime::get_named_arg("new_owner");

    _transfer_ownership(new_owner, true);
}

/// # Purpose
/// * Renounce ownership of contract.
#[no_mangle]
pub extern "C" fn renounce_ownership() {
    _transfer_ownership(Key::Account(AccountHash::default()), true);
}

#[no_mangle]
pub extern "C" fn call() {
    let entry_points = entry_points::default();

    _transfer_ownership(get_caller(), false);

    let mut named_keys = NamedKeys::new();

    let (contract_package_hash, access_uref) = create_contract_package_at_hash();
    named_keys.insert(
        "contract_package_hash".to_string(),
        storage::new_uref(contract_package_hash).into(),
    );

    // Add new version to the package.
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    runtime::put_key(&"ownable", contract_hash.into());
    runtime::put_key(&"ownable_hash", storage::new_uref(contract_hash).into());
    runtime::put_key(&"ownable_package_hash", contract_package_hash.into());
    runtime::put_key(&"ownable_access_token", access_uref.into());
}

fn _transfer_ownership(new_owner: Key, check_permission: bool) {
    let old_owner =
        get_optional_key::<Key>("owner").unwrap_or(Key::Account(AccountHash::default()));

    if check_permission && old_owner != get_caller() {
        runtime::revert(Error::CannotMintToZeroHash);
    }
    set_key("owner", new_owner);

    events::emit(&OwnableEvent::OwnershipTransferred {
        old_owner,
        new_owner,
    });
}

fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

fn get_optional_key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

/// Returns the immediate caller address, whether it's an account or a contract.
fn get_caller() -> Key {
    let mut callstack = runtime::get_call_stack();
    callstack.pop();
    match callstack
        .last()
        .ok_or(Error::InvalidContext)
        .unwrap_or_revert()
    {
        CallStackElement::Session { account_hash } => (*account_hash).into(),
        CallStackElement::StoredSession {
            account_hash,
            contract_package_hash: _,
            contract_hash: _,
        } => (*account_hash).into(),
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => Key::from(*contract_package_hash),
    }
}
