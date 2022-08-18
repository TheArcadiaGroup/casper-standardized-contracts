use contract::{
    contract_api::{
        runtime,
        storage::{self},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use std::collections::BTreeMap;
use types::{
    account::AccountHash, bytesrepr::ToBytes, CLType, EntryPoint, EntryPoints, Key, Parameter,
    URef, U256,
};

use crate::{
    error::Error,
    token::erc20::ERC20,
    utils::{
        contract_package_hash, endpoint, get, get_caller, get_key, key_to_str, ret, set, set_key,
    },
};

struct ERC20EntryPoints {}

impl ERC20EntryPoints {
    /// Returns the `name` entry point.
    pub fn name() -> EntryPoint {
        endpoint("name", vec![], CLType::String)
    }

    /// Returns the `symbol` entry point.
    pub fn symbol() -> EntryPoint {
        endpoint("symbol", vec![], CLType::String)
    }

    /// Returns the `decimals` entry point.
    pub fn decimals() -> EntryPoint {
        endpoint("decimals", vec![], CLType::U8)
    }

    /// Returns the `total_supply` entry point.
    pub fn total_supply() -> EntryPoint {
        endpoint("total_supply", vec![], CLType::U256)
    }

    /// Returns the `balance_of` entry point.
    pub fn balance_of() -> EntryPoint {
        endpoint(
            "balance_of",
            vec![Parameter::new("account", CLType::Key)],
            CLType::U256,
        )
    }

    /// Returns the `allowance` entry point.
    pub fn allowance() -> EntryPoint {
        endpoint(
            "allowance",
            vec![
                Parameter::new("owner", CLType::Key),
                Parameter::new("spender", CLType::Key),
            ],
            CLType::U256,
        )
    }

    /// Returns the `approve` entry point.
    pub fn approve() -> EntryPoint {
        endpoint(
            "approve",
            vec![
                Parameter::new("spender", CLType::Key),
                Parameter::new("amount", CLType::U256),
            ],
            CLType::Bool,
        )
    }

    /// Returns the `increase_allowance` entry point.
    pub fn increase_allowance() -> EntryPoint {
        endpoint(
            "increase_allowance",
            vec![
                Parameter::new("spender", CLType::Key),
                Parameter::new("amount", CLType::U256),
            ],
            CLType::Bool,
        )
    }

    /// Returns the `decrease_allowance` entry point.
    pub fn decrease_allowance() -> EntryPoint {
        endpoint(
            "decrease_allowance",
            vec![
                Parameter::new("spender", CLType::Key),
                Parameter::new("amount", CLType::U256),
            ],
            CLType::Bool,
        )
    }

    /// Returns the `transfer` entry point.
    pub fn transfer() -> EntryPoint {
        endpoint(
            "transfer",
            vec![
                Parameter::new("to", CLType::Key),
                Parameter::new("amount", CLType::U256),
            ],
            CLType::Bool,
        )
    }

    /// Returns the `transfer_from` entry point.
    pub fn transfer_from() -> EntryPoint {
        endpoint(
            "transfer_from",
            vec![
                Parameter::new("from", CLType::Key),
                Parameter::new("to", CLType::Key),
                Parameter::new("amount", CLType::U256),
            ],
            CLType::Bool,
        )
    }
}

pub enum ERC20Event {
    Transfer {
        from: Key,
        to: Key,
        value: U256,
    },
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },
}

impl ERC20Event {
    pub fn type_name(&self) -> String {
        match self {
            ERC20Event::Transfer {
                from: _,
                to: _,
                value: _,
            } => "transfer",
            ERC20Event::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approval",
        }
        .to_string()
    }
}

impl ERC20 {
    pub const ERC20_NAME_KEY: &'static str = "name";
    pub const ERC20_SYMBOL_KEY: &'static str = "symbol";
    pub const ERC20_DECIMALS_KEY: &'static str = "decimals";
    pub const ERC20_BALANCE_KEY: &'static str = "balances";
    pub const ERC20_ALLOWANCE_KEY: &'static str = "allowances";
    pub const ERC20_TOTAL_SUPPLY_KEY: &'static str = "total_supply";

    pub fn set_entry_points(current_entry_points: &mut EntryPoints) -> &EntryPoints {
        current_entry_points.add_entry_point(ERC20EntryPoints::name());
        current_entry_points.add_entry_point(ERC20EntryPoints::symbol());
        current_entry_points.add_entry_point(ERC20EntryPoints::decimals());
        current_entry_points.add_entry_point(ERC20EntryPoints::total_supply());
        current_entry_points.add_entry_point(ERC20EntryPoints::balance_of());
        current_entry_points.add_entry_point(ERC20EntryPoints::allowance());
        current_entry_points.add_entry_point(ERC20EntryPoints::approve());
        current_entry_points.add_entry_point(ERC20EntryPoints::increase_allowance());
        current_entry_points.add_entry_point(ERC20EntryPoints::decrease_allowance());
        current_entry_points.add_entry_point(ERC20EntryPoints::transfer());
        current_entry_points.add_entry_point(ERC20EntryPoints::transfer_from());

        current_entry_points
    }

    pub fn emit(erc20_event: &ERC20Event) {
        let mut events = Vec::new();
        let package = contract_package_hash();
        match erc20_event {
            ERC20Event::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc20_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            ERC20Event::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc20_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }

    pub fn name() -> String {
        get_key(ERC20::ERC20_NAME_KEY)
    }

    pub fn ret_name() {
        ret(ERC20::name())
    }

    pub fn symbol() -> String {
        get_key(ERC20::ERC20_SYMBOL_KEY)
    }

    pub fn ret_symbol() {
        ret(ERC20::symbol())
    }

    pub fn decimals() -> u8 {
        get_key(ERC20::ERC20_DECIMALS_KEY)
    }

    pub fn ret_decimals() {
        ret(ERC20::decimals())
    }

    pub fn balance_of(account: Key) -> U256 {
        let balance: U256 = get(ERC20::ERC20_BALANCE_KEY, &key_to_str(&account));
        balance
    }

    pub fn ret_balance_of() {
        let account: Key = runtime::get_named_arg("account");
        ret(ERC20::balance_of(account))
    }

    pub fn total_supply() -> U256 {
        let supply: U256 = get_key(ERC20::ERC20_TOTAL_SUPPLY_KEY);
        supply
    }

    pub fn ret_total_supply() {
        ret(ERC20::total_supply())
    }

    pub fn get_allowance(owner: Key, spender: Key) -> U256 {
        let allowance: U256 = get(
            ERC20::ERC20_ALLOWANCE_KEY,
            &ERC20::get_allowances_key(owner, spender),
        );
        allowance
    }

    pub fn ret_allowance() {
        let owner: Key = runtime::get_named_arg("owner");
        let spender: Key = runtime::get_named_arg("spender");
        ret(ERC20::get_allowance(owner, spender))
    }

    pub fn approve() {
        let owner: Key = get_caller();
        let spender: Key = runtime::get_named_arg("spender");
        let amount: U256 = runtime::get_named_arg("amount");

        ERC20::_approve(owner, spender, amount);
    }

    pub fn increase_allowance() {
        let owner: Key = get_caller();
        let spender: Key = runtime::get_named_arg("spender");
        let amount: U256 = runtime::get_named_arg("amount");

        ERC20::_approve(
            owner,
            spender,
            ERC20::get_allowance(owner, spender) + amount,
        );
    }

    pub fn decrease_allowance() {
        let owner: Key = get_caller();
        let spender: Key = runtime::get_named_arg("spender");
        let amount: U256 = runtime::get_named_arg("amount");

        let current_allowance = ERC20::get_allowance(owner, spender);
        if current_allowance < amount {
            runtime::revert(Error::InsufficientAllowance);
        }
        ERC20::_approve(
            owner,
            spender,
            ERC20::get_allowance(owner, spender) - amount,
        );
    }

    pub fn transfer() {
        let from: Key = get_caller();
        let to: Key = runtime::get_named_arg("to");
        let amount: U256 = runtime::get_named_arg("amount");

        ERC20::_transfer(from, to, amount);
    }

    pub fn transfer_from() {
        let spender: Key = get_caller();
        let from: Key = runtime::get_named_arg("from");
        let to: Key = runtime::get_named_arg("to");
        let amount: U256 = runtime::get_named_arg("amount");

        ERC20::_spend_allowance(from, spender, amount);
        ERC20::_transfer(from, to, amount);
    }

    pub fn _transfer(from: Key, to: Key, amount: U256) {
        if from == Key::Account(AccountHash::default())
            || to == Key::Account(AccountHash::default())
        {
            runtime::revert(Error::ZeroAddress);
        }

        let from_balance = ERC20::balance_of(from);
        if from_balance < amount {
            runtime::revert(Error::InsufficientBalance);
        }

        let to_balance = ERC20::balance_of(to);

        set(
            ERC20::ERC20_BALANCE_KEY,
            &key_to_str(&from),
            from_balance - amount,
        );
        set(
            ERC20::ERC20_BALANCE_KEY,
            &key_to_str(&to),
            to_balance + amount,
        );

        ERC20::emit(&ERC20Event::Transfer {
            from,
            to,
            value: amount,
        });
    }

    pub fn _mint(to: Key, amount: U256) {
        if to == Key::Account(AccountHash::default()) {
            runtime::revert(Error::ZeroAddress);
        }

        let to_balance = ERC20::balance_of(to);
        let supply = ERC20::total_supply();

        set_key(ERC20::ERC20_TOTAL_SUPPLY_KEY, supply + amount);
        set(
            ERC20::ERC20_BALANCE_KEY,
            &key_to_str(&to),
            to_balance + amount,
        );

        ERC20::emit(&ERC20Event::Transfer {
            from: Key::Account(AccountHash::default()),
            to,
            value: amount,
        });
    }

    pub fn _burn(account: Key, amount: U256) {
        if account == Key::Account(AccountHash::default()) {
            runtime::revert(Error::ZeroAddress);
        }

        let account_balance = ERC20::balance_of(account);
        if account_balance < amount {
            runtime::revert(Error::InsufficientBalance);
        }
        let supply = ERC20::total_supply();

        set_key(ERC20::ERC20_TOTAL_SUPPLY_KEY, supply - amount);
        set(
            ERC20::ERC20_BALANCE_KEY,
            &key_to_str(&account),
            account_balance - amount,
        );

        ERC20::emit(&ERC20Event::Transfer {
            from: account,
            to: Key::Account(AccountHash::default()),
            value: amount,
        });
    }

    pub fn _approve(owner: Key, spender: Key, amount: U256) {
        if owner == Key::Account(AccountHash::default())
            || spender == Key::Account(AccountHash::default())
        {
            runtime::revert(Error::ZeroAddress);
        }

        set(
            ERC20::ERC20_ALLOWANCE_KEY,
            &ERC20::get_allowances_key(owner, spender),
            amount,
        );

        ERC20::emit(&ERC20Event::Approval {
            owner,
            spender,
            value: amount,
        });
    }

    pub fn _spend_allowance(owner: Key, spender: Key, amount: U256) {
        let allowance = ERC20::get_allowance(owner, spender);

        if allowance != U256::MAX {
            if allowance < amount {
                runtime::revert(Error::InsufficientAllowance);
            }
            set(
                ERC20::ERC20_ALLOWANCE_KEY,
                &ERC20::get_allowances_key(owner, spender),
                allowance - amount,
            );
        }
    }

    pub fn get_allowances_key(owner: Key, spender: Key) -> String {
        let mut preimage = Vec::new();
        preimage.append(&mut owner.to_bytes().unwrap_or_revert());
        preimage.append(&mut spender.to_bytes().unwrap_or_revert());

        let key_bytes = runtime::blake2b(&preimage);
        hex::encode(&key_bytes)
    }
}
