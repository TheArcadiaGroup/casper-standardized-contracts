#![allow(unused_parens)]
#![allow(non_snake_case)]
#![no_main]

extern crate alloc;

use contract::{
    contract_api::{
        runtime,
        storage::{self, create_contract_package_at_hash},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use libs::access::AccessControl;
use types::{contracts::NamedKeys, EntryPoints, Key, U256};

/// # Purpose
/// * Returns the `has_role` property.
#[no_mangle]
pub extern "C" fn has_role() {
    AccessControl::ret_has_role()
}

/// # Purpose
/// * Returns the `get_role_admin` property.
#[no_mangle]
pub extern "C" fn get_role_admin() {
    AccessControl::ret_role_admin()
}

/// # Purpose
/// * Grant role to given address
/// # Arguments
/// * `role` - `U256` -> Role.
/// * `account` - `Key` -> Address of the account.
#[no_mangle]
pub extern "C" fn grant_role() {
    AccessControl::grant_role();
}

/// # Purpose
/// * Revoke role
/// # Arguments
/// * `role` - `U256` -> Role.
/// * `account` - `Key` -> Address of the account.
#[no_mangle]
pub extern "C" fn revoke_role() {
    AccessControl::revoke_role();
}

/// # Purpose
/// * Renounce role
/// # Arguments
/// * `role` - `U256` -> Role.
/// * `account` - `Key` -> Address of the account.
#[no_mangle]
pub extern "C" fn renounce_role() {
    AccessControl::renounce_role();
}

/// # Purpose
/// * Set admin role
/// # Arguments
/// * `role` - `U256` -> Role.
/// * `admin_role` - `U256` -> Admin role.
#[no_mangle]
pub extern "C" fn set_role_admin() {
    let role: U256 = runtime::get_named_arg("role");
    let admin_role: U256 = runtime::get_named_arg("admin_role");

    AccessControl::check_only_role(role);
    AccessControl::_set_role_admin(role, admin_role);
}

#[no_mangle]
pub extern "C" fn call() {
    let default_admin: Key = runtime::get_named_arg("default_admin");

    let mut entry_points = EntryPoints::new();

    AccessControl::set_entry_points(&mut entry_points);

    let role_admin_seed_uref =
        storage::new_dictionary(AccessControl::ACCESS_ROLE_ADMIN_KEY).unwrap_or_revert();
    let role_members_seed_uref =
        storage::new_dictionary(AccessControl::ACCESS_ROLE_MEMBER_KEY).unwrap_or_revert();

    let mut named_keys = NamedKeys::new();

    let (contract_package_hash, access_uref) = create_contract_package_at_hash();
    named_keys.insert(
        AccessControl::ACCESS_ROLE_ADMIN_KEY.to_string(),
        role_admin_seed_uref.into(),
    );
    named_keys.insert(
        AccessControl::ACCESS_ROLE_MEMBER_KEY.to_string(),
        role_members_seed_uref.into(),
    );
    named_keys.insert(
        "contract_package_hash".to_string(),
        storage::new_uref(contract_package_hash).into(),
    );

    storage::dictionary_put(
        role_members_seed_uref,
        &AccessControl::get_role_members_key(AccessControl::DEFAULT_ADMIN_ROLE, default_admin),
        true,
    );

    // Add new version to the package.
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    runtime::put_key(&"AccessControl", contract_hash.into());
    runtime::put_key(
        &"AccessControl_hash",
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(&"AccessControl_package_hash", contract_package_hash.into());
    runtime::put_key(&"AccessControl_access_token", access_uref.into());
}
