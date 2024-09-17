use anchor_lang::prelude::*;

use crate::{error::RpgError, Game, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(
        init,
        seeds = [b"GAME", treasury.key().as_ref()],
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
    if max_items_per_player == 0 {
        return Err(error!(RpgError::InvalidGameConfig));
    }

    let game = &mut ctx.accounts.game;
    game.game_master = ctx.accounts.game_master.key();
    game.treasury = ctx.accounts.treasury.key();
    game.action_points_collected = 0;
    game.game_config.max_items_per_player = max_items_per_player;

    msg!("Game created!");
    Ok(())
}
