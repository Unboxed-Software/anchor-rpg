pub const IS_FROZEN_FLAG: u8 = 1 << 0;
pub const IS_POISONED_FLAG: u8 = 1 << 1;
pub const IS_BURNING_FLAG: u8 = 1 << 2;
pub const IS_BLESSED_FLAG: u8 = 1 << 3;
pub const IS_CURSED_FLAG: u8 = 1 << 4;
pub const IS_STUNNED_FLAG: u8 = 1 << 5;
pub const IS_SLOWED_FLAG: u8 = 1 << 6;
pub const IS_BLEEDING_FLAG: u8 = 1 << 7;
pub const NO_EFFECT_FLAG: u8 = 0b00000000;
pub const SPAWN_MONSTER_ACTION_POINTS: u64 = 5;
pub const CREATE_PLAYER_ACTION_POINTS: u64 = 100;
pub const ATTACK_ACTION_POINTS: u64 = 1;

pub const MAX_INVENTORY_ITEMS: usize = 8;

pub const ANCHOR_DISCRIMINATOR: usize = 8;