use casper_types::{account::AccountHash, Key};

use crate::access::ownable_helper::{Ownable, Sender};

fn to_key(account: AccountHash) -> Key {
    Key::Account(account)
}

// ------------ START - ERC20 Tests ------------

#[test]
fn should_deploy_ownable() {
    let contract = Ownable::deployed();
    assert_eq!(
        contract.owner().to_string(),
        Key::Account(contract.ali).to_string()
    );
}

#[test]
fn should_transfer_ownership() {
    let mut contract = Ownable::deployed();
    contract.transfer_ownership(to_key(contract.bob), Sender(contract.ali));
    assert_eq!(
        contract.owner().to_string(),
        Key::Account(contract.bob).to_string()
    );
}

#[test]
#[should_panic = "Authorization"]
fn should_not_transfer_ownership_by_invalid_owner() {
    let mut contract = Ownable::deployed();
    contract.transfer_ownership(to_key(contract.bob), Sender(contract.joe));
}

#[test]
fn should_renounce_ownership() {
    let mut contract = Ownable::deployed();
    contract.renounce_ownership(Sender(contract.ali));
    assert_eq!(
        contract.owner().to_string(),
        Key::Account(AccountHash::default()).to_string()
    );
}

#[test]
#[should_panic = "Authorization"]
fn should_not_renounce_ownership_by_invalid_owner() {
    let mut contract = Ownable::deployed();
    contract.renounce_ownership(Sender(contract.joe));
}
