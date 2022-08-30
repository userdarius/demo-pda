use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod demo_pda {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        // Get Escrow Account
        let escrow = &mut ctx.accounts.escrow;

        // Set from
        escrow.from = ctx.accounts.from.key();

        // Set to 
        escrow.to = ctx.accounts.to.key();

        // Set amount
        escrow.amount = amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    // Escrow account PDA
    #[account(
        init,
        // State account seed uses the string "escrow" and the users' key
        // Note that we can only have 1 active trx
        seeds = [b"escrow".as_ref(), from.key().as_ref(), to.key().as_ref()],
        bump,
        payer = from,
        space = size_of::<EscrowAccount>()
    )]
    pub escrow: Account<'info, EscrowAccount>,

    #[account(mut)]
    pub from: Signer<'info>,
    /// CHECK: safe
    #[account(mut)]
    pub to: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct EscrowAccount {
    // from address
    pub from: Pubkey,

    // to address
    pub to: Pubkey,

    // Amount that is owed
    pub amount: u64,
}
