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

/// Returns the default set of Ownable entry points.
pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(owner());
    entry_points.add_entry_point(transfer_ownership());
    entry_points.add_entry_point(renounce_ownership());
    entry_points
}
