use contract::contract_api::{
    runtime,
    storage::{self},
};
use std::collections::BTreeMap;
use types::{CLType, EntryPoint, EntryPoints, Key, Parameter, URef, U256};

use crate::{
    access::AccessControl,
    error::Error,
    utils::{contract_package_hash, endpoint, get, get_caller, key_to_str, ret, set},
};

struct AccessControlEntryPoints {}

impl AccessControlEntryPoints {
    /// Returns the `has_role` entry point.
    pub fn has_role() -> EntryPoint {
        endpoint(
            "has_role",
            vec![
                Parameter::new("role", CLType::U256),
                Parameter::new("account", CLType::Key),
            ],
            CLType::Bool,
        )
    }

    /// Returns the `get_role_admin` entry point.
    pub fn get_role_admin() -> EntryPoint {
        endpoint(
            "get_role_admin",
            vec![Parameter::new("role", CLType::U256)],
            CLType::U256,
        )
    }

    /// Returns the `grant_role` entry point.
    pub fn grant_role() -> EntryPoint {
        endpoint(
            "grant_role",
            vec![
                Parameter::new("role", CLType::U256),
                Parameter::new("account", CLType::Key),
            ],
            CLType::Unit,
        )
    }

    /// Returns the `revoke_role` entry point.
    pub fn revoke_role() -> EntryPoint {
        endpoint(
            "revoke_role",
            vec![
                Parameter::new("role", CLType::U256),
                Parameter::new("account", CLType::Key),
            ],
            CLType::Unit,
        )
    }

    /// Returns the `renounce_role` entry point.
    pub fn renounce_role() -> EntryPoint {
        endpoint(
            "renounce_role",
            vec![
                Parameter::new("role", CLType::U256),
                Parameter::new("account", CLType::Key),
            ],
            CLType::Unit,
        )
    }
}

pub enum AccessControlEvent {
    OwnershipTransferred { old_owner: Key, new_owner: Key },
}

impl AccessControlEvent {
    pub fn type_name(&self) -> String {
        match self {
            AccessControlEvent::OwnershipTransferred {
                old_owner: _,
                new_owner: _,
            } => "ownership_transferred",
        }
        .to_string()
    }
}

impl AccessControl {
    pub const DEFAULT_ADMIN_ROLE: U256 = U256::zero();
    pub const ACCESS_ROLE_MEMBER_KEY: &'static str = "_access_control_members";
    pub const ACCESS_ROLE_ADMIN_KEY: &'static str = "_access_control_admin";

    pub fn set_entry_points(current_entry_points: &mut EntryPoints) -> &EntryPoints {
        current_entry_points.add_entry_point(AccessControlEntryPoints::has_role());
        current_entry_points.add_entry_point(AccessControlEntryPoints::get_role_admin());
        current_entry_points.add_entry_point(AccessControlEntryPoints::grant_role());
        current_entry_points.add_entry_point(AccessControlEntryPoints::revoke_role());
        current_entry_points.add_entry_point(AccessControlEntryPoints::renounce_role());

        current_entry_points
    }

    pub fn emit(ownable_event: &AccessControlEvent) {
        let mut events = Vec::new();
        let package = contract_package_hash();
        match ownable_event {
            AccessControlEvent::OwnershipTransferred {
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

    pub fn has_role(role: U256, account: Key) -> bool {
        get(
            AccessControl::ACCESS_ROLE_MEMBER_KEY,
            &AccessControl::get_role_members_key(role, account),
        )
    }

    pub fn ret_has_role() {
        let role: U256 = runtime::get_named_arg("role");
        let account: Key = runtime::get_named_arg("account");

        ret(AccessControl::has_role(role, account))
    }

    pub fn get_role_admin(role: U256) -> U256 {
        get(
            AccessControl::ACCESS_ROLE_ADMIN_KEY,
            &AccessControl::get_role_admin_key(role),
        )
    }

    pub fn ret_role_admin() {
        let role: U256 = runtime::get_named_arg("role");

        ret(AccessControl::get_role_admin(role))
    }

    pub fn grant_role() {
        let role: U256 = runtime::get_named_arg("role");
        let account: Key = runtime::get_named_arg("account");

        AccessControl::check_only_role(AccessControl::get_role_admin(role));

        AccessControl::_grant_role(role, account);
    }

    pub fn revoke_role() {
        let role: U256 = runtime::get_named_arg("role");
        let account: Key = runtime::get_named_arg("account");

        AccessControl::check_only_role(AccessControl::get_role_admin(role));

        AccessControl::_revoke_role(role, account);
    }

    pub fn renounce_role() {
        let role: U256 = runtime::get_named_arg("role");
        let account: Key = runtime::get_named_arg("account");

        if account != get_caller() {
            runtime::revert(Error::InvalidPermission);
        }

        AccessControl::_revoke_role(role, account);
    }

    pub fn check_only_role(role: U256) {
        if !AccessControl::has_role(role, get_caller()) {
            runtime::revert(Error::InvalidPermission);
        }
    }

    pub fn check_role(role: U256, account: Key) {
        if !AccessControl::has_role(role, account) {
            runtime::revert(Error::InvalidPermission);
        }
    }

    pub fn _set_role_admin(role: U256, admin_role: U256) {
        set(
            AccessControl::ACCESS_ROLE_ADMIN_KEY,
            &AccessControl::get_role_admin_key(role),
            admin_role,
        );
    }

    pub fn _grant_role(role: U256, account: Key) {
        if !AccessControl::has_role(role, account) {
            set(
                AccessControl::ACCESS_ROLE_MEMBER_KEY,
                &AccessControl::get_role_members_key(role, account),
                true,
            );
        }
    }

    pub fn _revoke_role(role: U256, account: Key) {
        if AccessControl::has_role(role, account) {
            set(
                AccessControl::ACCESS_ROLE_MEMBER_KEY,
                &AccessControl::get_role_members_key(role, account),
                false,
            );
        }
    }

    pub fn get_role_members_key(role: U256, account: Key) -> String {
        [key_to_str(&account), role.to_string()].join("_")
    }

    pub fn get_role_admin_key(role: U256) -> String {
        role.to_string()
    }
}
