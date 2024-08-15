use anchor_lang::prelude::*;

use crate::{
    spend_action_points, 
    Game, 
    Player, 
    ANCHOR_DISCRIMINATOR, 
    NO_EFFECT_FLAG
};

#[derive(Accounts)]
pub struct CreatePlayer<'info> {
    pub game: Box<Account<'info, Game>>,
    #[account(
        init, 
        seeds=[
            b"PLAYER", 
            game.key().as_ref(), 
            player.key().as_ref()
        ], 
        bump, 
        payer = player, 
        space = ANCHOR_DISCRIMINATOR + Player::INIT_SPACE
    )]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreatePlayer <'info> {
    pub fn run_create_player(&mut self) -> Result<()> {

        self.player_account.player = self.player.key().clone();
        self.player_account.game = self.game.key().clone();

        self.player_account.status_flag = NO_EFFECT_FLAG;
        self.player_account.experience = 0;
        self.player_account.kills = 0;

        msg!("Hero has entered the game!");

        {   
            // SOLUTION EDIT:
            let action_points_to_spend = self.game.game_config.ap_per_player_creation;

            spend_action_points(
                action_points_to_spend, 
                &mut self.player_account,
                &self.player.to_account_info(), 
                &self.system_program.to_account_info()
            )?;
        }

        Ok(())
    }
}