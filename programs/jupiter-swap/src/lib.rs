use anchor_lang::prelude::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use solana_program::{
    program::invoke_signed,
    pubkey::Pubkey,
    instruction::Instruction,
};

declare_id!("EqqmWW1mQC75v7g3R24ZGoVKVkRwNoxQZ7xGM3D7Sux9");

#[program]
pub mod jupiter_swap_contract {
    use super::*;

    pub fn swap_with_jupiter(
        ctx: Context<SwapWithJupiter>,
        amount_in: u64,
        minimum_amount_out: u64,
        jupiter_instruction_data: Vec<u8>,
    ) -> Result<()> {
        let jupiter_program_id = ctx.accounts.jupiter_program.key();
        let token_program_id = ctx.accounts.token_program.key();
        
        let mut cpi_accounts = vec![
            AccountMeta::new(ctx.accounts.user.key(), true), 
            AccountMeta::new(ctx.accounts.user_token_in.key(), false),
            AccountMeta::new(ctx.accounts.user_token_out.key(), false),
            AccountMeta::new_readonly(token_program_id, false), 
        ];

        let swap_instruction = Instruction {
            program_id: *jupiter_program_id,
            accounts: cpi_accounts,
            data: jupiter_instruction_data, 
        };
        invoke_signed(
            &swap_instruction,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.user_token_in.to_account_info(),
                ctx.accounts.user_token_out.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.jupiter_program.to_account_info(),
               
            ],
            &[], 
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SwapWithJupiter<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // The user initiating the swap
    #[account(mut)]
    pub user_token_in: Account<'info, TokenAccount>, // User's token account for the input token
    #[account(mut)]
    pub user_token_out: Account<'info, TokenAccount>, // User's token account for the output token
    #[account(address = jupiter_amm_v6::ID)] // Use Jupiter's actual program ID
    pub jupiter_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>, // The SPL Token program
    pub system_program: Program<'info, System>, // The System program
}


mod jupiter_amm_v6 {
    use solana_program::declare_id;
    declare_id!("JUP6LkbZGqvWAKXLWk5SdRNsxJojWsGJgRMahhabpyT"); // Jupiter Aggregator V6 Program ID
}
