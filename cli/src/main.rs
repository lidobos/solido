extern crate spl_stake_pool;
use std::fmt;
use std::path::PathBuf;

use clap::Clap;
use serde::Serialize;
use solana_client::rpc_client::RpcClient;
use solana_remote_wallet::locator::Locator;
use solana_remote_wallet::remote_keypair::generate_remote_keypair;
use solana_remote_wallet::remote_wallet::maybe_wallet_manager;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::derivation_path::DerivationPath;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signer::Signer;

use crate::config::*;
use crate::error::Abort;
use crate::helpers::command_add_maintainer;
use crate::helpers::command_remove_maintainer;
use crate::helpers::command_show_solido;
use crate::helpers::{command_add_validator, command_create_solido};
use crate::multisig::MultisigOpts;

mod config;
mod daemon;
mod error;
mod helpers;
mod maintenance;
mod multisig;
mod prometheus;
mod spl_token_utils;
mod stake_account;
mod util;

/// Solido -- Interact with Lido for Solana.
#[derive(Clap, Debug)]
struct Opts {
    /// The keypair to sign and pay with. [default: ~/.config/solana/id.json]
    #[clap(long)]
    keypair_path: Option<PathBuf>,

    /// URL of cluster to connect to (e.g., https://api.devnet.solana.com for solana devnet)
    #[clap(long, default_value = "http://127.0.0.1:8899")]
    cluster: String,

    /// Whether to output text or json.
    #[clap(long = "output", default_value = "text", possible_values = &["text", "json"])]
    output_mode: OutputMode,

    #[clap(subcommand)]
    subcommand: SubCommand,

    /// Optional config path
    #[clap(long)]
    config: Option<PathBuf>,
}

#[derive(Clap, Debug)]
enum SubCommand {
    /// Create a new Lido for Solana instance.
    #[clap(after_help = r"ACCOUNTS:

    This sets up a few things:

    * An SPL token mint for stake pool tokens.
    * An SPL token mint for stSOL.
    * stSOL-denominated SPL token accounts for fee receivers.
    * The stake pool managed by this Solido instance.
    * The Solido instance itself.

FEES:

    Of the validation rewards that the stake pool receives, a fraction
    «fee-numerator» / «fee-denominator» gets paid out as fees. The remaining
    rewards get distributed implicitly to stSOL holders because they now own
    a share of a larger pool of SOL.

    The fees are distributed among the treasury, validators, and the
    developer, according to the ratio

    «treasury-fee» : «validation-fee» : «developer-fee»

    For example, if the fees are set to a 1 : 2 : 1 proportion, then the
    treasury and developers would receive 50% of the fees, and the validation
    would receive the remaining 50%. Subsequently, the validation fee is divided
    equally among all validators.
    ")]
    CreateSolido(CreateSolidoOpts),

    /// Adds a new validator
    AddValidator(AddValidatorOpts),
    /// Adds a maintainer to the Solido instance
    AddMaintainer(AddRemoveMaintainerOpts),
    /// Adds a maintainer to the Solido instance
    RemoveMaintainer(AddRemoveMaintainerOpts),

    /// Show an instance of solido in detail
    ShowSolido(ShowSolidoOpts),

    /// Execute one iteration of periodic maintenance logic.
    ///
    /// This is mainly useful for testing. To perform maintenance continuously,
    /// use 'run-maintainer' instead.
    PerformMaintenance(PerformMaintenanceOpts),

    /// Start the maintainer daemon.
    RunMaintainer(RunMaintainerOpts),

    /// Interact with a deployed Multisig program for governance tasks.
    Multisig(MultisigOpts),
}

/// Determines which network to connect to, and who pays the fees.
pub struct Config<'a> {
    /// Program instance, so we can call RPC methods.
    rpc: RpcClient,
    /// Reference to a signer, can be a keypair or ledger device.
    signer: &'a dyn Signer,
    /// output mode, can be json or text.
    output_mode: OutputMode,
}

/// Resolve ~/.config/solana/id.json.
fn get_default_keypair_path() -> PathBuf {
    let home = std::env::var("HOME").expect("Expected $HOME to be set.");
    let mut path = PathBuf::from(home);
    path.push(".config/solana/id.json");
    path
}

fn print_output<Output: fmt::Display + Serialize>(mode: OutputMode, output: &Output) {
    match mode {
        OutputMode::Text => println!("{}", output),
        OutputMode::Json => {
            let json_string =
                serde_json::to_string_pretty(output).expect("Failed to serialize output as json.");
            println!("{}", json_string);
        }
    }
}

fn main() {
    let mut opts = Opts::parse();

    // Read from config file
    let config_file = opts.config.map(read_config);
    let config_file = config_file.as_ref();
    solana_logger::setup_with_default("solana=info");

    let payer_keypair_path = opts.keypair_path.unwrap_or_else(get_default_keypair_path);
    let signer = &*get_signer(payer_keypair_path);

    let config = Config {
        rpc: RpcClient::new_with_commitment(opts.cluster, CommitmentConfig::confirmed()),
        signer,
        output_mode: opts.output_mode,
    };
    let output_mode = opts.output_mode;

    merge_with_config(&mut opts.subcommand, config_file);
    match opts.subcommand {
        SubCommand::CreateSolido(cmd_opts) => {
            let output = command_create_solido(&config, &cmd_opts)
                .ok_or_abort_with("Failed to create Solido instance.");
            print_output(output_mode, &output);
        }
        SubCommand::Multisig(cmd_opts) => multisig::main(&config, output_mode, cmd_opts),
        SubCommand::PerformMaintenance(cmd_opts) => {
            // For now, this does one maintenance iteration. In the future we
            // might add a daemon mode that runs continuously, and which logs
            // to stdout and exposes Prometheus metrics (also to monitor Solido,
            // not just the maintenance itself).
            let output = maintenance::run_perform_maintenance(&config, &cmd_opts)
                .ok_or_abort_with("Failed to perform maintenance.");
            match (output_mode, output) {
                (OutputMode::Text, None) => {
                    println!("Nothing done, there was no maintenance to perform.")
                }
                (OutputMode::Json, None) => println!("null"),
                (mode, Some(output)) => print_output(mode, &output),
            }
        }
        SubCommand::RunMaintainer(cmd_opts) => {
            daemon::main(&config, &cmd_opts);
        }
        SubCommand::AddValidator(cmd_opts) => {
            let output = command_add_validator(&config, &cmd_opts)
                .ok_or_abort_with("Failed to add validator.");
            print_output(output_mode, &output);
        }
        SubCommand::AddMaintainer(cmd_opts) => {
            let output = command_add_maintainer(&config, &cmd_opts)
                .ok_or_abort_with("Failed to add maintainer.");
            print_output(output_mode, &output);
        }
        SubCommand::RemoveMaintainer(cmd_opts) => {
            let output = command_remove_maintainer(&config, &cmd_opts)
                .ok_or_abort_with("Failed to remove maintainer.");
            print_output(output_mode, &output);
        }
        SubCommand::ShowSolido(cmd_opts) => {
            let output = command_show_solido(&config, &cmd_opts)
                .ok_or_abort_with("Failed to show Solido data.");
            print_output(output_mode, &output);
        }
    }
}

fn merge_with_config(subcommand: &mut SubCommand, config_file: Option<&ConfigFile>) {
    match subcommand {
        SubCommand::CreateSolido(opts) => opts.merge_with_config(config_file),
        SubCommand::AddValidator(opts) => opts.merge_with_config(config_file),
        SubCommand::AddMaintainer(opts) | SubCommand::RemoveMaintainer(opts) => {
            opts.merge_with_config(config_file)
        }
        SubCommand::ShowSolido(opts) => opts.merge_with_config(config_file),
        SubCommand::PerformMaintenance(opts) => opts.merge_with_config(config_file),
        SubCommand::Multisig(opts) => opts.merge_with_config(config_file),
        SubCommand::RunMaintainer(opts) => opts.merge_with_config(config_file),
    }
}

// Get a boxed signer that lives long enough for us to use it in the Config.
fn get_signer(payer_keypair_path: PathBuf) -> Box<dyn Signer> {
    let boxed_signer: Box<dyn Signer> = if payer_keypair_path.starts_with("usb://") {
        let hw_wallet = maybe_wallet_manager()
            .expect("Remote wallet found, but failed to establish protocol. Maybe the Solana app is not open.")
            .expect("Failed to find a remote wallet, maybe Ledger is not connected or locked.");
        Box::new(
            generate_remote_keypair(
                Locator::new_from_path(
                    payer_keypair_path
                        .into_os_string()
                        .into_string()
                        .expect("Should have failed before"),
                )
                .expect("Failed reading URL."),
                DerivationPath::default(),
                &hw_wallet,
                false,    /* Confirm public key */
                "Solido", /* When multiple wallets are connected, used to display a hint */
            )
            .expect("Failed to contact remote wallet"),
        )
    } else {
        Box::new(
            read_keypair_file(&payer_keypair_path).expect("Failed to read key pair from file."),
        )
    };
    boxed_signer
}