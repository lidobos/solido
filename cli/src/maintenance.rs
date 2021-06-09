//! Entry point for maintenance operations, such as updating the pool balance.

use crate::helpers::{get_solido, get_stake_pool};
use crate::{Config, Error};
use clap::Clap;
use lido::{state::{Lido, Validator}, token::Lamports};
use solana_program::{pubkey::Pubkey, rent::Rent, clock::Clock, stake_history::StakeHistory, sysvar};
use solana_sdk::{account::Account, borsh::try_from_slice_unchecked, instruction::Instruction};
use solana_client::rpc_client::RpcClient;
use spl_stake_pool::state::{StakePool, ValidatorList};
use spl_stake_pool::stake_program::StakeState;
use lido::state::PubkeyAndEntry;
use borsh::BorshDeserialize;

type Result<T> = std::result::Result<T, Error>;

pub enum MaintenanceResult {
    DidExecute,
    NothingToDo,
}

#[derive(Clap, Debug)]
pub struct PerformMaintenanceOpts {
    /// Address of the Solido program.
    #[clap(long)]
    pub solido_program_id: Pubkey,

    /// Account that stores the data for this Solido instance.
    #[clap(long)]
    pub solido_address: Pubkey,

    /// Stake pool program id
    #[clap(long)]
    stake_pool_program_id: Pubkey,
}

/// A snapshot of on-chain accounts relevant to Solido.
struct SolidoState {
    // TODO: The dead_code below will no longer be dead once we implement the
    // maintenance tasks.
    #[allow(dead_code)]
    solido: Lido,

    #[allow(dead_code)]
    stake_pool: StakePool,

    #[allow(dead_code)]
    validator_list_account: Account,
    #[allow(dead_code)]
    validator_list: ValidatorList,

    /// For each validator, in the same order as in `solido.validators`, holds
    /// the stake balance of the derived stake accounts from the begin seed until
    /// end seed.
    validator_stake_accounts: Vec<Vec<(Pubkey, StakeBalance)>>,

    reserve_account: Account,
    rent: Rent,
}

/// The balance of a stake account, split into the four states that stake can be in.
///
/// The sum of the four fields is equal to the SOL balance of the stake account.
struct StakeBalance {
    inactive: Lamports,
    activating: Lamports,
    active: Lamports,
    deactivating: Lamports,
}

impl StakeBalance {
    pub fn is_fully_active(&self) -> bool {
        true
        && self.inactive == Lamports(0)
        && self.activating == Lamports(0)
        && self.deactivating == Lamports(0)
        // We define an empty stake account to not be fully active,
        // because the SPL stake pool program considers such accounts
        // to not be fully active. The case is not relevant anyway,
        // because an empty stake account would not be rent-exempt,
        // so it would disappear.
        && self.active > Lamports(0)
    }
}

fn get_validator_stake_accounts(
    rpc_client: &RpcClient,
    solido_program_id: &Pubkey,
    solido_address: &Pubkey,
    clock: &Clock,
    stake_history: &StakeHistory,
    validator: &PubkeyAndEntry<Validator>,
) -> Result<Vec<(Pubkey, StakeBalance)>> {
    let mut result = Vec::new();
    for seed in validator.entry.stake_accounts_seed_begin..validator.entry.stake_accounts_seed_end {
        let (addr, _bump_seed) = Validator::find_stake_account_address(solido_program_id, solido_address, &validator.pubkey, seed);
        let account = rpc_client.get_account(&addr)?;
        let stake_state = StakeState::try_from_slice(&account.data)
            .expect("Derived stake account contains invalid data.");
        let delegation = stake_state.delegation()
            .expect("Encountered undelegated stake account, this should not happen.");

        // TODO: Confirm that `delegation.voter_pubkey == validator.vote_account`. Unfortunately we
        // do not store the vote account in there at the moment. Once we switch to using the vote
        // account as key for the dictionary, we could check that here.

        let target_epoch = clock.epoch;
        let history = Some(stake_history);
        // TODO(ruuda): Confirm the meaning of this.
        let fix_stake_deactivate = true;

        let (active_lamports, activating_lamports, deactivating_lamports) = delegation
            .stake_activating_and_deactivating(target_epoch, history, fix_stake_deactivate);

        let inactive_lamports = account.lamports
            .checked_sub(active_lamports)
            .expect("Active stake cannot be larger than stake account balance.")
            .checked_sub(activating_lamports)
            .expect("Activating stake cannot be larger than stake account balance - active.")
            .checked_sub(deactivating_lamports)
            .expect("Deactivating stake cannot be larger than stake account balance - active - activating.");

        let balance = StakeBalance {
            inactive: Lamports(inactive_lamports),
            activating: Lamports(activating_lamports),
            active: Lamports(active_lamports),
            deactivating: Lamports(deactivating_lamports),
        };

        result.push((addr, balance));
    }
    Ok(result)
}

impl SolidoState {
    /// Read the state from the on-chain data.
    pub fn new(
        config: &Config,
        solido_program_id: &Pubkey,
        solido_address: &Pubkey,
    ) -> Result<SolidoState> {
        let rpc = &config.rpc;

        // TODO(ruuda): Transactions can execute in between those reads, leading to
        // a torn state. Make a function that re-reads everything with get_multiple_accounts.
        let solido = get_solido(rpc, solido_address)?;
        let stake_pool = get_stake_pool(rpc, &solido.stake_pool_account)?;

        let validator_list_account = rpc.get_account(&stake_pool.validator_list)?;
        let validator_list =
            try_from_slice_unchecked::<ValidatorList>(&validator_list_account.data)?;

        let reserve_account =
            rpc.get_account(&solido.get_reserve_account(solido_program_id, solido_address)?)?;

        let rent_account = rpc.get_account(&sysvar::rent::ID)?;
        let rent: Rent = bincode::deserialize(&rent_account.data)?;

        let clock_account = config.rpc().get_account(&sysvar::clock::ID)?;
        let clock: Clock = bincode::deserialize(&clock_account.data)?;

        let stake_history_account = config.rpc().get_account(&sysvar::stake_history::ID)?;
        let stake_history: StakeHistory = bincode::deserialize(&stake_history_account.data)?;

        let mut validator_stake_accounts = Vec::new();
        for validator in solido.validators.entries.iter() {
            validator_stake_accounts.push(get_validator_stake_accounts(rpc, solido_program_id, solido_address, &clock, &stake_history, validator)?);
        }

        Ok(SolidoState {
            solido,
            stake_pool,
            validator_list_account,
            validator_list,
            validator_stake_accounts,
            reserve_account,
            rent,
        })
    }

    /// Return the amount of SOL in the reserve account that could be spent
    /// while still keeping the reserve account rent-exempt.
    pub fn get_effective_reserve(&self) -> Lamports {
        Lamports(
            self.reserve_account
                .lamports
                .saturating_sub(self.rent.minimum_balance(0)),
        )
    }

    /// If there is a deposit that can be staked, return the instruction to do so.
    pub fn try_stake_deposit(&self) -> Result<Option<Instruction>> {
        let reserve_balance = self.get_effective_reserve();
        let minimum_stake_account_balance =
            Lamports(
                self.rent.minimum_balance(std::mem::size_of::<StakeState>())
            );

        // If there is not enough reserve to create a new stake account, we
        // can't stake the deposit, even if there is some balance.
        if reserve_balance < minimum_stake_account_balance {
            return Ok(None);
        }

        // We can make a deposit.
        // TODO: Implement
        Ok(None)
    }

    /// If there is active stake that can be deposited to the stake pool,
    /// return the instruction to do so.
    pub fn try_deposit_active_stake_to_pool(&self) -> Result<Option<Instruction>> {
        for (_validator, stake_accounts) in
            self.solido.validators.entries.iter().zip(self.validator_stake_accounts.iter()) {
            for (_stake_account_addr, stake_balance) in stake_accounts.iter() {
                if stake_balance.is_fully_active() {
                    // TODO: Generate DepositActiveStake instruction.
                }
            }
        }
        Ok(None)
    }
}

/// Inspect the on-chain Solido state, and if there is maintenance that can be
/// performed, do so.
///
/// This takes only one step, there might be more work left to do after this
/// function returns. Call it in a loop until it returns
/// [`MaintenanceResult::NothingToDo`]. (And then still call it in a loop,
/// because the on-chain state might change.)
pub fn perform_maintenance(
    config: &Config,
    opts: PerformMaintenanceOpts,
) -> Result<MaintenanceResult> {
    let state = SolidoState::new(config, &opts.solido_program_id, &opts.solido_address)?;

    // Try all of these operations one by one, and select the first one that
    // produces an instruction.
    let instruction: Option<Result<Instruction>> = None
        .or_else(|| state.try_stake_deposit().transpose())
        .or_else(|| state.try_deposit_active_stake_to_pool().transpose());

    match instruction {
        Some(Ok(_instr)) => {
            // TODO: Execute.
            Ok(MaintenanceResult::DidExecute)
        }
        Some(Err(err)) => Err(err),
        None => Ok(MaintenanceResult::NothingToDo),
    }
}
