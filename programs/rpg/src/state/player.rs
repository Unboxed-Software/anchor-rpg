use anchor_lang::prelude::*;

use crate::MAX_INVENTORY_ITEMS;

#[account]
#[derive(InitSpace)]
pub struct Player {
    pub player: Pubkey,
    pub game: Pubkey,
    pub action_points_spent: u64,
    pub action_points_to_be_collected: u64,
    pub status_flag: u8,
    pub experience: u64,
    pub kills: u64,
    pub next_monster_index: u64,
    pub for_future_use: [u8; 256],  // Attack/Speed/Defense/Health/Mana? Metadata?
    #[max_len(MAX_INVENTORY_ITEMS)] // Max 8 items
    pub inventory: Vec<InventoryItem>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct InventoryItem {
    pub name: [u8; 32], // Fixed Name up to 32 bytes
    pub amount: u64,
    pub for_future_use: [u8; 128], // Metadata? Effects? Flags?
}