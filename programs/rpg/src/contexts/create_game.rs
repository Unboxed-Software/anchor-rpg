use anchor_lang::prelude::*;

use crate::{Game, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(
        init, 
        seeds=[b"GAME", treasury.key().as_ref()],
        bump,
        payer = game_master, 
        space = ANCHOR_DISCRIMINATOR + Game::INIT_SPACE
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub game_master: Signer<'info>,
    pub treasury: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn run_create_game(ctx: Context<CreateGame>, max_items_per_player: u8) -> Result<()> {

    ctx.accounts.game.game_master = ctx.accounts.game_master.key().clone();
    ctx.accounts.game.treasury = ctx.accounts.treasury.key().clone();

    ctx.accounts.game.action_points_collected = 0;
    ctx.accounts.game.game_config.max_items_per_player = max_items_per_player;

    msg!("Game created!");

    Ok(())
}