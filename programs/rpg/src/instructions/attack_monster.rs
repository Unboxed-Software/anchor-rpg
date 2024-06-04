use anchor_lang::prelude::*;

use crate::{
    spend_action_points, 
    Game, 
    InventoryItem, 
    Monster, 
    Player, 
    BRONZE_SPEAR, 
    IS_CURSED_FLAG
};

#[derive(Accounts)]
pub struct AttackMonster<'info> {
     // SOLUTION EDIT:
     pub game: Box<Account<'info, Game>>,
    #[account(
        mut,
        has_one = player,
        // SOLUTION EDIT:
        has_one = game,
    )]
    pub player_account: Box<Account<'info, Player>>,
    #[account(
        mut,
        has_one = player,
        constraint = monster.game == player_account.game
    )]
    pub monster: Box<Account<'info, Monster>>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> AttackMonster<'info> {
    pub fn run_attack_monster(&mut self) -> Result<()> {

        let mut did_kill = false;

        let hp_before_attack =  self.monster.hitpoints;
        let hp_after_attack = self.monster.hitpoints.saturating_sub(1);
        let damage_dealt = hp_before_attack - hp_after_attack;
        self.monster.hitpoints = hp_after_attack;

            

        if hp_before_attack > 0 && hp_after_attack == 0 {
             did_kill = true;
        }

        if  damage_dealt > 0 {
            msg!("Damage Dealt: {}", damage_dealt);
        } else {
            msg!("Stop it's already dead!");

            // SOLUTION EDIT:
            self.player_account.status_flag |= IS_CURSED_FLAG;
        }


        self.player_account.experience = self.player_account.experience.saturating_add(1);
        msg!("+1 EXP");

        if did_kill {
            self.player_account.kills = self.player_account.kills.saturating_add(1);
            msg!("You killed the monster!");

            // SOLUTION EDIT:
            if self.player_account.inventory.len() < self.game.game_config.max_items_per_player as usize {
                self.player_account.inventory.push(
                    InventoryItem { 
                        name: BRONZE_SPEAR.clone(), 
                        amount: 1, 
                        for_future_use: [0; 128]
                    }
                );
                msg!("You looted the monster!");
            } else {
                msg!("You can't loot the monster, your inventory is full!");
            }

            // SOLUTION EDIT:
            let action_point_to_spend = self.game.game_config.ap_per_monster_attack;

            spend_action_points(
                action_point_to_spend, 
                &mut self.player_account,
                &self.player.to_account_info(), 
                &self.system_program.to_account_info()
            )?;
        }

        Ok(())
    }
}