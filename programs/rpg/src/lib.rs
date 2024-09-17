use anchor_lang::prelude::*;
use anchor_lang::solana_program::log::sol_log_compute_units;

declare_id!("5Sc3gJv4tvPiFzE75boYMJabbNRs44zRhtT23fLdKewz");

mod state;
mod instructions;
mod constants;
mod helpers;
mod error;

use state::*;
use instructions::*;
use constants::*;
use helpers::*;

#[program]
pub mod rpg {
    use super::*;

    // SOLUTION EDIT:
    pub fn create_game(
        ctx: Context<CreateGame>,
        max_items_per_player: u8,
        ap_per_player_creation: u64,
        ap_per_monster_spawn: u64,
        ap_per_monster_attack: u64
    ) -> Result<()> {
        run_create_game(
            ctx,
            max_items_per_player,
            ap_per_player_creation,
            ap_per_monster_spawn,
            ap_per_monster_attack
        )?;
        sol_log_compute_units();
        Ok(())
    }

    pub fn create_player(ctx: Context<CreatePlayer>) -> Result<()> {
        run_create_player(ctx)?;
        sol_log_compute_units();
        Ok(())
    }

    pub fn spawn_monster(ctx: Context<SpawnMonster>) -> Result<()> {
        run_spawn_monster(ctx)?;
        sol_log_compute_units();
        Ok(())
    }

    pub fn attack_monster(ctx: Context<AttackMonster>) -> Result<()> {
        run_attack_monster(ctx)?;
        sol_log_compute_units();
        Ok(())
    }

    pub fn deposit_action_points(ctx: Context<CollectActionPoints>) -> Result<()> {
        run_collect_action_points(ctx)?;
        sol_log_compute_units();
        Ok(())
    }
}