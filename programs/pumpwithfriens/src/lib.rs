use anchor_lang::prelude::*;
use std::str::FromStr;
use anchor_lang::{solana_program::{instruction::Instruction, program::invoke_signed}, system_program::{self, Transfer}};

declare_id!("CniMsrpSgFcRahq8aZVYaPvoXLxbr5CtaMTdGj4KjTG2");

#[program]
pub mod pumpinator {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let friend = &mut ctx.accounts.friend.load_init()?;
        friend.authority = *ctx.accounts.authority.key;
        Ok(())
    }
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(ctx.accounts.system_program.to_account_info().clone(), Transfer {
            from: ctx.accounts.authority.to_account_info(),
            to: ctx.accounts.friend.to_account_info(),
        });
        system_program::transfer(cpi_context, amount)?;
        Ok(())
    }
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let seeds: &[&[_]] = &[b"friend", ctx.accounts.authority.key.as_ref()];
        let bump = &[ctx.bumps.friend];
        let signers_seeds = &[seeds, &[bump]];
        let cpi_context = CpiContext::new_with_signer(ctx.accounts.system_program.to_account_info().clone(), Transfer {
            from: ctx.accounts.friend.to_account_info(),
            to: ctx.accounts.authority.to_account_info(),
        }, signers_seeds);
        system_program::transfer(cpi_context, amount)?;
        Ok(())
    }
    pub fn pump(ctx: Context<Pump>, data: Vec<u8>) -> Result<()> {
        let remaining_accounts = ctx.remaining_accounts;
        let keys = remaining_accounts.iter().map(|account| account.to_account_info()).collect::<Vec<_>>();
        assert!(keys[6].key == ctx.accounts.friend.to_account_info().key);
        let data = data.to_vec();
        let seeds: &[&[_]] = &[b"friend", ctx.accounts.authority.key.as_ref()];
        let bump = &[ctx.bumps.friend];
        let signers_seeds = &[seeds, &[bump]];
        let instruction = Instruction {
            program_id: Pubkey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap(),
            accounts: keys.to_account_metas(Some(true)),
            data
        };
        invoke_signed(&instruction, &remaining_accounts, signers_seeds)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Pump<'info> {
    #[account(mut, constraint = jare.key == &Pubkey::from_str("7ihN8QaTfNoDTRTQGULCzbUT3PHwPDTu5Brcu4iT2paP").unwrap())]
    pub jare: Signer<'info>,
    #[account(mut, seeds = [b"friend", authority.key.as_ref()], bump)]
    pub friend: AccountLoader<'info, Friend>,
    #[account(mut)]
    /// CHECK: 
    pub authority: AccountInfo<'info>
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init,
        seeds = [b"friend", authority.key.as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<Friend>()+8,
    )]
    pub friend: AccountLoader<'info, Friend>,
    pub system_program: Program<'info, System>,

}
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,
        seeds = [b"friend", authority.key.as_ref()],
        bump)]
    pub friend: AccountLoader<'info, Friend>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,
        seeds = [b"friend", authority.key.as_ref()],
        bump)]
    pub friend: AccountLoader<'info, Friend>,
    pub system_program: Program<'info, System>,
}

#[account(zero_copy)]   
pub struct Friend {
    pub authority: Pubkey,
    pub buffer: [u8; 512]
}