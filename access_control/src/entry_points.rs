//! Contains definition of the ERC20 contract entry points.
use types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter};

pub fn endpoint(name: &str, param: Vec<Parameter>, ret: CLType) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        param,
        ret,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `has_role` entry point.
pub fn has_role() -> EntryPoint {
    endpoint(
        "has_role", 
        vec![
            Parameter::new("role", CLType::U256),
            Parameter::new("account", CLType::Key),
        ],
        CLType::Bool
    )
}

/// Returns the `get_role_admin` entry point.
pub fn get_role_admin() -> EntryPoint {
    endpoint(
        "get_role_admin", 
        vec![
            Parameter::new("role", CLType::U256),
        ],
        CLType::U256
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

/// Returns the default set of AccessControl entry points.
pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(has_role());
    entry_points.add_entry_point(get_role_admin());
    entry_points.add_entry_point(grant_role());
    entry_points.add_entry_point(revoke_role());
    entry_points.add_entry_point(renounce_role());
    entry_points
}
