use crate::utilities::{key_to_str, CasperHelper, Hash, Sender};

use casper_types::{
    account::AccountHash, runtime_args, CLTyped, ContractHash, Key, Motes, RuntimeArgs, U256,
};
use libs::token::erc20::ERC20;
use std::path::PathBuf;

pub mod token_cfg {
    use super::*;
    pub const NAME: &str = "ERC20";
    pub const SYMBOL: &str = "ERC";
    pub const DECIMALS: u8 = 8;
    pub fn total_supply() -> U256 {
        U256::from("1000000000000000")
    }
}

// contains methods that can simulate a real-world deployment (storing the contract in the blockchain)
// and transactions to invoke the methods in the contract.
pub const ERC20_TOKEN_CONTRACT_KEY_NAME: &str = "Erc20";
pub struct Erc20 {
    pub helper: CasperHelper,
    pub hash: Hash,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
}

impl Erc20 {
    pub fn deployed(name: String, symbol: String, decimals: u8, total_supply: U256) -> Erc20 {
        let mut helper = CasperHelper::new();

        // ====================== CONTRACT DEPLOYMENT ======================
        let session_code = PathBuf::from("erc20.wasm");
        let session_args = runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "total_supply" => total_supply,
        };

        let hash = helper.deploy_contract(
            session_code,
            session_args,
            helper.keys[0].clone(),
            ERC20_TOKEN_CONTRACT_KEY_NAME.to_string(),
        );

        // ====================== FUNCTION RETURN ======================
        Erc20 {
            helper: helper.clone(),
            hash,
            ali: helper.accounts[0].clone(),
            bob: helper.accounts[1].clone(),
            joe: helper.accounts[2].clone(),
        }
    }

    pub fn name(&self) -> String {
        self.helper
            .query_contract(
                ERC20_TOKEN_CONTRACT_KEY_NAME.to_string(),
                ERC20::ERC20_NAME_KEY,
            )
            .unwrap()
    }

    pub fn symbol(&self) -> String {
        self.helper
            .query_contract(
                ERC20_TOKEN_CONTRACT_KEY_NAME.to_string(),
                ERC20::ERC20_SYMBOL_KEY,
            )
            .unwrap()
    }

    pub fn decimals(&self) -> u8 {
        self.helper
            .query_contract(
                ERC20_TOKEN_CONTRACT_KEY_NAME.to_string(),
                ERC20::ERC20_DECIMALS_KEY,
            )
            .unwrap()
    }

    pub fn total_supply(&self) -> U256 {
        self.helper
            .query_contract(
                ERC20_TOKEN_CONTRACT_KEY_NAME.to_string(),
                ERC20::ERC20_TOTAL_SUPPLY_KEY,
            )
            .unwrap()
    }

    pub fn balance_of(&self, account: Key) -> U256 {
        self.helper
            .query_dictionary_value(self.hash, ERC20::ERC20_BALANCE_KEY, key_to_str(&account))
            .unwrap_or_default()
    }
}
