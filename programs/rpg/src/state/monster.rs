use anchor_lang::prelude::*;

#[account]
pub struct Monster { // 8 bytes
    pub player: Pubkey,                 // 32 bytes
    pub game: Pubkey,                   // 32 bytes
    pub hitpoints: u64,                 // 8 bytes
}