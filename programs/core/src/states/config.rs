use crate::error::ErrorCode;
use anchor_lang::prelude::*;


pub const AMM_CONFIG_SEED: &str = "amm_config";

/// Holds the current owner of the factory
///
/// # The owner can
///
/// 1. Set and collect a pool's protocol fees
/// 2. Enable a new fee amount for pool creation
/// 3. Set another address as an owner
///
/// PDA of `[]`
///
#[account]
#[derive(Default, Debug)]
pub struct AmmConfig {
    /// Bump to identify PDA
    pub bump: u8,

    /// Address of the protocol owner
    pub owner: Pubkey,

    /// The global protocol fee
    pub protocol_fee_rate: u32,
    // padding space for upgrade
    // pub padding: [u64; 16],
}

impl AmmConfig {
    pub const LEN: usize = 8 + 1 + 32 + 1 + 128;

    pub fn is_authorized<'info>(
        &self,
        signer: &Signer<'info>,
        expect_pubkey: Pubkey,
    ) -> Result<()> {
        require!(
            signer.key() == self.owner || expect_pubkey == signer.key(),
            ErrorCode::NotApproved
        );
        Ok(())
    }
}

#[event]
pub struct CreateConfigEvent {
    /// The owner before the owner was changed
    #[index]
    pub owner: Pubkey,

    pub protocol_fee_rate: u32,
}

/// Emitted when the owner of the factory is changed
#[event]
pub struct OwnerChangedEvent {
    /// The owner before the owner was changed
    #[index]
    pub old_owner: Pubkey,

    /// The owner after the owner was changed
    #[index]
    pub new_owner: Pubkey,
}

/// Emitted when the protocol fee is changed for a pool
#[event]
pub struct SetProtocolFeeRateEvent {
    /// The previous value of the protocol fee
    pub protocol_fee_rate_old: u32,

    /// The updated value of the protocol fee
    pub protocol_fee_rate_new: u32,
}