use anchor_lang::prelude::*;

#[account]
pub struct Game { // 8 bytes
    pub game_master: Pubkey,            // 32 bytes
    pub treasury: Pubkey,               // 32 bytes
    pub action_points_collected: u64,   // 8 bytes  
    pub game_config: GameConfig,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameConfig {
    pub max_items_per_player: u8,
    pub for_future_use: [u64; 16], // Health of Enemies?? Experience per item?? Action Points per Action??
}