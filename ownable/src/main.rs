#![allow(unused_parens)]
#![allow(non_snake_case)]
#![no_main]

extern crate alloc;

use contract::contract_api::{
    runtime,
    storage::{self, create_contract_package_at_hash},
};
use libs::access::ownable::OwnableLib;
use types::{contracts::NamedKeys, EntryPoints, Key};

/// # Purpose
/// * Returns the `owner` property.
#[cfg(not(feature = "no_owner"))]
#[no_mangle]
pub extern "C" fn owner() {
    OwnableLib::ret_owner()
}

/// # Purpose
/// * Transfers the ownership of the contract to the given address.
/// # Arguments
/// * `new_owner` - `Key` -> Address of the new owner.
#[no_mangle]
pub extern "C" fn transfer_ownership() {
    OwnableLib::transfer_ownership();
}

/// # Purpose
/// * Renounce ownership of contract.
#[no_mangle]
pub extern "C" fn renounce_ownership() {
    OwnableLib::renounce_ownership();
}

#[no_mangle]
pub extern "C" fn call() {
    let owner: Key = runtime::get_named_arg("owner");

    let mut entry_points = EntryPoints::new();

    OwnableLib::set_entry_points(&mut entry_points);

    let mut named_keys = NamedKeys::new();

    let (contract_package_hash, access_uref) = create_contract_package_at_hash();
    named_keys.insert("owner".to_string(), storage::new_uref(owner.clone()).into());
    named_keys.insert(
        "contract_package_hash".to_string(),
        storage::new_uref(contract_package_hash).into(),
    );

    // Add new version to the package.
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    runtime::put_key(&"Ownable", contract_hash.into());
    runtime::put_key(&"Ownable_hash", storage::new_uref(contract_hash).into());
    runtime::put_key(&"Ownable_package_hash", contract_package_hash.into());
    runtime::put_key(&"Ownable_access_token", access_uref.into());
}
