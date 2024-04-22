use core_mx_life_bonding_sc::{admin::ProxyTrait as _, config::ProxyTrait as _, ProxyTrait as _};
use datanftmint::{collection_management::ProxyTrait as _, ProxyTrait as _};
use multiversx_sc::{
    codec::multi_types::MultiValue2,
    types::{Address, MultiValueEncoded},
};
use multiversx_sc_scenario::{
    api::StaticApi,
    managed_address, managed_biguint, managed_token_id, managed_token_id_wrapped,
    scenario_model::{Account, AddressValue, ScCallStep, ScDeployStep, SetStateStep, TxExpect},
    ContractInfo, ScenarioWorld,
};

pub const BONDING_CONTRACT_PATH: &str = "mxsc:output/core_mx_life_bonding_sc.mxsc.json";

pub const MINTER_CONTRACT_PATH: &str = "mxsc:output/datanftmint.mxsc.json";
pub const MINTER_CONTRACT_ADDRESS_EXPR: &str = "sc:datanftmint";

pub const MINTER_OWNER_ADDRESS_EXPR: &str = "address:minter-owner";
pub const MINTER_ADMIN_ADDRESS_EXPR: &str = "address:minter-admin";

pub const BONDING_OWNER_ADDRESS_EXPR: &str = "address:bonding-owner";
pub const BONDING_ADMIN_ADDRESS_EXPR: &str = "address:bonding-admin";

pub const BONDING_CONTRACT_ADDRESS_EXPR: &str = "sc:bond_contract";

pub const ITHEUM_TOKEN_IDENTIFIER_EXPR: &str = "str:ITHEUM-fce905";
pub const ITHEUM_TOKEN_IDENTIFIER: &[u8] = b"ITHEUM-fce905";

pub const ANOTHER_TOKEN_IDENTIFIER_EXPR: &str = "str:ANOTHER-fce905";
pub const ANOTHER_TOKEN_IDENTIFIER: &[u8] = b"ANOTHER-fce905";

pub const DATA_NFT_IDENTIFIER_EXPR: &str = "str:DATANFT-12345";
pub const DATA_NFT_IDENTIFIER: &[u8] = b"DATANFT-12345";

pub const COLLECTION_NAME: &str = "DATANFT-FT";

pub const FIRST_USER_ADDRESS_EXPR: &str = "address:first_user";
pub const SECOND_USER_ADDRESS_EXPR: &str = "address:second_user";
pub const THIRD_USER_ADDRESS_EXPR: &str = "address:third_user";

pub const TREAASURY_ADDRESS_EXPR: &str = "address:treasury";

pub const WITHDRAWAL_ADDRESS_EXPR: &str = "address:withdrawal";

type MinterContract = ContractInfo<datanftmint::Proxy<StaticApi>>;
type BondContract = ContractInfo<core_mx_life_bonding_sc::Proxy<StaticApi>>;

pub fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("");

    blockchain.register_contract(MINTER_CONTRACT_PATH, datanftmint::ContractBuilder);

    blockchain.register_contract(
        BONDING_CONTRACT_PATH,
        core_mx_life_bonding_sc::ContractBuilder,
    );

    blockchain
}

pub struct ContractsState {
    pub world: ScenarioWorld,
    pub minter_contract: MinterContract,
    pub bond_contract: BondContract,
    pub admin: Address,
    pub first_user: Address,
    pub second_user: Address,
    pub third_user: Address,
    pub treasury: Address,
}

impl ContractsState {
    pub fn new() -> Self {
        let mut world = world();

        world.set_state_step(
            SetStateStep::new()
                .put_account(
                    MINTER_OWNER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("20000000000000000000")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "1_000"),
                )
                .new_address(MINTER_OWNER_ADDRESS_EXPR, 1, MINTER_CONTRACT_ADDRESS_EXPR)
                .put_account(
                    MINTER_ADMIN_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("1_000")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "1_000"),
                )
                .put_account(
                    BONDING_OWNER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("1_000")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "1_000"),
                )
                .new_address(BONDING_OWNER_ADDRESS_EXPR, 1, BONDING_CONTRACT_ADDRESS_EXPR)
                .put_account(
                    BONDING_ADMIN_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("1_000")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "1_000"),
                )
                .put_account(
                    FIRST_USER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("100")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "200")
                        .esdt_balance(ANOTHER_TOKEN_IDENTIFIER_EXPR, "5"),
                )
                .put_account(
                    SECOND_USER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("100")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "200"),
                )
                .put_account(
                    THIRD_USER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("100")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "200"),
                )
                .put_account(WITHDRAWAL_ADDRESS_EXPR, Account::new().nonce(1))
                .put_account(TREAASURY_ADDRESS_EXPR, Account::new().nonce(1)),
        );

        let minter_contract = MinterContract::new(MINTER_CONTRACT_ADDRESS_EXPR);
        let bond_contract = BondContract::new(BONDING_CONTRACT_ADDRESS_EXPR);

        let admin = AddressValue::from(MINTER_ADMIN_ADDRESS_EXPR).to_address();
        let first_user = AddressValue::from(FIRST_USER_ADDRESS_EXPR).to_address();
        let second_user = AddressValue::from(SECOND_USER_ADDRESS_EXPR).to_address();
        let third_user = AddressValue::from(THIRD_USER_ADDRESS_EXPR).to_address();
        let treasury = AddressValue::from(TREAASURY_ADDRESS_EXPR).to_address();

        Self {
            world,
            minter_contract,
            bond_contract,
            admin,
            first_user,
            second_user,
            third_user,
            treasury,
        }
    }

    //minter setup
    pub fn deploy_minter(&mut self) -> &mut Self {
        let minter_code = self.world.code_expression(MINTER_CONTRACT_PATH);

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(MINTER_OWNER_ADDRESS_EXPR)
                .code(minter_code)
                .call(self.minter_contract.init()),
        );
        self
    }

    pub fn minter_freeze_single_nft(
        &mut self,
        caller: &str,
        nonce: u64,
        address: &Address,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.minter_contract
                        .freeze_single_token_for_address(nonce, managed_address!(address)),
                )
                .expect(TxExpect::ok()),
        );
        self
    }

    pub fn minter_unfreeze_single_nft(
        &mut self,
        caller: &str,
        nonce: u64,
        address: &Address,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.minter_contract
                        .unfreeze_single_token_for_address(nonce, managed_address!(address)),
                )
                .expect(TxExpect::ok()),
        );
        self
    }

    pub fn minter_wipe_single_nft(
        &mut self,
        caller: &str,
        nonce: u64,
        address: &Address,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.minter_contract
                        .wipe_single_token_for_address(nonce, managed_address!(address)),
                )
                .expect(TxExpect::ok()),
        );
        self
    }

    pub fn minter_freeze_collection_for_address(
        &mut self,
        caller: &str,
        address: &Address,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.minter_contract
                        .freeze_collection_for_address(managed_address!(address)),
                )
                .expect(TxExpect::ok()),
        );
        self
    }

    pub fn minter_unfreeze_collection_for_address(
        &mut self,
        caller: &str,
        address: &Address,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.minter_contract
                        .unfreeze_collection_for_address(managed_address!(address)),
                )
                .expect(TxExpect::ok()),
        );
        self
    }

    pub fn minter_set_administarator(
        &mut self,
        caller: &str,
        address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.set_administrator(address))
                .expect(tx_expect),
        );
        self
    }

    pub fn minter_set_treasury_address(
        &mut self,
        caller: &str,
        address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.set_treasury_address(address))
                .expect(tx_expect),
        );
        self
    }

    pub fn minter_set_donation_treasury_address(
        &mut self,
        caller: &str,
        address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.set_donation_treasury_address(address))
                .expect(tx_expect),
        );
        self
    }

    pub fn minter_set_donation_max_percentage(
        &mut self,
        caller: &str,
        max_donation_percentage: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.minter_contract
                        .set_max_donation_percentage(max_donation_percentage),
                )
                .expect(tx_expect),
        );
        self
    }

    pub fn pause_minter_contract(&mut self, caller: &str, expect: Option<TxExpect>) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.set_is_paused(true))
                .expect(tx_expect),
        );
        self
    }

    pub fn unpause_minter_contract(&mut self, caller: &str, expect: Option<TxExpect>) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.set_is_paused(false))
                .expect(tx_expect),
        );
        self
    }

    pub fn minter_enable_whitelist(&mut self, caller: &str, expect: Option<TxExpect>) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.set_whitelist_enabled(true))
                .expect(tx_expect),
        );
        self
    }

    pub fn minter_disable_whitelist(
        &mut self,
        caller: &str,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.set_whitelist_enabled(false))
                .expect(tx_expect),
        );
        self
    }

    pub fn minter_set_royalties_limits(
        &mut self,
        caller: &str,
        min_royalties: u64,
        max_royalties: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.set_royalties_limits(
                    managed_biguint!(min_royalties),
                    managed_biguint!(max_royalties),
                ))
                .expect(tx_expect),
        );
        self
    }

    pub fn minter_set_max_supply(
        &mut self,
        caller: &str,
        max_supply: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.minter_contract
                        .set_max_supply(managed_biguint!(max_supply)),
                )
                .expect(tx_expect),
        );
        self
    }

    pub fn minter_set_anti_spam_tax_token_and_amount(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        amount: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.minter_contract
                        .set_anti_spam_tax(managed_token_id_wrapped!(token_identifier), amount),
                )
                .expect(tx_expect),
        );
        self
    }

    pub fn minter_add_to_whitelist(
        &mut self,
        caller: &str,
        address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        let mut multivalue = MultiValueEncoded::new();
        multivalue.push(managed_address!(&address));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.set_whitelist_spots(multivalue))
                .expect(tx_expect),
        );
        self
    }

    pub fn minter_set_bond_contract_address(
        &mut self,
        caller: &str,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        let bond_address = self.bond_contract.address.clone().into_option().unwrap();
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.set_bond_contract_address(bond_address))
                .expect(tx_expect),
        );
        self
    }

    pub fn minter_remove_from_whitelist(
        &mut self,
        caller: &str,
        address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        let mut multivalue = MultiValueEncoded::new();

        multivalue.push(managed_address!(&address));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.remove_whitelist_spots(multivalue))
                .expect(tx_expect),
        );
        self
    }

    pub fn minter_set_mint_time_limit(
        &mut self,
        caller: &str,
        mint_time_limit: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.set_mint_time_limit(mint_time_limit))
                .expect(tx_expect),
        );
        self
    }

    pub fn set_withdrawal_address(
        &mut self,
        caller: &str,
        address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.set_withdrawal_address(address))
                .expect(tx_expect),
        );
        self
    }

    pub fn minter_withdraw(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        nonce: u64,
        amount: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.withdraw(
                    managed_token_id_wrapped!(token_identifier),
                    nonce,
                    managed_biguint!(amount),
                ))
                .expect(expect.unwrap_or(TxExpect::ok())),
        );
        self
    }

    pub fn minter_burn(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        nonce: u64,
        amount: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .esdt_transfer(token_identifier, nonce, amount)
                .call(self.minter_contract.burn_token())
                .expect(expect.unwrap_or(TxExpect::ok())),
        );
        self
    }

    pub fn minter_mint(
        &mut self,
        caller: &str,
        name: &str,
        media: &str,
        medatada: &str,
        data_marshal: &str,
        data_stream: &str,
        data_preview: &str,
        royalties: u64,
        supply: u64,
        title: &str,
        description: &str,
        lock_period: u64,
        payment_token_identifier: &[u8],
        payment_token_nonce: u64,
        payment_amount: u64,
        donation_percentage: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .esdt_transfer(
                    payment_token_identifier,
                    payment_token_nonce,
                    payment_amount,
                )
                .call(self.minter_contract.mint_token(
                    name,
                    media,
                    medatada,
                    data_marshal,
                    data_stream,
                    data_preview,
                    managed_biguint!(royalties),
                    managed_biguint!(supply),
                    title,
                    description,
                    lock_period,
                    donation_percentage,
                    MultiValueEncoded::new(),
                ))
                .expect(expect.unwrap_or(TxExpect::ok())),
        );
        self
    }

    pub fn minter_set_local_roles(&mut self, caller: &str, expect: Option<TxExpect>) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.minter_contract.set_local_roles())
                .expect(tx_expect),
        );
        self
    }

    pub fn minter_initialize_contract(
        &mut self,
        caller: &str,
        collection_name: &str,
        token_ticker: &str,
        anti_spam_tax_token: &[u8],
        anti_spam_tax_value: u64,
        mint_time_limit: u64,
        treasury_address: Address,
        amount: Option<u64>,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .egld_value(amount.unwrap_or(50000000000000000))
                .call(self.minter_contract.initialize_contract(
                    collection_name,
                    token_ticker,
                    managed_token_id_wrapped!(anti_spam_tax_token),
                    managed_biguint!(anti_spam_tax_value),
                    mint_time_limit,
                    treasury_address,
                ))
                .expect(expect.unwrap_or(TxExpect::ok())),
        );

        self
    }

    pub fn mock_minter_initialized(
        &mut self,
        anti_spam_tax_token: &[u8],
        anti_spam_tax_value: u64,
        mint_time_limit: u64,
    ) -> &mut Self {
        let admin = self.admin.clone();
        let treasury_address = self.treasury.clone();
        let minter_code = self.world.code_expression(MINTER_CONTRACT_PATH);

        let mut acc = Account::new()
            .esdt_roles(
                DATA_NFT_IDENTIFIER_EXPR,
                vec![
                    "ESDTRoleNFTCreate".to_string(),
                    "ESDTRoleNFTBurn".to_string(),
                ],
            )
            .code(minter_code);

        acc.storage.insert(
            b"sft_token_id".to_vec().into(),
            DATA_NFT_IDENTIFIER.to_vec().into(),
        );

        acc.storage
            .insert(b"roles_are_set".to_vec().into(), b"1".to_vec().into());

        acc.owner = Option::Some(AddressValue::from(MINTER_OWNER_ADDRESS_EXPR));
        self.world.set_state_step(
            SetStateStep::new()
                .new_token_identifier(DATA_NFT_IDENTIFIER_EXPR)
                .put_account(MINTER_CONTRACT_ADDRESS_EXPR, acc),
        );

        self.minter_enable_whitelist(MINTER_OWNER_ADDRESS_EXPR, None);
        self.minter_set_max_supply(MINTER_OWNER_ADDRESS_EXPR, 20u64, None);
        self.minter_set_royalties_limits(MINTER_OWNER_ADDRESS_EXPR, 0u64, 8000u64, None);
        self.minter_set_administarator(MINTER_OWNER_ADDRESS_EXPR, admin, None);
        self.minter_set_bond_contract_address(MINTER_OWNER_ADDRESS_EXPR, None);
        self.minter_set_treasury_address(MINTER_OWNER_ADDRESS_EXPR, treasury_address.clone(), None);
        self.minter_set_donation_treasury_address(
            MINTER_ADMIN_ADDRESS_EXPR,
            treasury_address,
            None,
        );
        self.minter_set_anti_spam_tax_token_and_amount(
            MINTER_OWNER_ADDRESS_EXPR,
            anti_spam_tax_token,
            anti_spam_tax_value,
            None,
        );
        self.minter_set_mint_time_limit(MINTER_OWNER_ADDRESS_EXPR, mint_time_limit, None);
        self.minter_set_local_roles(MINTER_OWNER_ADDRESS_EXPR, None);
        self
    }

    pub fn minter_upgrade(&mut self) -> &mut Self {
        let minter_code = self.world.code_expression(MINTER_CONTRACT_PATH);
        self.world.sc_call(
            ScCallStep::new()
                .from(MINTER_OWNER_ADDRESS_EXPR)
                .to(MINTER_CONTRACT_ADDRESS_EXPR)
                .function("upgradeContract")
                .argument(&minter_code)
                .argument("0x0502") // codeMetadata
                .expect(TxExpect::ok()),
        );
        self
    }

    // bonding setup
    pub fn deploy_bonding(&mut self) -> &mut Self {
        let bonding_code = self.world.code_expression(BONDING_CONTRACT_PATH);

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(BONDING_OWNER_ADDRESS_EXPR)
                .code(bonding_code)
                .call(self.bond_contract.init()),
        );
        self
    }

    pub fn bond_set_administrator(
        &mut self,
        caller: &str,
        address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.bond_contract
                        .set_administrator(managed_address!(&address)),
                )
                .expect(tx_expect),
        );
        self
    }

    pub fn bond_set_accepted_caller(
        &mut self,
        caller: &str,
        address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        let mut arg = MultiValueEncoded::new();

        arg.push(managed_address!(&address));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.bond_contract.set_accepted_callers(arg))
                .expect(tx_expect),
        );
        self
    }

    pub fn bond_set_bond_token(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.bond_contract
                        .set_bond_token(managed_token_id!(token_identifier)),
                )
                .expect(tx_expect),
        );
        self
    }

    pub fn bond_set_lock_period_and_bond(
        &mut self,
        caller: &str,
        lock_period: u64,
        bond: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        let mut arg = MultiValueEncoded::new();
        arg.push(MultiValue2((lock_period, managed_biguint!(bond))));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.bond_contract.set_lock_periods_with_bonds(arg))
                .expect(tx_expect),
        );
        self
    }

    pub fn bond_pause_contract(&mut self, caller: &str, expect: Option<TxExpect>) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.bond_contract.set_contract_state_inactive())
                .expect(tx_expect),
        );
        self
    }

    pub fn bond_unpause_contract(&mut self, caller: &str, expect: Option<TxExpect>) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.bond_contract.set_contract_state_active())
                .expect(tx_expect),
        );
        self
    }

    pub fn bond_contract_default_deploy_and_set(
        &mut self,
        lock_period: u64,
        bond_amount: u64,
    ) -> &mut Self {
        let admin = self.admin.clone();
        self.deploy_bonding()
            .bond_set_administrator(BONDING_OWNER_ADDRESS_EXPR, admin.clone(), None)
            .bond_set_accepted_caller(
                BONDING_OWNER_ADDRESS_EXPR,
                AddressValue::from(MINTER_CONTRACT_ADDRESS_EXPR).to_address(),
                None,
            )
            .bond_set_bond_token(BONDING_OWNER_ADDRESS_EXPR, ITHEUM_TOKEN_IDENTIFIER, None)
            .bond_set_lock_period_and_bond(
                BONDING_OWNER_ADDRESS_EXPR,
                lock_period,
                bond_amount,
                None,
            )
            .bond_unpause_contract(BONDING_OWNER_ADDRESS_EXPR, None);

        self
    }
}
