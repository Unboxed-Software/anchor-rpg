use anchor_lang::prelude::*;

use crate::Game;

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(
        init, 
        seeds=[b"GAME", treasury.key().as_ref()],
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

impl<'info> CreateGame<'info> {
    pub fn run_create_game(&mut self,
        max_items_per_player: u8,
        // SOLUTION EDIT: added game config ( could make this a struct )
        ap_per_player_creation: u64,
        ap_per_monster_spawn: u64,
        ap_per_monster_attack: u64,
    ) -> Result<()> {

        self.game.game_master = self.game_master.key().clone();
        self.game.treasury = self.treasury.key().clone();

        self.game.action_points_collected = 0;
        self.game.game_config.max_items_per_player = max_items_per_player;
        // SOLUTION EDIT:
        self.game.game_config.ap_per_player_creation = ap_per_player_creation;
        self.game.game_config.ap_per_monster_spawn = ap_per_monster_spawn;
        self.game.game_config.ap_per_monster_attack = ap_per_monster_attack;

        msg!("Game created!");

        Ok(())
    }
}