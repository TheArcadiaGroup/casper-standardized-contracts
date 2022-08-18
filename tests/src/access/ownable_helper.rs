use crate::utilities::{CasperHelper, Hash, Sender};
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs};
use std::path::PathBuf;

// contains methods that can simulate a real-world deployment (storing the contract in the blockchain)
// and transactions to invoke the methods in the contract.
pub const OWNABLE_CONTRACT_KEY_NAME: &str = "Ownable";

pub struct Ownable {
    pub helper: CasperHelper,
    pub hash: Hash,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
}

impl Ownable {
    pub fn deployed() -> Ownable {
        let mut helper = CasperHelper::new();

        // ====================== CONTRACT DEPLOYMENT ======================
        let session_code = PathBuf::from("ownable.wasm");
        let session_args = runtime_args! {
            "owner" => Key::Account(helper.accounts[0])
        };

        let hash = helper.deploy_contract(
            session_code,
            session_args,
            helper.keys[0].clone(),
            OWNABLE_CONTRACT_KEY_NAME.to_string(),
        );

        // ====================== FUNCTION RETURN ======================
        Ownable {
            helper: helper.clone(),
            hash,
            ali: helper.accounts[0].clone(),
            bob: helper.accounts[1].clone(),
            joe: helper.accounts[2].clone(),
        }
    }

    pub fn owner(&self) -> Key {
        self.helper
            .query_contract(OWNABLE_CONTRACT_KEY_NAME.to_string(), "owner")
            .unwrap()
    }

    pub fn transfer_ownership(&mut self, new_owner: Key, sender: Sender) {
        self.helper.call(
            self.hash,
            sender,
            "transfer_ownership",
            runtime_args! {
                "new_owner" => new_owner
            },
        );
    }

    pub fn renounce_ownership(&mut self, sender: Sender) {
        self.helper
            .call(self.hash, sender, "renounce_ownership", runtime_args! {});
    }
}
