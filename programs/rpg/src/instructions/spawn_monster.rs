use anchor_lang::prelude::*;

use crate::{
    spend_action_points, 
    Game, 
    Monster,
    Player, 
    ANCHOR_DISCRIMINATOR
};

#[derive(Accounts)]
pub struct SpawnMonster<'info> {
    pub game: Box<Account<'info, Game>>,
    #[account(mut,
        has_one = game,
        has_one = player,
    )]
    pub player_account: Box<Account<'info, Player>>,
    #[account(
        init, 
        seeds=[
            b"MONSTER", 
            game.key().as_ref(), 
            player.key().as_ref(),
            player_account.next_monster_index.to_le_bytes().as_ref()
        ], 
        bump, 
        payer = player, 
        space = ANCHOR_DISCRIMINATOR + Monster::INIT_SPACE
    )]
    pub monster: Account<'info, Monster>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> SpawnMonster<'info> {
    pub fn run_spawn_monster(&mut self) -> Result<()> {
        self.monster.player = self.player.key().clone();
        self.monster.game = self.game.key().clone();
        self.monster.hitpoints = 100;

        msg!("Monster Spawned!");

        self.player_account.next_monster_index = self.player_account.next_monster_index.checked_add(1).unwrap();

        // SOLUTION EDIT:
        let action_point_to_spend = self.game.game_config.ap_per_monster_spawn;

        spend_action_points(
            action_point_to_spend, 
            &mut self.player_account,
            &self.player.to_account_info(), 
            &self.system_program.to_account_info()
        )?;

        Ok(())
    }
}