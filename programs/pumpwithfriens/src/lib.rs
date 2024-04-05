use anchor_lang::prelude::*;
use std::str::FromStr;
use anchor_lang::{solana_program::instruction::Instruction, system_program::{self, Transfer}};

declare_id!("22Jwcjoi8ffW8gG7LYqVX5mHK4u7RyN5xNf8TE1hsS9P");

fn lamports_per_token(remaining_percentage: f64) -> f64 {
    let a: f64 = 13169.68; // Estimated parameter 'a' of the power law curve
    let b: f64 = 1.705;    // Estimated parameter 'b' of the power law curve
    let lamports_per_sol: f64 = 1_000_000_000.0; // 1 SOL = 1,000,000,000 lamports

    // Calculate tokens per SOL using the bonding curve
    // should this be decimal or percentage?
    let tokens_per_sol = a * remaining_percentage.powf(b);

    // Invert the relationship to get lamports per token
    lamports_per_sol / tokens_per_sol
}

#[program]
pub mod pumpinator {

    use anchor_lang::solana_program::{program::invoke, program_pack::Pack};
    use anchor_spl::token::spl_token::state::Account;

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
        let bump = &[ctx.bumps.friend];

        let authority_seeds = &[
            b"friend",
            ctx.accounts.authority.key.as_ref(),
            bump
        ];
        let signers = &[&authority_seeds[..]];
                let cpi_context = CpiContext::new_with_signer(ctx.accounts.system_program.to_account_info().clone(), Transfer {
            from: ctx.accounts.friend.to_account_info(),
            to: ctx.accounts.authority.to_account_info(),
        }, signers);
        system_program::transfer(cpi_context, amount)?;
        Ok(())
    }
    
    
    pub fn pump<'info>(ctx: Context<'_, '_, '_, 'info, Pump<'info>>) -> Result<()> {
        let remaining_accounts = ctx.remaining_accounts;
        let curve_ata = remaining_accounts[4].to_account_info();
        let decoded_ata: Account = Account::unpack(&curve_ata.data.borrow())?;
        let total_supply = 1_000_000_000f64; // Total supply is a billion
        let remaining_amount = decoded_ata.amount as f64 / 1_000_000_f64; // Remaining amount

        // Calculate remaining percentage
        let remaining_percentage = (remaining_amount / total_supply) * 100f64; // Convert to percentage

        let lamports_per_token = lamports_per_token(remaining_percentage); //1300

        let data = hex::decode("66063d1201daebea972771084101000028e3600c00000000").unwrap();
        let amount = u64::from_le_bytes(data[8..16].try_into().unwrap()); // 13000000
        let lamports_per_amount = (lamports_per_token * amount as f64) as u64 / 1_000_000; // 13000000 * 1300 = 16900000

        let mut account_metas = ctx.remaining_accounts.to_vec().iter().map(|account| AccountMeta {
            pubkey: *account.key,
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        }).collect::<Vec<AccountMeta>>();
        // transfer all lamports to acocunt_metas[6] from freind
        account_metas[6].is_signer = true;

        let instruction = Instruction {
            program_id: Pubkey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap(),
            accounts: account_metas,
            data: data.clone()
        };
        let from_account =  &mut ctx.accounts.friend.to_account_info();
        let to_account = &mut ctx.accounts.jare.to_account_info();
        msg!("lamports_per_amount: {}", lamports_per_amount);
        { 
            let friend = &mut ctx.accounts.friend.load_mut()?;
            friend.owed += lamports_per_amount;
        }
        invoke(&instruction, &remaining_accounts)?; // down ? sol = 3.5sol
        


        // transfer remaining_accounts[2].pubkey mint from remaining_accounts[6].pubkey owner remaining_accounts[5].pubkey ata 
        // to friend account, ata remaining_accounts[-1].pubkey
        let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info().clone(), anchor_spl::token::Transfer {
            from: remaining_accounts[5].to_account_info(),
            to: remaining_accounts[remaining_accounts.len() - 1].to_account_info(),
            authority: ctx.accounts.jare.to_account_info(),
        });

        anchor_spl::token::transfer(cpi_context, amount)?;

        

        Ok(())
    }
    pub fn friend(ctx: Context<Friendly>) -> Result<()> {
        {
        let from = &mut ctx.accounts.friend.to_account_info();
        let to = &mut ctx.accounts.jare;
        let owed = ctx.accounts.friend.load()?.owed;
        **from.lamports.borrow_mut() -= owed;
        **to.lamports.borrow_mut() += owed;
    }
        {
            let friend = &mut ctx.accounts.friend.load_mut()?;
            friend.owed = 0;
            }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Friendly<'info> {
    #[account(mut, constraint = jare.key == &Pubkey::from_str("7ihN8QaTfNoDTRTQGULCzbUT3PHwPDTu5Brcu4iT2paP").unwrap())]
    pub jare: Signer<'info>,
    #[account(mut, seeds = [b"friend", authority.key.as_ref()], bump)]
    pub friend: AccountLoader<'info, Friend>,
    #[account(mut)]
    /// CHECK:
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pump<'info> {
    #[account(mut, constraint = jare.key == &Pubkey::from_str("7ihN8QaTfNoDTRTQGULCzbUT3PHwPDTu5Brcu4iT2paP").unwrap())]
    pub jare: Signer<'info>,
    #[account(mut, seeds = [b"friend", authority.key.as_ref()], bump)]
    pub friend: AccountLoader<'info, Friend>,
    #[account(mut)]
    /// CHECK: 
    pub authority: AccountInfo<'info>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub system_program: Program<'info, System>,
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
    pub owed: u64,
    pub buffer: [u64; 8],
}