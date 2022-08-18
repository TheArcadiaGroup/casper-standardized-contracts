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
use libs::{
    utils::CONTRACT_PACKAGE_HASH_KEY,
    {token::erc20::ERC20, utils::key_to_str},
};
use types::{contracts::NamedKeys, EntryPoints, Key, U256};

/// # Purpose
/// * Returns the `name` property.
#[no_mangle]
pub extern "C" fn name() {
    ERC20::ret_name()
}

/// # Purpose
/// * Returns the `symbol` property.
#[no_mangle]
pub extern "C" fn symbol() {
    ERC20::ret_symbol()
}

/// # Purpose
/// * Returns the `decimals` property.
#[no_mangle]
pub extern "C" fn decimals() {
    ERC20::ret_decimals()
}

/// # Purpose
/// * Returns the `total_supply` of the token.
#[no_mangle]
pub extern "C" fn total_supply() {
    ERC20::ret_total_supply()
}

/// # Purpose
/// * Returns how much tokens the given `address` owns.
/// # Arguments
/// * `account` - `Key` -> Address that we are looking for it's token balance.
/// # Returns
/// * `balance` - `U256` -> The given `address`'s balance.
#[no_mangle]
pub extern "C" fn balance_of() {
    ERC20::ret_balance_of()
}

/// # Purpose
/// * Returns how much allowance the `owner` has given to the `spender`.
/// # Arguments
/// * `owner` - `Key` -> Address of the owner.
/// * `spender` - `Key` -> Address of the spender.
/// # Returns
/// * `amount` - `U256` -> Amount of the allowance.
#[no_mangle]
pub extern "C" fn allowance() {
    ERC20::ret_allowance()
}

/// # Purpose
/// * Grants an address the liberty to spend an amount of the caller's tokens.
/// # Arguments
/// * `spender` - `Key` -> Address of the spender.
/// * `amount` - `U256` -> Amount of the allowance.
#[no_mangle]
pub extern "C" fn approve() {
    ERC20::approve()
}

/// # Purpose
/// * Transfers an amount of the caller's tokens to the given address.
/// # Arguments
/// * `to` - `Key` -> Address of the recipient.
/// * `amount` - `U256` -> Amount of the tokens to be sent.
#[no_mangle]
pub extern "C" fn transfer() {
    ERC20::transfer()
}

/// # Purpose
/// * Increase allowance from current allowance
/// # Arguments
/// * `spender` - `Key` -> Address of the spender.
/// * `amount` - `U256` -> Amount of the allowance.
#[no_mangle]
pub extern "C" fn increase_allowance() {
    ERC20::increase_allowance()
}

/// # Purpose
/// * Decrease allowance from current allowance
/// # Arguments
/// * `spender` - `Key` -> Address of the spender.
/// * `amount` - `U256` -> Amount of the allowance.
#[no_mangle]
pub extern "C" fn decrease_allowance() {
    ERC20::decrease_allowance()
}

/// # Purpose
/// * Transfers an `amount` of tokens from `from` to `to`.
/// # Arguments
/// * `from` - `Key` -> Address of the owner.
/// * `to` - `Key` -> Address of the recipient.
/// * `amount` - `U256` -> Amount of the tokens to be sent.
#[no_mangle]
pub extern "C" fn transfer_from() {
    ERC20::transfer_from()
}

#[no_mangle]
pub extern "C" fn call() {
    let token_name: String = runtime::get_named_arg("name");
    let token_symbol: String = runtime::get_named_arg("symbol");
    let token_decimals: u8 = runtime::get_named_arg("decimals");
    let token_total_supply: U256 = runtime::get_named_arg("total_supply");

    let mut entry_points = EntryPoints::new();

    ERC20::set_entry_points(&mut entry_points);

    let balances_seed_uref = storage::new_dictionary(ERC20::ERC20_BALANCE_KEY).unwrap_or_revert();

    storage::dictionary_put(
        balances_seed_uref,
        &key_to_str(&Key::Account(runtime::get_caller())),
        token_total_supply,
    );

    let allowances_seed_uref =
        storage::new_dictionary(ERC20::ERC20_ALLOWANCE_KEY).unwrap_or_revert();
    let mut named_keys = NamedKeys::new();

    named_keys.insert(
        ERC20::ERC20_NAME_KEY.to_string(),
        storage::new_uref(token_name.clone()).into(),
    );
    named_keys.insert(
        ERC20::ERC20_SYMBOL_KEY.to_string(),
        storage::new_uref(token_symbol).into(),
    );
    named_keys.insert(
        ERC20::ERC20_DECIMALS_KEY.to_string(),
        storage::new_uref(token_decimals).into(),
    );
    named_keys.insert(
        ERC20::ERC20_TOTAL_SUPPLY_KEY.to_string(),
        storage::new_uref(token_total_supply).into(),
    );
    named_keys.insert(
        ERC20::ERC20_BALANCE_KEY.to_string(),
        balances_seed_uref.into(),
    );
    named_keys.insert(
        ERC20::ERC20_ALLOWANCE_KEY.to_string(),
        allowances_seed_uref.into(),
    );

    let (contract_package_hash, access_uref) = create_contract_package_at_hash();
    named_keys.insert(
        CONTRACT_PACKAGE_HASH_KEY.to_string(),
        storage::new_uref(contract_package_hash).into(),
    );

    // Add new version to the package.
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    runtime::put_key(&"Erc20", contract_hash.into());
    runtime::put_key(&"Erc20_hash", storage::new_uref(contract_hash).into());
    runtime::put_key(&"Erc20_package_hash", contract_package_hash.into());
    runtime::put_key(&"Erc20_access_token", access_uref.into());
}
