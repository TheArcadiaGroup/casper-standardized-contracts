#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use contract::{
    contract_api::{
        runtime,
        storage::{self},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use core::convert::TryInto;
use types::{
    bytesrepr::{FromBytes, ToBytes},
    system::CallStackElement,
    CLType, CLTyped, CLValue, ContractPackageHash, EntryPoint, EntryPointAccess, EntryPointType,
    Key, Parameter, URef,
};

use crate::error::Error;

pub fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

pub fn get_optional_key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(value) => {
            let key: URef = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or(None)
        }
    }
}

pub fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
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
pub fn get_caller() -> Key {
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

pub fn contract_package_hash() -> ContractPackageHash {
    get_key::<ContractPackageHash>("contract_package_hash")
}

pub fn endpoint(name: &str, param: Vec<Parameter>, ret: CLType) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        param,
        ret,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn get_dictionary_seed_uref(name: &str) -> URef {
    match runtime::get_key(name) {
        Some(key) => key.into_uref().unwrap_or_revert(),
        None => {
            let new_dict = storage::new_dictionary(name).unwrap_or_revert();
            let key = storage::new_uref(new_dict).into();
            runtime::put_key(name, key);
            new_dict
        }
    }
}

pub fn get<T: FromBytes + CLTyped + Default>(dictionary_name: &str, key: &str) -> T {
    let dictionary_seed_uref = get_dictionary_seed_uref(dictionary_name);
    storage::dictionary_get(dictionary_seed_uref, key)
        .unwrap_or_default()
        .unwrap_or_default()
}

pub fn set<T: ToBytes + CLTyped>(dictionary_name: &str, key: &str, value: T) {
    let dictionary_seed_uref = get_dictionary_seed_uref(dictionary_name);
    storage::dictionary_put(dictionary_seed_uref, key, value)
}

pub fn key_to_str(key: &Key) -> String {
    let preimage = key.to_bytes().unwrap_or_revert();
    base64::encode(&preimage)
}
