use anchor_lang::prelude::*;

use crate::Game;

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(
        init, 
        seeds = [b"GAME", treasury.key().as_ref()],
        bump,
        payer = game_master, 
        space = 8 + Game::INIT_SPACE
    )]
    pub game: Account<'info, Game>,

    #[account(mut)]
    pub game_master: Signer<'info>,

    /// CHECK: Need to know they own the treasury
    pub treasury: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn run_create_game(
    ctx: Context<CreateGame>,
    max_items_per_player: u8,
    // SOLUTION EDIT: added game config ( could make this a struct )
    ap_per_player_creation: u64,
    ap_per_monster_spawn: u64,
    ap_per_monster_attack: u64,
) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let game_master = &ctx.accounts.game_master;
    let treasury = &ctx.accounts.treasury;

    game.game_master = game_master.key();
    game.treasury = treasury.key();

    game.action_points_collected = 0;
    game.game_config.max_items_per_player = max_items_per_player;
    // SOLUTION EDIT:
    game.game_config.ap_per_player_creation = ap_per_player_creation;
    game.game_config.ap_per_monster_spawn = ap_per_monster_spawn;
    game.game_config.ap_per_monster_attack = ap_per_monster_attack;

    msg!("Game created!");

    Ok(())
}