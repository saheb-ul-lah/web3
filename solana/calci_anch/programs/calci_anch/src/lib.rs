use anchor_lang::prelude::*;

declare_id!("rNcZmmYUsV9vkgPDmhwTdAufbBAfJ1oitJnzcoR13CZ");

#[program]
pub mod calci_anch {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let calci_acc = &mut ctx.accounts.calci_acc;
        calci_acc.calci_result = 0;
        calci_acc.payer = ctx.accounts.fee_payer.key();

        msg!("Calculator initialized. Result set to 0");
        Ok(())
    }

    pub fn add(ctx: Context<Add>, a: u8, b: u8) -> Result<()> {
        let calci_acc = &mut ctx.accounts.calci_acc;
        calci_acc.calci_result = a + b;

        msg!("Addition result stored: {}", calci_acc.calci_result);
        Ok(())
    }
}

#[account]
pub struct CalciResult {
    pub calci_result: u8,
    pub payer: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub fee_payer: Signer<'info>,

    #[account(
        init,
        space = 8 + 1 + 32,
        payer = fee_payer
    )]
    pub calci_acc: Account<'info, CalciResult>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub calci_acc: Account<'info, CalciResult>,
}
