use anchor_lang::{prelude::*, system_program};

use crate::{error::RpgError, Player};

pub fn spend_action_points<'info>(
    action_points: u64,
    player_account: &mut Account<'info, Player>,
    player: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
) -> Result<()> {
    player_account.action_points_spent = player_account.action_points_spent
        .checked_add(action_points)
        .ok_or(error!(RpgError::ArithmeticOverflow))?;

    player_account.action_points_to_be_collected = player_account.action_points_to_be_collected
        .checked_add(action_points)
        .ok_or(error!(RpgError::ArithmeticOverflow))?;

    // Transfer lamports from player to player_account
    system_program::transfer(
        CpiContext::new(
            system_program.to_account_info(),
            system_program::Transfer {
                from: player.to_account_info(),
                to: player_account.to_account_info(),
            }
        ),
        action_points
    )?;

    msg!("Spent {} action points", action_points);

    Ok(())
}