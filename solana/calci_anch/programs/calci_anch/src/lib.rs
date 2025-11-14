use anchor_lang::prelude::*;

declare_id!("rNcZmmYUsV9vkgPDmhwTdAufbBAfJ1oitJnzcoR13CZ");

#[program]
pub mod calci_anch {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from Calculator Program: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn add(ctx: Context<Add>) -> Result<()> {
        msg!("Greetings from Calculator Program: {:?}", ctx.program_id);
        Ok(())
    }
}

struct CalciResult{
    calci_result: u8,
    payer: pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {} {
    #[account(mut)]
    fee_payer: Signer<'info>,

    #[account(init, space=8+1+32, payer=fee_payer)]
    calci_acc: Account<'info, CalciResult>,

    system_program: Program<'Info, System>,
}

#[derive(Accounts)]
pub struct Add {}