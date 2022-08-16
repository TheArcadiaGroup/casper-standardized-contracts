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
use core::convert::TryInto;
use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::NamedKeys,
    system::CallStackElement,
    CLTyped, CLValue, ContractPackageHash, Key, URef,
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
