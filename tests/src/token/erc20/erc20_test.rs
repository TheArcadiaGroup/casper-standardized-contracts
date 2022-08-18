use casper_types::{account::AccountHash, Key};

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

// #[test]
// fn should_transfer_ownership() {
//     let mut contract = Ownable::deployed();
//     contract.transfer_ownership(to_key(contract.bob), Sender(contract.ali));
//     assert_eq!(
//         contract.owner().to_string(),
//         Key::Account(contract.bob).to_string()
//     );
// }

// #[test]
// #[should_panic = "Authorization"]
// fn should_not_transfer_ownership_by_invalid_owner() {
//     let mut contract = Ownable::deployed();
//     contract.transfer_ownership(to_key(contract.bob), Sender(contract.joe));
// }

// #[test]
// fn should_renounce_ownership() {
//     let mut contract = Ownable::deployed();
//     contract.renounce_ownership(Sender(contract.ali));
//     assert_eq!(
//         contract.owner().to_string(),
//         Key::Account(AccountHash::default()).to_string()
//     );
// }

// #[test]
// #[should_panic = "Authorization"]
// fn should_not_renounce_ownership_by_invalid_owner() {
//     let mut contract = Ownable::deployed();
//     contract.renounce_ownership(Sender(contract.joe));
// }
