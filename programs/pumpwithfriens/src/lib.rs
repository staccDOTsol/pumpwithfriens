use anchor_lang::prelude::*;
use std::str::FromStr;
use anchor_lang::system_program::{self, Transfer};

declare_id!("cAQ5uztwVrNw7Us1GbCLmsdSdQtUmwtS23mj9y5hhXy");

#[program]
pub mod pumpinator {

    use anchor_spl::token_2022::spl_token_2022::extension::confidential_transfer_fee::instruction;
    use pump::BondingCurve;
    use spl_token::solana_program::program::invoke_signed;

    use super::*;
/*
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let friend = &mut ctx.accounts.friend.load_init()?;
        friend.authority = *ctx.accounts.authority.key;
        Ok(())
    }*/
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(ctx.accounts.system_program.to_account_info().clone().clone(), Transfer {
            from: ctx.accounts.authority.to_account_info().clone(),
            to: ctx.accounts.friend.to_account_info().clone(),
        });
        system_program::transfer(cpi_context, amount)?;
        let friend = &mut ctx.accounts.friend.load_mut()?;
        friend.deposited+=amount;
        Ok(())
    }
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        {
            let from = &mut ctx.accounts.friend.to_account_info().clone();
            let to = &mut ctx.accounts.authority.to_account_info().clone();
            **from.lamports.borrow_mut() -= amount;
            **to.lamports.borrow_mut() += amount;
        }
           let friend = &mut ctx.accounts.friend.load_mut()?;
           friend.withdrawn+=amount;
            Ok(())

    }
    use anchor_lang::solana_program::{
        instruction::Instruction,
        program::invoke,
    };
    
    pub fn pump<'info>(ctx: Context<'_, '_, '_, 'info, Pump<'info>>, amount: u64) -> Result<()> {
        let bump = &[ctx.bumps.friend];
        
        let authority_seeds = &[
            b"friend",
            ctx.accounts.authority.key.as_ref(),
            bump,
        ];
        let remaining_accounts = &ctx.remaining_accounts;
        let friend = &ctx.accounts.friend.to_account_info();
        let jare = &ctx.accounts.jare;
        let curve_account_info = &ctx.remaining_accounts[3];
        
        let mut curve = curve_account_info.try_borrow_mut_data()?;
        let curve_data = BondingCurve::try_deserialize(&mut curve.as_ref())?;
        let amount_sol = curve_data.buy_quote(amount.into());
        drop(curve);

        msg!("Amount of SOL to buy: {}", amount_sol);
        msg!("Friend lamports: {}", friend.lamports());
        if friend.lamports() > amount_sol + 1_000_000 {

            **friend.to_account_info().lamports.borrow_mut() -= (amount_sol + 1_000_000);
            msg!("Friend lamports after: {}", friend.lamports());
            **jare.to_account_info().lamports.borrow_mut() += (amount_sol + 1_000_000);
            msg!("Jare lamports after: {}", jare.lamports());
            // Adjust lamports directly on the account info objects
            let signer_seeds = &[&authority_seeds[..]];
            let d = hex::decode("66063d1201daebea00e8684e41010000785a5e0500000000").unwrap();

            let mut data = Vec::new();
            data.extend_from_slice(d[0..8].as_ref());
            data.extend_from_slice(&amount.to_le_bytes());
            data.extend_from_slice(&(u64::MAX-1).to_le_bytes());
            // Create the in    struction for the pump::cpi::buy call
            let ix = Instruction {
                program_id: ctx.remaining_accounts[11].key.clone(),
                accounts: vec![
                    AccountMeta::new_readonly(*remaining_accounts[0].key, false),
                    AccountMeta::new_readonly(*remaining_accounts[1].key, false),
                    AccountMeta::new(*remaining_accounts[2].key, false),
                    AccountMeta::new(*remaining_accounts[3].key, false),
                    AccountMeta::new(*remaining_accounts[4].key, false),
                    AccountMeta::new(*remaining_accounts[5].key, false),
                    AccountMeta::new(*remaining_accounts[6].key, true),
                    AccountMeta::new_readonly(*remaining_accounts[7].key, false),
                    AccountMeta::new_readonly(*remaining_accounts[8].key, false),
                    AccountMeta::new_readonly(*remaining_accounts[9].key, false),
                    AccountMeta::new_readonly(*remaining_accounts[10].key, false),
                    AccountMeta::new_readonly(*remaining_accounts[11].key, false),
                ],
                data
            };
        
            // Invoke the instruction
            invoke_signed(
                &ix,
                &[
                    remaining_accounts[0].to_account_info(),
                    remaining_accounts[1].to_account_info(),
                    remaining_accounts[2].to_account_info(),
                    remaining_accounts[3].to_account_info(),
                    remaining_accounts[4].to_account_info(),
                    remaining_accounts[5].to_account_info(),
                    remaining_accounts[6].to_account_info(),
                    remaining_accounts[7].to_account_info(),
                    remaining_accounts[8].to_account_info(),
                    remaining_accounts[9].to_account_info(),
                    remaining_accounts[10].to_account_info(),
                    remaining_accounts[11].to_account_info(),
                ],
                signer_seeds
            )?;
            msg!("Bought amount: {}", amount);
        
            // Transfer remaining_accounts[2].pubkey mint from remaining_accounts[6].pubkey owner remaining_accounts[5].pubkey ata 
            // to friend account, ata remaining_accounts[-1].pubkey
            let transfer_ix = spl_token::instruction::transfer(
                &ctx.accounts.token_program.to_account_info().key,
                &ctx.remaining_accounts[5].to_account_info().key,
                &ctx.remaining_accounts[ctx.remaining_accounts.len() - 1].to_account_info().key,
                &ctx.accounts.jare.to_account_info().key,
                &[],
                amount,
            )?;
        
            invoke(
                &transfer_ix,
                &[
                    ctx.accounts.token_program.to_account_info(),
                    ctx.remaining_accounts[5].to_account_info(),
                    ctx.remaining_accounts[ctx.remaining_accounts.len() - 1].to_account_info(),
                    ctx.accounts.jare.to_account_info(),
                ],
            )?;
            msg!("Transferred amount: {}", amount);

            
        }
        
        Ok(())
    }
    pub fn unpump<'info>(ctx: Context<'_, '_, '_, 'info, Pump<'info>>, amount: u64) -> Result<()> {
        let bump = &[ctx.bumps.friend];
        
        let authority_seeds = &[
            b"friend",
            ctx.accounts.authority.key.as_ref(),
            bump,
        ];
    
        let signer_seeds = &[&authority_seeds[..]];
        let remaining_accounts = &ctx.remaining_accounts;
        let friend = &ctx.accounts.friend;
        let jare = &ctx.accounts.jare;
        let curve_account_info = &remaining_accounts[3];
        let fkey = friend.to_account_info().key;
        let mut curve = curve_account_info.try_borrow_mut_data()?;
        let curve_data = BondingCurve::try_deserialize(&mut curve.as_ref())?;
        let amount_sol = curve_data.sell_quote(amount.into());
        let adjustment = amount_sol + 1_000_000;
        drop(curve);
        // Adjust lamports directly on the account info objects
        **friend.to_account_info().lamports.borrow_mut() -= adjustment;
        **jare.to_account_info().lamports.borrow_mut() += adjustment;
        let d = hex::decode("33e685a4017f83ad00e8684e410100004c234d0100000000").unwrap();
        let mut  data = Vec::new();
        data.extend_from_slice(d[0..8].as_ref());
        data.extend_from_slice(&amount.to_le_bytes());
        data.extend_from_slice(&0u64.to_le_bytes());
        let sell_ix = Instruction {
            program_id: remaining_accounts[11].key.clone(),
            accounts: vec![
                AccountMeta::new_readonly(*remaining_accounts[0].key, false),
                AccountMeta::new_readonly(*remaining_accounts[1].key, false),
                AccountMeta::new(*remaining_accounts[2].key, false),
                AccountMeta::new(*remaining_accounts[3].key, false),
                AccountMeta::new(*remaining_accounts[4].key, false),
                AccountMeta::new(*remaining_accounts[5].key, false),
                AccountMeta::new(*remaining_accounts[6].key, true),
                AccountMeta::new_readonly(*remaining_accounts[7].key, false),
                AccountMeta::new_readonly(*remaining_accounts[8].key, false),
                AccountMeta::new(*remaining_accounts[9].key, false),
                AccountMeta::new_readonly(*remaining_accounts[10].key, false),
                AccountMeta::new_readonly(*remaining_accounts[11].key, false),
            ],
            data
        };
    
        // Invoke the instruction
        invoke_signed(
            &sell_ix,
            &[
                remaining_accounts[0].to_account_info(),
                remaining_accounts[1].to_account_info(),
                remaining_accounts[2].to_account_info(),
                remaining_accounts[3].to_account_info(),
                remaining_accounts[4].to_account_info(),
                remaining_accounts[5].to_account_info(),
                remaining_accounts[6].to_account_info(),
                remaining_accounts[7].to_account_info(),
                remaining_accounts[8].to_account_info(),
                remaining_accounts[9].to_account_info(),
                remaining_accounts[10].to_account_info(),
                remaining_accounts[11].to_account_info(),
            ],
            signer_seeds
        )?;
    
        // Transfer 'adjustment' from 'jare' to 'friend'
        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.jare.key,
            &fkey,
            adjustment,
        );
    
        invoke(
            &transfer_ix,
            &[
                ctx.accounts.jare.to_account_info(),
                ctx.accounts.friend.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    
        // Transfer remaining_accounts[2].pubkey mint from remaining_accounts[6].pubkey owner remaining_accounts[5].pubkey ata 
        // to friend account, ata remaining_accounts[-1].pubkey
        let token_transfer_ix = spl_token::instruction::transfer(
            &ctx.accounts.token_program.to_account_info().key,
            &remaining_accounts[5].to_account_info().key,
            &remaining_accounts[remaining_accounts.len() - 1].to_account_info().key,
            &fkey,
            &[],
            amount,
        )?;
    
        invoke(
            &token_transfer_ix,
            &[
                ctx.accounts.token_program.to_account_info(),
                remaining_accounts[5].to_account_info(),
                remaining_accounts[remaining_accounts.len() - 1].to_account_info(),
                ctx.accounts.friend.to_account_info(),
            ],
        )?;
    
        Ok(())
    }

}
#[derive(Accounts)]
pub struct Pump<'info> {
    #[account(mut, constraint = jare.key == &Pubkey::from_str("Czbmb7osZxLaX5vGHuXMS2mkdtZEXyTNKwsAUUpLGhkG").unwrap())]
    pub jare: Signer<'info>,
    #[account(mut, seeds = [b"friend", authority.key.as_ref()], bump)]
    pub friend: AccountLoader<'info, Friend>,
    #[account(mut)]
    /// CHECK: 
    pub authority: AccountInfo<'info>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub system_program: Program<'info, System>,
}
/* 
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

} */
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
    pub deposited: u64,
    pub withdrawn: u64,
}