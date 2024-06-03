// SOLUTION EDIT: Added in an item drop
pub const BRONZE_SPEAR: [u8; 32] = [
    b'b', b'r', b'o', b'n', b'z', b'e', b',', b' ', b's', b'p', b'e', b'a', b'r',
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];

pub const IS_FROZEN_FLAG: u8 = 1 << 0;
pub const IS_POISONED_FLAG: u8 = 1 << 1;
pub const IS_BURNING_FLAG: u8 = 1 << 2;
pub const IS_BLESSED_FLAG: u8 = 1 << 3;
pub const IS_CURSED_FLAG: u8 = 1 << 4;
pub const IS_STUNNED_FLAG: u8 = 1 << 5;
pub const IS_SLOWED_FLAG: u8 = 1 << 6;
pub const IS_BLEEDING_FLAG: u8 = 1 << 7;
pub const NO_EFFECT_FLAG: u8 = 0b00000000;
pub const ANCHOR_DISCRIMINATOR: usize = 8;
pub const MAX_INVENTORY_ITEMS: usize = 8;