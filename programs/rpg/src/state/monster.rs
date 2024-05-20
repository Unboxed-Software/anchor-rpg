use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Monster {
    pub player: Pubkey,
    pub game: Pubkey,
    pub hitpoints: u64,
}