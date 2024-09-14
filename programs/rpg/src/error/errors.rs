use anchor_lang::prelude::*;

#[error_code]
pub enum RpgError {
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    #[msg("Invalid game configuration")]
    InvalidGameConfig,
    #[msg("Player not found")]
    PlayerNotFound,
    #[msg("Monster not found")]
    MonsterNotFound,
    #[msg("Insufficient action points")]
    InsufficientActionPoints,
    #[msg("Invalid attack")]
    InvalidAttack,
    #[msg("Maximum inventory size reached")]
    MaxInventoryReached,
    #[msg("Invalid item operation")]
    InvalidItemOperation,
    #[msg("Monster and player are not in the same game")]
    GameMismatch,
    #[msg("Invalid treasury account")]
    InvalidTreasury,
    #[msg("Player does not belong to the specified game")]
    PlayerGameMismatch,
    #[msg("Insufficient funds for transfer")]
    InsufficientFunds
}