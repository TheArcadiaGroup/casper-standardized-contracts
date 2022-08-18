use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::{
    DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT,
    DEFAULT_ACCOUNT_INITIAL_BALANCE, DEFAULT_ACCOUNT_PUBLIC_KEY, DEFAULT_AUCTION_DELAY,
    DEFAULT_GENESIS_CONFIG_HASH, DEFAULT_GENESIS_TIMESTAMP_MILLIS,
    DEFAULT_LOCKED_FUNDS_PERIOD_MILLIS, DEFAULT_PAYMENT, DEFAULT_PROPOSER_PUBLIC_KEY,
    DEFAULT_PROTOCOL_VERSION, DEFAULT_ROUND_SEIGNIORAGE_RATE, DEFAULT_SYSTEM_CONFIG,
    DEFAULT_UNBONDING_DELAY, DEFAULT_VALIDATOR_SLOTS, DEFAULT_WASM_CONFIG,
};
use casper_execution_engine::core::engine_state::{
    genesis::{ExecConfig, GenesisAccount},
    run_genesis_request::RunGenesisRequest,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, CLTyped, ContractHash, Key, Motes, PublicKey, RuntimeArgs, SecretKey,
    StoredValue,
};
use rand::Rng;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

pub fn blake2b256(item_key_string: &[u8]) -> Box<[u8]> {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(item_key_string);
    hasher.finalize_boxed()
}

pub fn key_to_str(key: &Key) -> String {
    let preimage = key.to_bytes().unwrap();
    base64::encode(&preimage)
}

pub fn two_key_to_str(key1: Key, key2: Key) -> String {
    let mut preimage = Vec::new();
    preimage.append(&mut key1.to_bytes().unwrap());
    preimage.append(&mut key2.to_bytes().unwrap());
    let key_bytes = blake2b256(&preimage);
    hex::encode(&key_bytes)
}

pub fn to_key(account: AccountHash) -> Key {
    Key::Account(account)
}

pub struct Sender(pub AccountHash);
pub type Hash = [u8; 32];

#[derive(Clone)]
pub struct CasperHelper {
    pub builder: InMemoryWasmTestBuilder,
    pub keys: Vec<PublicKey>,
    pub accounts: Vec<AccountHash>,
}

impl CasperHelper {
    pub fn new() -> CasperHelper {
        let mut keys: Vec<PublicKey> = vec![];
        for i in 1..11 {
            keys.push(PublicKey::from(
                &SecretKey::ed25519_from_bytes([i; 32]).unwrap(),
            ))
        }

        // ====================== BLOCKCHAIN SETUP ======================
        // create our WasmBuilder.
        let mut builder = InMemoryWasmTestBuilder::default();

        // initialize the blockchain network to get our first block.

        // implement custom accounts.
        let genesis_accounts: Vec<GenesisAccount> = CasperHelper::set_custom_accounts(keys.clone());

        // implement custom exec config.
        let custom_exec_config: ExecConfig = CasperHelper::set_custom_exec_config(genesis_accounts);

        // implement custom run genesis request.
        let custom_run_genesis_request: RunGenesisRequest =
            CasperHelper::set_custom_run_genesis_request(custom_exec_config);

        // run genesis request with the custom exec config.
        builder.run_genesis(&custom_run_genesis_request).commit();

        CasperHelper {
            builder,
            keys: keys.clone(),
            accounts: keys.iter().map(|x| x.to_account_hash()).collect(),
        }
    }

    /// Creates a vector of [`GenesisAccount`] out of a vector of [`PublicKey`].
    pub fn set_custom_accounts(public_keys: Vec<PublicKey>) -> Vec<GenesisAccount> {
        let mut genesis_accounts = Vec::new();

        // add default and proposer accounts.
        let genesis_account = GenesisAccount::account(
            DEFAULT_ACCOUNT_PUBLIC_KEY.clone(),
            Motes::new(DEFAULT_ACCOUNT_INITIAL_BALANCE.into()),
            None,
        );
        genesis_accounts.push(genesis_account);
        let proposer_account = GenesisAccount::account(
            DEFAULT_PROPOSER_PUBLIC_KEY.clone(),
            Motes::new(DEFAULT_ACCOUNT_INITIAL_BALANCE.into()),
            None,
        );
        genesis_accounts.push(proposer_account);

        // add custom accounts.
        for public_key in public_keys {
            let genesis_account = GenesisAccount::account(
                public_key.clone(),
                Motes::new(DEFAULT_ACCOUNT_INITIAL_BALANCE.into()),
                None,
            );
            genesis_accounts.push(genesis_account);
        }
        genesis_accounts
    }

    /// Creates an [`ExecConfig`] out of the given `genesis_accounts`
    /// and uses default values for the other params.
    pub fn set_custom_exec_config(genesis_accounts: Vec<GenesisAccount>) -> ExecConfig {
        ExecConfig::new(
            genesis_accounts,
            *DEFAULT_WASM_CONFIG,
            *DEFAULT_SYSTEM_CONFIG,
            DEFAULT_VALIDATOR_SLOTS,
            DEFAULT_AUCTION_DELAY,
            DEFAULT_LOCKED_FUNDS_PERIOD_MILLIS,
            DEFAULT_ROUND_SEIGNIORAGE_RATE,
            DEFAULT_UNBONDING_DELAY,
            DEFAULT_GENESIS_TIMESTAMP_MILLIS,
        )
    }

    /// Creates a new [`RunGenesisRequest`] given a custom [`ExecConfig`].
    pub fn set_custom_run_genesis_request(custom_exec_config: ExecConfig) -> RunGenesisRequest {
        RunGenesisRequest::new(
            *DEFAULT_GENESIS_CONFIG_HASH,
            *DEFAULT_PROTOCOL_VERSION,
            custom_exec_config,
        )
    }

    /// Deploys a contract and returns the `contract_hash` and the updated `builder`.
    pub fn deploy_contract(
        &mut self,
        session_code: PathBuf,
        session_args: RuntimeArgs,
        deployer: PublicKey,
        contract_hash_key: String,
    ) -> Hash {
        let mut rng = rand::thread_rng();

        let deploy_item = DeployItemBuilder::new()
            // .with_payment_bytes(module_bytes, args)
            .with_empty_payment_bytes(runtime_args! {
                ARG_AMOUNT => *DEFAULT_PAYMENT
            })
            .with_session_code(session_code, session_args)
            .with_deploy_hash(rng.gen())
            .with_authorization_keys(&[deployer.to_account_hash()])
            .with_address(deployer.to_account_hash())
            .build();

        // prepare the execute request.
        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item)
            .with_block_time(get_current_time())
            .build();

        // pre-assertion before the contract deployment.
        let contract_hash = self.builder.query(
            None,
            Key::Account(deployer.to_account_hash()),
            &[contract_hash_key.clone()],
        );

        assert!(contract_hash.is_err());

        // deploy the contract.
        self.builder.exec(execute_request).commit().expect_success();

        // retrieving hashes & post-assertions after the contract deployment.
        let contract_hash = self
            .builder
            .get_account(deployer.to_account_hash())
            .expect("should have account")
            .named_keys()
            .get(&contract_hash_key)
            .and_then(|key| key.into_hash())
            .map(ContractHash::new)
            .expect("should have contract hash")
            .value();

        assert_ne!(contract_hash, [0u8; 32]);

        contract_hash
    }

    /// query a contract's named key.
    pub fn query_contract<T: CLTyped + FromBytes>(
        &self,
        contract_hash_key: String,
        name: &str,
    ) -> Option<T> {
        match self.builder.query(
            None,
            Key::Account(self.accounts[0]),
            &[contract_hash_key, name.to_string()],
        ) {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .as_cl_value()
                    .expect("should be cl value.")
                    .clone()
                    .into_t()
                    .expect("should have the correct type.");
                Some(value)
            }
        }
    }

    pub fn query_dictionary_value<T: CLTyped + FromBytes>(
        &self,
        hash: Hash,
        dict_name: &str,
        key: String,
    ) -> Option<T> {
        // prepare the dictionary seed uref.
        let stored_value = self
            .builder
            .query(None, Key::Hash(hash), &[])
            .map_err(|_| "error")
            .unwrap();

        // get the named keys of the given Key.
        let named_keys = match &stored_value {
            StoredValue::Account(account) => account.named_keys(),
            StoredValue::Contract(contract) => contract.named_keys(),
            _ => return None,
        };

        // get the dictionary uref.
        let dictionary_uref = named_keys.get(dict_name).and_then(Key::as_uref).unwrap();

        let dictionary_key_bytes = key.as_bytes();

        let _address = Key::dictionary(*dictionary_uref, dictionary_key_bytes);

        // query the dictionary.
        match self
            .builder
            .query_dictionary_item(None, *dictionary_uref, &key)
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .as_cl_value()
                    .expect("should be cl value.")
                    .clone()
                    .into_t()
                    .expect("should have the correct type.");
                Some(value)
            }
        }
    }

    /// call a contract's specific entry point.
    pub fn call(&mut self, hash: Hash, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;

        // prepare the deploy item.
        let deploy_item = DeployItemBuilder::new()
            // .with_payment_bytes(module_bytes, args)
            .with_empty_payment_bytes(runtime_args! {
                ARG_AMOUNT => *DEFAULT_PAYMENT
            })
            .with_stored_session_hash(hash.into(), method, args)
            .with_authorization_keys(&[address])
            .with_address(address)
            .build();

        // prepare the execute request.
        // we can use .with_block_time() when setting the execute request.
        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();

        // executes the execute_request.
        self.builder.exec(execute_request).commit().expect_success();
    }
}
