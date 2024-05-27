use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Game {
    pub game_master: Pubkey,
    pub treasury: Pubkey,
    pub action_points_collected: u64, 
    pub game_config: GameConfig,
}

// ----------- GAME CONFIG ----------

#[derive(Clone, AnchorSerialize, AnchorDeserialize, InitSpace)]
pub struct GameConfig {
    pub max_items_per_player: u8,  
    // SOLUTION EDIT:
    pub ap_per_player_creation: u64,
    pub ap_per_monster_spawn: u64,
    pub ap_per_monster_attack: u64,
    // SOLUTION EDIT: 16 -> 13
    pub for_future_use: [u64; 13], // Health of Enemies?? Experince per item?? Action Points per Action??
}