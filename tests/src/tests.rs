use casper_types::{account::AccountHash, Key, PublicKey, U256};
use casper_types::{AsymmetricType, ContractHash, ContractPackageHash};
use k256::ecdsa::VerifyingKey;
use libs::converters::set_size_32;

use crate::ownable::{token_cfg, Ownable, Sender};
// use crate::utilities::{
//     encode_price, expand_to_18_decimals, get_burn_amount, get_pair_approval_digest,
//     increment_block_timestamp,
// };
use ed25519_dalek::{Keypair, Signature as EdSignature, Signer};
use k256::ecdsa::{Signature, SigningKey};
use rand::rngs::OsRng;

fn to_key(account: AccountHash) -> Key {
    Key::Account(account)
}

// ------------ START - ERC20 Tests ------------

#[test]
fn should_deploy_ownable() {
    let t = Ownable::deployed();
    // assert_eq!(t.name(), token_cfg::NAME);
    // assert_eq!(t.symbol(), token_cfg::SYMBOL);
    // assert_eq!(t.decimals(), token_cfg::DECIMALS);
    // assert_eq!(t.balance_of(to_key(t.ali)), token_cfg::total_supply());
}

// #[test]
// fn should_transfer_erc20() {
//     let amount = 10.into();
//     let mut t = Ownable::deployed("ERC20", "ERC");
//     t.transfer(to_key(t.bob), amount, Sender(t.ali));
//     assert_eq!(
//         t.balance_of(to_key(t.ali)),
//         token_cfg::total_supply() - amount
//     );
//     assert_eq!(t.balance_of(to_key(t.bob)), amount);
// }

// #[test]
// #[should_panic = "65534"]
// fn should_not_transfer_too_much_erc20() {
//     let amount = 1.into();
//     let mut t = Ownable::deployed("ERC20", "ERC");
//     t.transfer(to_key(t.ali), amount, Sender(t.bob));
// }

// #[test]
// fn should_approve_erc20() {
//     let amount = 10.into();
//     let mut t = Ownable::deployed("ERC20", "ERC");
//     t.approve(to_key(t.bob), amount, Sender(t.ali));
//     assert_eq!(t.balance_of(to_key(t.ali)), token_cfg::total_supply());
//     assert_eq!(t.allowance(to_key(t.ali), to_key(t.bob)), amount);
// }

// #[test]
// fn should_approve_erc20_to_zero_address() {
//     let amount = 10.into();
//     let mut t = Ownable::deployed("ERC20", "ERC");
//     let zero_key = Key::Account(AccountHash::default());
//     t.approve(zero_key, amount, Sender(t.ali));
// }

// #[test]
// fn should_transfer_erc20_from() {
//     let allowance = 10.into();
//     let amount = 3.into();
//     let mut t = Ownable::deployed("ERC20", "ERC");
//     t.approve(to_key(t.bob), allowance, Sender(t.ali));
//     assert_eq!(t.allowance(to_key(t.ali), to_key(t.bob)), allowance);
//     t.transfer_from(to_key(t.ali), to_key(t.joe), amount, Sender(t.bob));
//     assert_eq!(
//         t.balance_of(to_key(t.ali)),
//         token_cfg::total_supply() - amount
//     );
//     assert_eq!(t.balance_of(to_key(t.joe)), amount);
//     assert_eq!(
//         t.allowance(to_key(t.ali), to_key(t.bob)),
//         allowance - amount
//     );
// }

// #[test]
// #[should_panic = "65534"]
// fn should_not_transfer_from_too_much_erc20() {
//     let amount = token_cfg::total_supply().checked_add(1.into()).unwrap();
//     let mut t = Ownable::deployed("ERC20", "ERC");
//     t.approve(to_key(t.bob), amount, Sender(t.ali));
//     t.transfer_from(to_key(t.ali), to_key(t.joe), amount, Sender(t.bob));
// }

// #[test]
// fn should_transfer_erc20_to_zero_address() {
//     let amount = 1.into();
//     let mut t = Ownable::deployed("ERC20", "ERC");
//     let zero_key = Key::Account(AccountHash::default());
//     t.transfer(zero_key, amount, Sender(t.ali));
// }

// #[test]
// #[should_panic = "User(65533)"]
// fn should_not_transfer_from_erc20_without_approval() {
//     let amount = U256::from(1);
//     let mut t = Ownable::deployed("ERC20", "ERC");
//     t.transfer_from(to_key(t.ali), to_key(t.joe), amount, Sender(t.bob));
// }

// #[test]
// #[should_panic = "User(65533)"]
// fn should_not_transfer_from_erc20_with_low_allowance() {
//     let allowance = 1.into();
//     let amount = 3.into();
//     let mut t = Ownable::deployed("ERC20", "ERC");
//     t.approve(to_key(t.bob), allowance, Sender(t.ali));
//     t.transfer_from(to_key(t.ali), to_key(t.joe), amount, Sender(t.bob));
// }

// // ------------ START - Pair Tests ------------
// #[test]
// fn should_deploy_pair() {
//     let pair = Pair::deployed();
//     assert_eq!(pair.name(), pair.name);
//     assert_eq!(pair.symbol(), pair.symbol);
//     assert_eq!(pair.token0().value(), pair.token0_hash);
//     assert_eq!(pair.token1().value(), pair.token1_hash);
//     assert_eq!(pair.decimals(), pair_cfg::DECIMALS);
//     assert_eq!(pair.total_supply(), U256::from(0));
//     assert_eq!(pair.reserve0(), [0u8; 14]);
//     assert_eq!(pair.reserve1(), [0u8; 14]);
//     assert_eq!(pair.factory().value(), pair.factory_hash);
//     assert_eq!(pair.factory_package_hash().value(), pair.factory_package);
//     assert_eq!(pair.permit_typehash(), pair_cfg::PERMIT_TYPEHASH);
//     assert_eq!(pair.k_last(), U256::default());
//     assert_eq!(pair.price0_cumulative_last(), U256::default());
//     assert_eq!(pair.price1_cumulative_last(), U256::default());
//     assert_eq!(pair.block_timestamp_last(), u64::default());
//     assert_eq!(pair.selector(), pair_cfg::SELECTOR);
//     assert_eq!(
//         pair.balance_of(to_key(pair.owner)),
//         pair_cfg::total_supply()
//     );
//     assert_eq!(pair.nonce_of(to_key(pair.owner)), U256::default());
// }

// #[test]
// fn should_transfer_pair() {
//     let mut pair = Pair::deployed();
//     let amount = 10.into();
//     pair.transfer(
//         to_key(pair.token0_owner),
//         amount,
//         P_Sender(pair.owner),
//         pair.hash,
//     );
//     assert_eq!(
//         pair.balance_of(to_key(pair.owner)),
//         pair_cfg::total_supply() - amount
//     );
//     assert_eq!(pair.balance_of(to_key(pair.token0_owner)), amount);
// }

// #[test]
// #[should_panic = "65534"]
// fn should_not_transfer_too_much_pair() {
//     let amount = 1.into();
//     let mut pair = Pair::deployed();
//     pair.transfer(
//         to_key(pair.ali),
//         amount,
//         P_Sender(pair.token0_owner),
//         pair.hash,
//     );
// }

// #[test]
// fn should_approve_pair() {
//     let amount = 10.into();
//     let mut pair = Pair::deployed();
//     pair.approve(
//         to_key(pair.token0_owner),
//         amount,
//         P_Sender(pair.owner),
//         pair.hash,
//     );
//     assert_eq!(
//         pair.balance_of(to_key(pair.owner)),
//         pair_cfg::total_supply()
//     );
//     assert_eq!(
//         pair.allowance(to_key(pair.owner), to_key(pair.token0_owner)),
//         amount
//     );
// }

// #[test]
// #[should_panic = "User(65530)"]
// fn should_not_approve_pair_to_zero_address() {
//     let amount = 10.into();
//     let mut pair = Pair::deployed();
//     let zero_key = Key::Account(AccountHash::default());
//     pair.approve(zero_key, amount, P_Sender(pair.owner), pair.hash);
// }

// #[test]
// fn should_transfer_from_pair() {
//     let allowance = 10.into();
//     let amount = 3.into();
//     let mut pair = Pair::deployed();
//     pair.approve(
//         to_key(pair.token0_owner),
//         allowance,
//         P_Sender(pair.owner),
//         pair.hash,
//     );
//     assert_eq!(
//         pair.allowance(to_key(pair.owner), to_key(pair.token0_owner)),
//         allowance
//     );
//     pair.transfer_from(
//         to_key(pair.owner),
//         to_key(pair.token1_owner),
//         amount,
//         P_Sender(pair.token0_owner),
//         pair.hash,
//     );
//     assert_eq!(
//         pair.balance_of(to_key(pair.owner)),
//         pair_cfg::total_supply() - amount
//     );
//     assert_eq!(pair.balance_of(to_key(pair.token1_owner)), amount);
//     assert_eq!(
//         pair.allowance(to_key(pair.owner), to_key(pair.token0_owner)),
//         allowance - amount
//     );
// }

// #[test]
// #[should_panic = "65534"]
// fn should_not_transfer_from_too_much_pair() {
//     let amount = pair_cfg::total_supply().checked_add(1.into()).unwrap();
//     let mut pair = Pair::deployed();
//     pair.approve(
//         to_key(pair.token0_owner),
//         amount,
//         P_Sender(pair.owner),
//         pair.hash,
//     );
//     pair.transfer_from(
//         to_key(pair.owner),
//         to_key(pair.token1_owner),
//         amount,
//         P_Sender(pair.token0_owner),
//         pair.hash,
//     );
// }

// #[test]
// fn should_transfer_lp_to_zero_address() {
//     let amount = 1.into();
//     let mut pair = Pair::deployed();
//     let zero_key = Key::Account(AccountHash::default());

//     pair.transfer(zero_key, amount, P_Sender(pair.owner), pair.hash);

//     assert_eq!(
//         pair.balance_of(to_key(pair.owner)),
//         pair_cfg::total_supply() - amount
//     );
//     assert_eq!(pair.balance_of(zero_key), amount);
// }

// #[test]
// #[should_panic = "User(65533)"]
// fn should_not_transfer_from_pair_without_approval() {
//     let amount = U256::from(1);
//     let mut pair = Pair::deployed();
//     pair.transfer_from(
//         to_key(pair.owner),
//         to_key(pair.token1_owner),
//         amount,
//         P_Sender(pair.token0_owner),
//         pair.hash,
//     );
// }

// #[test]
// #[should_panic = "User(65533)"]
// fn should_not_transfer_from_pair_with_low_allowance() {
//     let allowance = 1.into();
//     let amount = 3.into();
//     let mut pair = Pair::deployed();
//     pair.approve(
//         to_key(pair.token0_owner),
//         allowance,
//         P_Sender(pair.owner),
//         pair.hash,
//     );
//     pair.transfer_from(
//         to_key(pair.owner),
//         to_key(pair.token1_owner),
//         amount,
//         P_Sender(pair.token0_owner),
//         pair.hash,
//     );
// }

// #[test]
// fn should_permit_pair_using_edd25519() {
//     let mut pair = Pair::deployed();
//     let value = U256::from(10 * 18);
//     let nonce = pair.nonces(to_key(pair.owner));
//     let deadline = u64::MAX;
//     let digest = get_pair_approval_digest(
//         &pair,
//         to_key(pair.owner),
//         to_key(pair.token1_owner),
//         value,
//         nonce + U256::from(1),
//         deadline,
//     );
//     let mut csprng = OsRng {};
//     let keypair: Keypair = Keypair::generate(&mut csprng);
//     let signature: EdSignature = keypair.sign(&digest);
//     let signature_as_bytes = signature.to_bytes();
//     let (r, s) = signature_as_bytes.split_at(32);
//     let pubkey_as_bytes = keypair.public.as_bytes();
//     pair.permit(
//         PublicKey::ed25519_from_bytes(pubkey_as_bytes).unwrap(),
//         to_key(pair.owner),
//         to_key(pair.token1_owner),
//         value,
//         deadline,
//         U256::from_big_endian(&set_size_32(r)),
//         U256::from_big_endian(&set_size_32(s)),
//         P_Sender(pair.owner),
//         pair.hash,
//     );
//     assert_eq!(
//         pair.allowance(to_key(pair.owner), to_key(pair.token1_owner)),
//         value
//     );
//     assert_eq!(pair.nonces(to_key(pair.owner)), nonce + U256::from(1));
// }

// #[test]
// fn should_permit_pair_using_secp256k1() {
//     let mut pair = Pair::deployed();
//     let value = U256::from(10 * 18);
//     let nonce = pair.nonces(to_key(pair.owner));
//     let deadline = u64::MAX;
//     let digest = get_pair_approval_digest(
//         &pair,
//         to_key(pair.owner),
//         to_key(pair.token1_owner),
//         value,
//         nonce + U256::from(1),
//         deadline,
//     );
//     let mut csprng = OsRng {};
//     let signing_key = SigningKey::random(&mut csprng);
//     let verify_key = VerifyingKey::from(&signing_key);
//     let signature: Signature = signing_key.sign(&digest);
//     let signature_as_ref = signature.as_ref();
//     let (r, s) = signature_as_ref.split_at(32);
//     let (s, _v) = s.split_at(32);
//     let pubkey_as_bytes = verify_key.to_bytes();
//     pair.permit(
//         PublicKey::secp256k1_from_bytes(pubkey_as_bytes).unwrap(),
//         to_key(pair.owner),
//         to_key(pair.token1_owner),
//         value,
//         deadline,
//         U256::from_big_endian(&set_size_32(r)),
//         U256::from_big_endian(&set_size_32(s)),
//         P_Sender(pair.owner),
//         pair.hash,
//     );
//     assert_eq!(
//         pair.allowance(to_key(pair.owner), to_key(pair.token1_owner)),
//         value
//     );
//     assert_eq!(pair.nonces(to_key(pair.owner)), nonce + U256::from(1));
// }

// fn add_liquidity(mut pair: Pair, amount0: U256, amount1: U256) -> Pair {
//     pair.transfer(
//         Key::Hash(pair.package_hash),
//         amount0,
//         P_Sender(pair.token0_owner),
//         pair.token0_hash,
//     );
//     pair.transfer(
//         Key::Hash(pair.package_hash),
//         amount1,
//         P_Sender(pair.token1_owner),
//         pair.token1_hash,
//     );
//     pair.mint(to_key(pair.factory_owner), P_Sender(pair.owner));
//     pair
// }

// #[test]
// fn should_mint_pair() {
//     let amount0 = expand_to_18_decimals(1.into());
//     let amount1 = expand_to_18_decimals(4.into());
//     let expected_liquidity = expand_to_18_decimals(2.into());
//     let default_account = AccountHash::default();
//     let mut pair = Pair::deployed();
//     pair.transfer(
//         Key::Hash(pair.package_hash),
//         amount0,
//         P_Sender(pair.token0_owner),
//         pair.token0_hash,
//     );
//     pair.transfer(
//         Key::Hash(pair.package_hash),
//         amount1,
//         P_Sender(pair.token1_owner),
//         pair.token1_hash,
//     );
//     assert_eq!(
//         pair.token_balance_of(Key::Hash(pair.package_hash), pair.token0_hash),
//         amount0
//     );
//     assert_eq!(
//         pair.token_balance_of(Key::Hash(pair.package_hash), pair.token1_hash),
//         amount1
//     );
//     pair.mint(to_key(pair.factory_owner), P_Sender(pair.owner));
//     assert_eq!(
//         pair.balance_of(to_key(default_account)),
//         pair_cfg::minimum_liquidity()
//     );
//     assert_eq!(
//         pair.balance_of(to_key(pair.factory_owner)),
//         expected_liquidity
//             .checked_sub(pair_cfg::minimum_liquidity())
//             .unwrap()
//     );
//     assert_eq!(pair.total_supply(), expected_liquidity);
//     assert_eq!(
//         pair.token_balance_of(Key::Hash(pair.package_hash), pair.token0_hash),
//         amount0
//     );
//     assert_eq!(
//         pair.token_balance_of(Key::Hash(pair.package_hash), pair.token1_hash),
//         amount1
//     );
//     let reserves = pair.get_reserves();
//     assert_eq!(U256::from_big_endian(&reserves.0), amount0);
//     assert_eq!(U256::from_big_endian(&reserves.1), amount1);
// }

// #[test]
// #[should_panic = "User(65517)"]
// fn should_not_mint_pair_with_insufficient_liquidity() {
//     let amount = expand_to_18_decimals(1.into());
//     let mut pair = Pair::deployed();
//     pair = add_liquidity(pair, amount, amount);
//     // currently amount0 and amount1 are 0 since balance0 = reserve0 & balance1 = reserve1 = amount
//     // so liquidity will be equal to 0 since total_supply != 0
//     pair.mint(to_key(pair.factory_owner), P_Sender(pair.owner));
// }

// #[test]
// fn should_burn_pair() {
//     let amount = expand_to_18_decimals(3.into());
//     let expected_liquidity = expand_to_18_decimals(3.into());
//     let default_account = AccountHash::default();
//     let mut pair = Pair::deployed();
//     pair = add_liquidity(pair, amount, amount);
//     // ############## BURN #############
//     // we need to transfer to pair some tokens
//     pair.transfer(
//         Key::Hash(pair.package_hash),
//         expected_liquidity
//             .checked_sub(pair_cfg::minimum_liquidity())
//             .unwrap(),
//         P_Sender(pair.factory_owner),
//         pair.hash,
//     );
//     pair.burn(to_key(pair.factory_owner), P_Sender(pair.owner));
//     assert_eq!(
//         pair.balance_of(to_key(default_account)),
//         pair_cfg::minimum_liquidity()
//     );
//     assert_eq!(pair.balance_of(to_key(pair.factory_owner)), U256::from(0));
//     assert_eq!(pair.total_supply(), pair_cfg::minimum_liquidity());
//     assert_eq!(
//         pair.token_balance_of(Key::Hash(pair.package_hash), pair.token0_hash),
//         pair_cfg::minimum_liquidity()
//     );
//     assert_eq!(
//         pair.token_balance_of(Key::Hash(pair.package_hash), pair.token1_hash),
//         pair_cfg::minimum_liquidity()
//     );
//     assert_eq!(
//         pair.token_balance_of(to_key(pair.factory_owner), pair.token0_hash),
//         amount.checked_sub(U256::from(1000)).unwrap()
//     );
//     assert_eq!(
//         pair.token_balance_of(to_key(pair.factory_owner), pair.token1_hash),
//         amount.checked_sub(U256::from(1000)).unwrap()
//     );
// }

// #[test]
// #[should_panic = "User(65516)"]
// fn should_not_burn_pair_with_insufficient_liquidity() {
//     let amount = expand_to_18_decimals(1.into());
//     let mut pair = Pair::deployed();
//     pair = add_liquidity(pair, amount, amount);
//     // ############## BURN #############
//     pair.burn(to_key(pair.factory_owner), P_Sender(pair.owner));
// }

// #[test]
// fn should_swap_token0() {
//     let amount0 = expand_to_18_decimals(5.into());
//     let amount1 = expand_to_18_decimals(10.into());
//     let swap_amount = expand_to_18_decimals(1.into());
//     let expected_output_amount = U256::from(1662497915624478906u64);
//     let mut pair = Pair::deployed();
//     pair = add_liquidity(pair, amount0, amount1);
//     pair.transfer(
//         Key::Hash(pair.package_hash),
//         swap_amount,
//         P_Sender(pair.token0_owner),
//         pair.token0().value(),
//     );
//     pair.swap(
//         0.into(),
//         expected_output_amount,
//         to_key(pair.factory_owner),
//         P_Sender(pair.owner),
//     );
//     let reserves = pair.get_reserves();
//     assert_eq!(
//         U256::from_big_endian(&reserves.0),
//         amount0.checked_add(swap_amount).unwrap()
//     );
//     assert_eq!(
//         U256::from_big_endian(&reserves.1),
//         amount1.checked_sub(expected_output_amount).unwrap()
//     );
//     assert_eq!(
//         pair.token_balance_of(Key::Hash(pair.package_hash), pair.token0_hash),
//         amount0.checked_add(swap_amount).unwrap()
//     );
//     assert_eq!(
//         pair.token_balance_of(Key::Hash(pair.package_hash), pair.token1_hash),
//         amount1.checked_sub(expected_output_amount).unwrap()
//     );
//     let total_supply_token_0 = pair.token_total_supply(pair.token0_owner);
//     assert_eq!(
//         pair.token_balance_of(to_key(pair.token0_owner), pair.token0_hash),
//         total_supply_token_0
//             .checked_sub(amount0)
//             .unwrap()
//             .checked_sub(swap_amount)
//             .unwrap()
//     );
//     assert_eq!(
//         pair.token_balance_of(to_key(pair.factory_owner), pair.token1_hash),
//         expected_output_amount
//     );
// }

// #[test]
// fn should_swap_token1() {
//     let amount0 = expand_to_18_decimals(5.into());
//     let amount1 = expand_to_18_decimals(10.into());
//     let swap_amount = expand_to_18_decimals(1.into());
//     let expected_output_amount = U256::from(453305446940074565u64);
//     let mut pair = Pair::deployed();
//     pair = add_liquidity(pair, amount0, amount1);
//     pair.transfer(
//         Key::Hash(pair.package_hash),
//         swap_amount,
//         P_Sender(pair.token1_owner),
//         pair.token1().value(),
//     );
//     pair.swap(
//         expected_output_amount,
//         0.into(),
//         to_key(pair.factory_owner),
//         P_Sender(pair.owner),
//     );
//     let reserves = pair.get_reserves();
//     assert_eq!(
//         U256::from_big_endian(&reserves.0),
//         amount0.checked_sub(expected_output_amount).unwrap()
//     );
//     assert_eq!(
//         U256::from_big_endian(&reserves.1),
//         amount1.checked_add(swap_amount).unwrap()
//     );
//     assert_eq!(
//         pair.token_balance_of(Key::Hash(pair.package_hash), pair.token0_hash),
//         amount0.checked_sub(expected_output_amount).unwrap()
//     );
//     assert_eq!(
//         pair.token_balance_of(Key::Hash(pair.package_hash), pair.token1_hash),
//         amount1.checked_add(swap_amount).unwrap()
//     );
//     let total_supply_token_1 = pair.token_total_supply(pair.token1_owner);
//     assert_eq!(
//         pair.token_balance_of(to_key(pair.factory_owner), pair.token0_hash),
//         expected_output_amount
//     );
//     assert_eq!(
//         pair.token_balance_of(to_key(pair.token1_owner), pair.token1_hash),
//         total_supply_token_1
//             .checked_sub(amount1)
//             .unwrap()
//             .checked_sub(swap_amount)
//             .unwrap()
//     );
// }

// #[test]
// fn should_update_price_cumulative_last() {
//     let amount0 = expand_to_18_decimals(3.into());
//     let amount1 = expand_to_18_decimals(3.into());
//     let mut pair = Pair::deployed();
//     pair = add_liquidity(pair, amount0, amount1);

//     increment_block_timestamp(1);
//     pair.sync(P_Sender(pair.owner));
//     let initial_price = encode_price(amount0, amount1);
//     assert_eq!(pair.price0_cumulative_last(), initial_price.0);
//     assert_eq!(pair.price1_cumulative_last(), initial_price.1);

//     let swap_amount = expand_to_18_decimals(3.into());
//     pair.transfer(
//         Key::Hash(pair.package_hash),
//         swap_amount,
//         P_Sender(pair.token0_owner),
//         pair.token0().value(),
//     );
//     increment_block_timestamp(1);
//     pair.swap(
//         0.into(),
//         expand_to_18_decimals(1.into()),
//         to_key(pair.factory_owner),
//         P_Sender(pair.owner),
//     );
//     assert_eq!(
//         pair.price0_cumulative_last(),
//         initial_price.0.checked_mul(U256::from(2)).unwrap()
//     );
//     assert_eq!(
//         pair.price1_cumulative_last(),
//         initial_price.1.checked_mul(U256::from(2)).unwrap()
//     );

//     increment_block_timestamp(2);
//     pair.sync(P_Sender(pair.owner));
//     let new_price = encode_price(
//         expand_to_18_decimals(6.into()),
//         expand_to_18_decimals(2.into()),
//     );
//     assert_eq!(
//         pair.price0_cumulative_last(),
//         initial_price
//             .0
//             .checked_mul(U256::from(2))
//             .unwrap()
//             .checked_add(new_price.0)
//             .unwrap()
//     );
//     assert_eq!(
//         pair.price1_cumulative_last(),
//         initial_price
//             .1
//             .checked_mul(U256::from(2))
//             .unwrap()
//             .checked_add(new_price.1)
//             .unwrap()
//     );
// }

// #[test]
// fn test_fee_to_off() {
//     let amount0 = expand_to_18_decimals(1000.into());
//     let amount1 = expand_to_18_decimals(1000.into());
//     let mut pair = Pair::deployed();
//     pair = add_liquidity(pair, amount0, amount1);

//     let swap_amount = expand_to_18_decimals(1.into());
//     let expected_output_amount = U256::from(996006981039903216u64);
//     pair.transfer(
//         Key::Hash(pair.package_hash),
//         swap_amount,
//         P_Sender(pair.token1_owner),
//         pair.token1().value(),
//     );
//     pair.swap(
//         expected_output_amount,
//         0.into(),
//         to_key(pair.factory_owner),
//         P_Sender(pair.owner),
//     );

//     let expected_liquidity = expand_to_18_decimals(1000.into());
//     pair.transfer(
//         Key::Hash(pair.package_hash),
//         expected_liquidity
//             .checked_sub(pair_cfg::minimum_liquidity())
//             .unwrap(),
//         P_Sender(pair.factory_owner),
//         pair.hash,
//     );
//     pair.burn(to_key(pair.factory_owner), P_Sender(pair.owner));
//     assert_eq!(pair.total_supply(), pair_cfg::minimum_liquidity());
// }

// #[test]
// fn test_fee_to_on() {
//     let amount0 = expand_to_18_decimals(1000.into());
//     let amount1 = expand_to_18_decimals(1000.into());
//     let mut pair = Pair::deployed();
//     pair.factory_set_fee_to(pair.token0_owner.into(), P_Sender(pair.factory_owner));
//     pair = add_liquidity(pair, amount0, amount1);

//     let swap_amount = expand_to_18_decimals(1.into());
//     let expected_output_amount = U256::from(996006981039903216u64);
//     pair.transfer(
//         Key::Hash(pair.package_hash),
//         swap_amount,
//         P_Sender(pair.token1_owner),
//         pair.token1().value(),
//     );
//     pair.swap(
//         expected_output_amount,
//         0.into(),
//         to_key(pair.factory_owner),
//         P_Sender(pair.owner),
//     );

//     let expected_liquidity = expand_to_18_decimals(1000.into());
//     pair.transfer(
//         Key::Hash(pair.package_hash),
//         expected_liquidity
//             .checked_sub(pair_cfg::minimum_liquidity())
//             .unwrap(),
//         P_Sender(pair.factory_owner),
//         pair.hash,
//     );
//     pair.burn(to_key(pair.factory_owner), P_Sender(pair.owner));
//     assert_eq!(
//         pair.total_supply(),
//         pair_cfg::minimum_liquidity()
//             .checked_add(U256::from(249750499251388u64))
//             .unwrap()
//     );
//     assert_eq!(
//         pair.balance_of(to_key(pair.token0_owner)),
//         U256::from(249750499251388u64)
//     );

//     // using 1000 here instead of the symbolic MINIMUM_LIQUIDITY because the amounts only happen to be equal...
//     // ...because the initial liquidity amounts were equal
//     assert_eq!(
//         pair.token_balance_of(Key::Hash(pair.package_hash), pair.token0_hash),
//         U256::from(1000)
//             .checked_add(U256::from(249501683697445u64))
//             .unwrap()
//     );
//     assert_eq!(
//         pair.token_balance_of(Key::Hash(pair.package_hash), pair.token1_hash),
//         U256::from(1000)
//             .checked_add(U256::from(250000187312969u64))
//             .unwrap()
//     );
// }

// #[test]
// #[should_panic = "User(65518)"]
// fn should_not_set_emergency_mode() {
//     let mut pair = Pair::deployed();

//     pair.set_emergency_mode(true, P_Sender(pair.owner));
// }

// #[test]
// #[should_panic = "User(65508)"]
// fn should_not_call_emergency_withdraw() {
//     let mut pair = Pair::deployed();

//     assert!(!pair.emergency_mode());

//     pair.emergency_withdraw(U256::one(), P_Sender(pair.owner));
// }

// #[test]
// fn should_call_emergency_withdraw() {
//     let mut pair = Pair::deployed();

//     let amount0 = expand_to_18_decimals(1000.into());
//     let amount1 = expand_to_18_decimals(1000.into());

//     pair = add_liquidity(pair, amount0, amount1);

//     assert!(!pair.emergency_mode());

//     pair.trigger_emergency_mode_for_pair(
//         Key::Hash(pair.package_hash),
//         1u32,
//         true,
//         P_Sender(pair.factory_owner),
//     );

//     assert!(pair.emergency_mode());

//     let user_lp_tokens: U256 = pair.balance_of(pair.factory_owner.into());
//     let user_token0_balance: U256 =
//         pair.token_balance_of(pair.factory_owner.into(), pair.token0_hash);
//     let user_token1_balance: U256 =
//         pair.token_balance_of(pair.factory_owner.into(), pair.token1_hash);

//     let pair_token0_balance: U256 =
//         pair.token_balance_of(Key::Hash(pair.package_hash), pair.token0_hash);
//     let pair_token1_balance: U256 =
//         pair.token_balance_of(Key::Hash(pair.package_hash), pair.token1_hash);
//     let expected_liquidity: U256 = pair
//         .balance_of(Key::Hash(pair.package_hash))
//         .checked_add(user_lp_tokens)
//         .unwrap();

//     let pair_total_supply: U256 = pair.total_supply();

//     let expected_amount0_out: U256 =
//         get_burn_amount(expected_liquidity, pair_token0_balance, pair_total_supply);
//     let expected_amount1_out: U256 =
//         get_burn_amount(expected_liquidity, pair_token1_balance, pair_total_supply);

//     pair.emergency_withdraw(user_lp_tokens, P_Sender(pair.factory_owner));

//     let user_token0_new_balance: U256 =
//         pair.token_balance_of(pair.factory_owner.into(), pair.token0_hash);
//     let user_token1_new_balance: U256 =
//         pair.token_balance_of(pair.factory_owner.into(), pair.token1_hash);

//     assert_eq!(pair.total_supply(), pair_cfg::minimum_liquidity());
//     assert_eq!(pair.balance_of(pair.factory_owner.into()), U256::zero());
//     assert_eq!(
//         user_token0_new_balance,
//         user_token0_balance
//             .checked_add(expected_amount0_out)
//             .unwrap(),
//         "user's token0 balance should be incremented after emergency_withdraw()"
//     );
//     assert_eq!(
//         user_token1_new_balance,
//         user_token1_balance
//             .checked_add(expected_amount1_out)
//             .unwrap(),
//         "user's token1 balance should be incremented after emergency_withdraw()"
//     );
// }

// #[test]
// #[should_panic = "User(65508)"]
// fn should_not_call_restricted_functions_in_emergency() {
//     let mut pair = Pair::deployed();

//     assert!(!pair.emergency_mode());

//     pair.trigger_emergency_mode_for_pair(
//         Key::Hash(pair.package_hash),
//         1u32,
//         true,
//         P_Sender(pair.factory_owner),
//     );

//     assert!(pair.emergency_mode());

//     pair.burn(to_key(pair.factory_owner), P_Sender(pair.owner));
// }

// // ------------ START - Factory Tests ------------
// #[test]
// fn should_deploy_factory() {
//     let f = Factory::setup();
//     assert_eq!(f.fee_to_setter(), f.owner.into());
// }

// #[test]
// fn should_set_fee_to() {
//     let mut f = Factory::setup();
//     f.set_fee_to(f.ali.into(), F_Sender(f.owner));
//     assert_eq!(f.fee_to(), f.ali.into());
// }

// #[test]
// #[should_panic = "User(65518)"]
// fn should_not_set_fee_to_when_unauthorized() {
//     let mut f = Factory::setup();
//     f.set_fee_to(f.ali.into(), F_Sender(f.ali));
// }

// #[test]
// fn should_set_fee_to_setter() {
//     let mut f = Factory::setup();
//     f.set_fee_to_setter(f.ali.into(), F_Sender(f.owner));
//     assert_eq!(f.fee_to_setter(), f.ali.into());
// }

// #[test]
// #[should_panic = "User(65518)"]
// fn should_not_set_fee_to_setter_when_unauthorized() {
//     let mut f = Factory::setup();
//     f.set_fee_to_setter(f.ali.into(), F_Sender(f.ali));
// }

// #[test]
// fn should_create_pair() {
//     let mut f = Factory::setup();
//     f.create_pair(
//         Key::Hash(f.token0_hash),
//         Key::Hash(f.token1_hash),
//         F_Sender(f.owner),
//     );
//     assert_eq!(f.all_pairs().len(), 1);
//     let pair = f.all_pairs().pop().unwrap();
//     assert_eq!(
//         f.pair_of(
//             ContractHash::new(f.token0_hash),
//             ContractHash::new(f.token1_hash)
//         ),
//         (pair, 1u32)
//     );
//     assert_eq!(
//         f.pair_of(
//             ContractHash::new(f.token1_hash),
//             ContractHash::new(f.token0_hash)
//         ),
//         (pair, 1u32)
//     );
// }

// #[test]
// #[should_panic = "User(65519)"]
// fn should_not_create_an_existing_pair() {
//     let mut f = Factory::setup();
//     f.create_pair(
//         Key::Hash(f.token0_hash),
//         Key::Hash(f.token1_hash),
//         F_Sender(f.owner),
//     );
//     f.create_pair(
//         Key::Hash(f.token1_hash),
//         Key::Hash(f.token0_hash),
//         F_Sender(f.owner),
//     );
// }

// #[test]
// #[should_panic = "User(65518)"]
// fn should_not_trigger_emergency_mode_for_pair_when_unauthorized() {
//     let mut f = Factory::setup();
//     f.trigger_emergency_mode_for_pair(
//         Key::Hash(ContractPackageHash::default().value()),
//         1u32,
//         true,
//         F_Sender(f.ali),
//     );
// }

// #[test]
// fn should_trigger_emergency_mode_for_pair() {
//     let mut f = Factory::setup();

//     f.create_pair(
//         Key::Hash(f.token0_hash),
//         Key::Hash(f.token1_hash),
//         F_Sender(f.owner),
//     );

//     assert_eq!(f.all_pairs().len(), 1);

//     let pair = f.all_pairs().pop().unwrap();

//     assert_eq!(
//         f.pair_of(
//             ContractHash::new(f.token0_hash),
//             ContractHash::new(f.token1_hash)
//         ),
//         (pair, 1u32)
//     );
//     assert_eq!(
//         f.pair_of(
//             ContractHash::new(f.token1_hash),
//             ContractHash::new(f.token0_hash)
//         ),
//         (pair, 1u32)
//     );

//     f.trigger_emergency_mode_for_pair(pair.into(), 1u32, true, F_Sender(f.owner));
// }
