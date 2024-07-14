use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount, Transfer, transfer},
};
use mpl_token_metadata::{
    accounts::{MasterEdition, Metadata as Meta}, 
    types::DataV2,
    };

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("E4KC2A8JJPK6GRiXEPhL64p8KvKke7BPdejD72noPkjP");

#[program]
mod hello_anchor {
    use super::*;
    
    pub fn init_nft(
        ctx: Context<MintNFT>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        // create mint account
        msg!("Creating mint account");
        let seeds = &["mint".as_bytes(), &[ctx.bumps.mint]];
        let signer = [&seeds[..]];


        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
            &signer,
        );
        mint_to(cpi_context, 1)?;

        // create metadata account
        msg!("Creating metadata account v3");
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                mint_authority: ctx.accounts.authority.to_account_info(),
                update_authority: ctx.accounts.authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &signer,
        );

        let data_v2 = DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(cpi_context, data_v2, true, true, None)?;

        // create master edition account
        msg!("Creating master edition v3");
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                update_authority: ctx.accounts.authority.to_account_info(),
                mint_authority: ctx.accounts.authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &signer
        );

        create_master_edition_v3(cpi_context, Some(1))?;

        msg!("Token mint created successfully.");


        Ok(())
    }

    pub fn swap_sol_for_nft(ctx: Context<LockNFT>, sol: u64, nft: u64) -> Result<()> {
        // Ensure the user has transferred the correct amount of $SOL
        let expected_sol_amount = nft; // For simplicity, assume 1 NFT = 1 SOL
        if sol != expected_sol_amount {
            return Err(ErrorCodes::Inequivalent.into());
        }

        // Transfer $SOL from the user to the protocol's treasury
        let cpi_accounts_sol = Transfer {
            from: ctx.accounts.user_nft_account.to_account_info(),
            to: ctx.accounts.vault_nft_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program_sol = ctx.accounts.system_program.to_account_info();
        let cpi_context_sol = CpiContext::new(cpi_program_sol, cpi_accounts_sol);
        transfer(cpi_context_sol, sol)?;

        // Transfer NFTs from the protocol's NFT vault to the user

        let bump = &[ctx.bumps.vault_nft_account];
        let seeds= &[b"vault".as_ref(), bump];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts_nft = Transfer {
            from: ctx.accounts.vault_nft_account.to_account_info(),
            to: ctx.accounts.user_nft_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program_nft = ctx.accounts.token_program.to_account_info();
        let cpi_context_nft = CpiContext::new(cpi_program_nft, cpi_accounts_nft).with_signer(signer_seeds);
        transfer(cpi_context_nft, nft)?;

        Ok(())
    }


    pub fn lock_nft(ctx: Context<LockNFT>, nft_a: u64) -> Result<()> {
    // Transfer NFT to vault
    let cpi_accounts_nft = Transfer {
        from: ctx.accounts.user_nft_account.to_account_info(),
        to: ctx.accounts.vault_nft_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program_nft = ctx.accounts.token_program.to_account_info();
    let cpi_context_nft = CpiContext::new(cpi_program_nft, cpi_accounts_nft);
    transfer(cpi_context_nft, nft_a)?;

    Ok(())

    }

    pub fn transfer_rental_fee(ctx: Context<LockNFT>, rental_fee: u64) -> Result<()> {
    // Handle rental fees (example)
    // Transfer rental fee to protocol account

    let bump = &[ctx.bumps.vault_nft_account];
    let seeds= &[b"vault".as_ref(), bump];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts_fee = Transfer {
        from: ctx.accounts.vault_nft_account.to_account_info(),
        to: ctx.accounts.protocol_fee_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program_fee = ctx.accounts.system_program.to_account_info();
    let cpi_context_fee = CpiContext::new(cpi_program_fee, cpi_accounts_fee).with_signer(signer_seeds);
    transfer(cpi_context_fee, rental_fee)?;

    Ok(())
    }
}


#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
        seeds = [b"mint"],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    pub token_metadata_program: Program<'info, Metadata>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    /// CHECK: address
    pub metadata_account: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            token_metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    /// CHECK: address
    pub master_edition_account: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct LockNFT<'info> {

    #[account(
        mut,
        seeds = [b"vault".as_ref()],
        bump
    )]
    pub vault_nft_account: SystemAccount<'info>,

    #[account(mut)]
    pub user_nft_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub protocol_fee_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    
    pub nft_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[error_code]
pub enum ErrorCodes {
    #[msg("Inequivalent SOL amount.")]
    Inequivalent,
}