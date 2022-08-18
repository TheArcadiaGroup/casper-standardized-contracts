use casper_types::{account::AccountHash, Key, U256};

use crate::{
    token::erc20::erc20_helper::{token_cfg, Erc20},
    utilities::{key_to_str, to_key, Sender},
};

// ------------ START - ERC20 Tests ------------

#[test]
fn should_deploy() {
    let contract = Erc20::deployed(
        token_cfg::NAME.to_string(),
        token_cfg::SYMBOL.to_string(),
        token_cfg::DECIMALS,
        token_cfg::total_supply(),
    );

    assert_eq!(contract.name(), token_cfg::NAME);
    assert_eq!(contract.symbol(), token_cfg::SYMBOL);
    assert_eq!(contract.decimals(), token_cfg::DECIMALS);
    assert_eq!(contract.total_supply(), token_cfg::total_supply());

    assert_eq!(
        contract.balance_of(to_key(contract.ali)),
        token_cfg::total_supply()
    );
}

#[test]
fn should_transfer_token() {
    let mut contract = Erc20::deployed(
        token_cfg::NAME.to_string(),
        token_cfg::SYMBOL.to_string(),
        token_cfg::DECIMALS,
        token_cfg::total_supply(),
    );

    let amount = U256::from(1000_000_000_000u128);
    contract.transfer(to_key(contract.bob), amount, Sender(contract.ali));
    assert_eq!(contract.balance_of(to_key(contract.bob)), amount);
}

#[test]
#[should_panic = "65530"]
fn should_not_transfer_token_to_zero_address() {
    let mut contract = Erc20::deployed(
        token_cfg::NAME.to_string(),
        token_cfg::SYMBOL.to_string(),
        token_cfg::DECIMALS,
        token_cfg::total_supply(),
    );

    let amount = U256::from(1000_000_000_000u128);
    contract.transfer(
        Key::Account(AccountHash::default()),
        amount,
        Sender(contract.ali),
    );
}

#[test]
#[should_panic = "65529"]
fn should_not_transfer_bigger_amount_than_balance() {
    let mut contract = Erc20::deployed(
        token_cfg::NAME.to_string(),
        token_cfg::SYMBOL.to_string(),
        token_cfg::DECIMALS,
        token_cfg::total_supply(),
    );

    let amount = token_cfg::total_supply() + U256::from(10u128);
    contract.transfer(to_key(contract.bob), amount, Sender(contract.ali));
}

#[test]
fn should_approve_token() {
    let mut contract = Erc20::deployed(
        token_cfg::NAME.to_string(),
        token_cfg::SYMBOL.to_string(),
        token_cfg::DECIMALS,
        token_cfg::total_supply(),
    );

    let amount = U256::from(1000_000_000_000u128);
    contract.approve(to_key(contract.bob), amount, Sender(contract.ali));
    assert_eq!(
        contract.allowance(to_key(contract.ali), to_key(contract.bob)),
        amount
    );
}

#[test]
#[should_panic = "65530"]
fn should_not_approve_token_to_zero_address() {
    let mut contract = Erc20::deployed(
        token_cfg::NAME.to_string(),
        token_cfg::SYMBOL.to_string(),
        token_cfg::DECIMALS,
        token_cfg::total_supply(),
    );

    let amount = U256::from(1000_000_000_000u128);
    contract.approve(
        Key::Account(AccountHash::default()),
        amount,
        Sender(contract.ali),
    );
}
