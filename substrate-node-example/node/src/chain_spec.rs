use node_template_runtime::{
	AccountId, AuraConfig, Balance, BalancesConfig, BlockNumber, ChainlinkFeedConfig,
	GenesisConfig, GrandpaConfig, Signature, SudoConfig, SystemConfig, Value, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::fs::File;
use std::path::PathBuf;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// trys to parse the json content into a `ChainSpec`,
/// if the given file was `ChainlinkFeedConfig` then the `local_testnet_config` is used with
/// the `ChainlinkFeedConfig` of the given file
pub fn from_json_file(path: &str) -> Result<ChainSpec, String> {
	if let spec @ Ok(_) = ChainSpec::from_json_file(PathBuf::from(path)) {
		spec
	} else {
		let path = path.to_string();
		let wasm_binary =
			WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
		Ok(ChainSpec::from_genesis(
			// Name
			"Local Testnet",
			// ID
			"local_testnet",
			ChainType::Local,
			move || {
				let mut config = testnet_genesis(
					wasm_binary,
					// Initial PoA authorities
					vec![authority_keys_from_seed("Alice")],
					// Sudo account
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					// Pre-funded accounts
					vec![
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
					],
					true,
				);
				let feed_config = serde_json::from_str(&path).unwrap_or_else(|_| {
					let file = File::open(&path)
						.map_err(|e| format!("Error opening spec file: {}", e))
						.unwrap();
					serde_json::from_reader::<_, ChainlinkFeedConfig>(file)
						.map_err(|e| format!("Error parsing spec file: {}", e))
						.unwrap()
				});
				config.chainlink_feed = feed_config;
				config
			},
			// Bootnodes
			vec![],
			// Telemetry
			None,
			// Protocol ID
			None,
			// Properties
			None,
			// Extensions
			None,
		))
	}
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					authority_keys_from_seed("Alice"),
					authority_keys_from_seed("Bob"),
				],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities
				.iter()
				.map(|x| (x.1.clone(), 1))
				.collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key.clone(),
		},
		chainlink_feed: ChainlinkFeedConfig {
			pallet_admin: Some(root_key),
			feed_creators: endowed_accounts,
			feeds: vec![],
		},
	}
}

/// Create a feed for genesis
#[allow(unused)]
pub fn feed_builder() -> node_template_runtime::FeedBuilder<AccountId, Balance, BlockNumber, Value>
{
	node_template_runtime::FeedBuilder::new()
		.owner(get_account_id_from_seed::<sr25519::Public>("Alice"))
		.decimals(8)
		.payment(1_000)
		.restart_delay(0)
		.timeout(10)
		.description(b"LINK".to_vec())
		.value_bounds(1, 1_000)
		.min_submissions(2)
		.oracles(vec![
			(
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			),
		])
}
