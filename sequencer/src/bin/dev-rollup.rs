use std::str::FromStr;

use anyhow::Result;
use clap::{Parser, Subcommand};
use committable::Committable;
use espresso_types::{
    v0_3::{RollupRegistration, RollupRegistrationBody, RollupUpdate, RollupUpdatebody},
    SeqTypes,
};
use hotshot::types::BLSPubKey;
use hotshot_types::traits::{node_implementation::NodeType, signature_key::SignatureKey};
use marketplace_solver::SolverError;

use sequencer_utils::logging;
use url::Url;
use vbs::version::StaticVersion;

#[derive(Debug, Parser)]
struct Options {
    #[clap(flatten)]
    logging: logging::Config,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Register(Register),
    Update(Update),
}

// Options for registering a rollup
#[derive(Parser, Debug)]
struct Register {
    #[clap(short, long, env = "ESPRESSO_MARKETPLACE_SOLVER_API_URL")]
    pub solver_url: Url,

    #[clap(short, long = "ns")]
    pub namespace_id: u64,

    #[clap(
        long,
        env = "ESPRESSO_MARKETPLACE_RESERVE_BUILDER_URL",
        default_value_t = Url::from_str("http://localhost").unwrap()
    )]
    pub reserve_url: Url,

    #[clap(long, default_value_t = 200)]
    pub reserve_price: u64,

    #[clap(long, default_value_t = false)]
    pub active: bool,

    #[clap(long, default_value = "test")]
    pub text: String,

    /// The private key is provided in tagged-base64 format.
    /// If not provided, a default private key with a seed of `[0; 32]` and an index of `9876` will be used.
    #[clap(long = "privkey")]
    pub private_key: Option<String>,
}

// Options for updating an already registered rollup
#[derive(Parser, Debug)]
struct Update {
    #[clap(short, long, env = "ESPRESSO_MARKETPLACE_SOLVER_API_URL")]
    pub solver_url: Url,

    #[clap(short, long = "ns")]
    pub namespace_id: u64,

    #[clap(long, env = "ESPRESSO_MARKETPLACE_RESERVE_BUILDER_URL")]
    pub reserve_url: Option<Url>,

    #[clap(long)]
    pub reserve_price: Option<u64>,

    #[clap(long)]
    pub active: Option<bool>,

    #[clap(long, default_value = "test")]
    pub text: Option<String>,

    /// The private key is provided in tagged-base64 format.
    /// If not provided, a default private key with a seed of `[0; 32]` and an index of `9876` will be used.
    #[clap(long = "privkey")]
    pub private_key: Option<String>,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let opt = Options::parse();
    opt.logging.init();

    match opt.command {
        Command::Register(opt) => register(opt).await,
        Command::Update(opt) => update(opt).await,
    }
}

async fn register(opt: Register) -> Result<()> {
    let Register {
        solver_url,
        namespace_id,
        reserve_url,
        reserve_price,
        active,
        text,
        private_key,
    } = opt;

    let client = surf_disco::Client::<SolverError, StaticVersion<0, 1>>::new(
        solver_url.join("marketplace-solver").unwrap(),
    );

    let (pubkey, privkey) = if let Some(privkey) = private_key {
        let privkey = <BLSPubKey as SignatureKey>::PrivateKey::from_str(&privkey)
            .expect("invalid private key provided");
        let pubkey = BLSPubKey::from_private(&privkey);

        (pubkey, privkey)
    } else {
        BLSPubKey::generated_from_seed_indexed([0; 32], 9876)
    };

    let reg_body = RollupRegistrationBody {
        namespace_id: namespace_id.into(),
        reserve_url: reserve_url.clone(),
        reserve_price: reserve_price.into(),
        active,
        signature_keys: vec![pubkey],
        text,
        signature_key: pubkey,
    };

    // Sign the registration body
    let signature =
        <SeqTypes as NodeType>::SignatureKey::sign(&privkey, reg_body.commit().as_ref())
            .expect("failed to sign");

    let reg = RollupRegistration {
        body: reg_body.clone(),
        signature,
    };

    // registering a rollup
    client
        .post::<RollupRegistration>("register_rollup")
        .body_json(&reg)
        .unwrap()
        .send()
        .await
        .unwrap();

    tracing::info!("rollup with namespace {namespace_id} registered");

    Ok(())
}

async fn update(opt: Update) -> Result<()> {
    let Update {
        solver_url,
        namespace_id,
        reserve_url,
        reserve_price,
        active,
        text,
        private_key,
    } = opt;

    let client = surf_disco::Client::<SolverError, StaticVersion<0, 1>>::new(
        solver_url.join("marketplace-solver").unwrap(),
    );
    let (pubkey, privkey) = if let Some(privkey) = private_key {
        let privkey = <BLSPubKey as SignatureKey>::PrivateKey::from_str(&privkey)
            .expect("invalid private key provided");
        let pubkey = BLSPubKey::from_private(&privkey);

        (pubkey, privkey)
    } else {
        BLSPubKey::generated_from_seed_indexed([0; 32], 9876)
    };

    let body = RollupUpdatebody {
        namespace_id: namespace_id.into(),
        reserve_url,
        reserve_price: reserve_price.map(Into::into),
        active,
        signature_keys: None,
        signature_key: pubkey,
        text,
    };

    // Sign the rollup update body
    let signature = <SeqTypes as NodeType>::SignatureKey::sign(&privkey, body.commit().as_ref())
        .expect("failed to sign");

    let update = RollupUpdate {
        body: body.clone(),
        signature,
    };

    // update a rollup
    client
        .post::<RollupRegistration>("register_rollup")
        .body_json(&update)
        .unwrap()
        .send()
        .await
        .unwrap();

    tracing::info!("rollup with namespace {namespace_id} updated");

    Ok(())
}
