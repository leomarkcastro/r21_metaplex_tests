use anchor_lang::{
    prelude::*,
    solana_program::program::invoke,
    system_program::{create_account, CreateAccount},
};
use anchor_spl::{
    associated_token,
    token::{self, Mint, MintTo, Token, TokenAccount, Transfer},
};
use mpl_token_metadata::{instruction as token_instruction, ID as TOKEN_METADATA_ID};

declare_id!("7ghLrtu6EqZuRcNQX5cvWp8THJ6tgfbSXEAKZ8GhVRy4");

mod mpl_simplified_methods {
    use super::*;

    pub fn _create_account<'info>(
        system_program: AccountInfo<'info>,
        owner_account: AccountInfo<'info>,
        minter_account: AccountInfo<'info>,
        token_program: AccountInfo<'info>,
    ) -> Result<()> {
        create_account(
            CpiContext::new(
                // We'll use the system program to create account, but token_program also has the create_account function
                system_program.clone(), // Target program
                CreateAccount {
                    from: owner_account.clone(), // From pubkey
                    to: minter_account.clone(),  // To pubkey
                },
            ),
            10000000,             // Lamports (1 SOL)
            82,                   // Space (has to be exact, idk where this value comes from)
            &token_program.key(), // Owner
        )?;
        Ok(())
    }

    pub fn _initialize_mint_account<'info>(
        owner_account: AccountInfo<'info>,
        minter_account: AccountInfo<'info>,
        token_program: AccountInfo<'info>,
        rent_program: AccountInfo<'info>,
    ) -> Result<()> {
        token::initialize_mint(
            CpiContext::new(
                token_program.clone(), // Target program
                token::InitializeMint {
                    mint: minter_account.clone(), // Mint pubkey
                    rent: rent_program.clone(),   // Rent pubkey
                },
            ),
            0,                          // decimals
            &owner_account.key(),       // mint authority
            Some(&owner_account.key()), // freeze authority
        )?;
        Ok(())
    }

    pub fn _initialize_token_holder_account<'info>(
        owner_account: AccountInfo<'info>,
        mint_account: AccountInfo<'info>,
        token_holder_account: AccountInfo<'info>,
        associated_token_program: AccountInfo<'info>,
        token_program: AccountInfo<'info>,
        rent_program: AccountInfo<'info>,
        system_program: AccountInfo<'info>,
    ) -> Result<()> {
        associated_token::create(CpiContext::new(
            associated_token_program.clone(), // Target program
            associated_token::Create {
                payer: owner_account.clone(),                   // Payer pubkey
                associated_token: token_holder_account.clone(), // Associated token pubkey
                authority: owner_account.clone(),               // Authority pubkey
                mint: mint_account.clone(),                     // Mint pubkey
                system_program: system_program.clone(),         // System program pubkey
                token_program: token_program.clone(),           // Token program pubkey
                rent: rent_program.clone(),                     // Rent program pubkey
            },
        ))
    }

    pub fn _mint_token_to_account<'info>(
        minter_account: AccountInfo<'info>,
        token_holder_account: AccountInfo<'info>,
        owner_account: AccountInfo<'info>,
        token_program: AccountInfo<'info>,
    ) -> Result<()> {
        // Minting Token
        token::mint_to(
            CpiContext::new(
                token_program.clone(),
                MintTo {
                    mint: minter_account.clone(),
                    to: token_holder_account.clone(),
                    authority: owner_account.clone(),
                },
            ),
            1,
        )
    }

    pub fn _create_metadata_account<'info>(
        metadata_account: AccountInfo<'info>,
        minter_account: AccountInfo<'info>,
        authority_account: AccountInfo<'info>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: minter_account.clone().key(),
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: authority_account.clone().key(),
                verified: false,
                share: 0,
            },
        ];

        // Minting Metadata
        invoke(
            &token_instruction::create_metadata_accounts_v3(
                TOKEN_METADATA_ID,               // Target Program Address
                metadata_account.clone().key(),  // Metadata Account
                minter_account.clone().key(),    // Minter Account
                authority_account.clone().key(), // Authority Account
                authority_account.clone().key(), // Payer Account
                authority_account.clone().key(), // Update Authority Account
                metadata_title,                  // Metadata Title
                metadata_symbol,                 // Metadata Symbol
                metadata_uri,                    // Metadata URI
                Some(creator),                   // Creators
                1,                               // Seller Fee Basis Points
                true,                            // Update Authority is Signer
                true,                            // Is Mutable
                None,                            // Collection
                None,                            // Uses
                None,                            // Collection Details
            ),
            &[
                metadata_account.clone(),  // Metadata Account
                minter_account.clone(),    // Minter Account
                authority_account.clone(), // Authority Account
            ],
        )?;

        Ok(())
    }

    pub fn _create_master_edition_account<'info>(
        master_edition: AccountInfo<'info>,
        minter_account: AccountInfo<'info>,
        token_holder_account: AccountInfo<'info>,
        metadata_account: AccountInfo<'info>,
        authority_account: AccountInfo<'info>,
        rent_program: AccountInfo<'info>,
    ) -> Result<()> {
        invoke(
            &token_instruction::create_master_edition_v3(
                TOKEN_METADATA_ID,               // Target Program Address
                master_edition.clone().key(),    // Master Edition Account
                minter_account.clone().key(),    // Minter Account
                authority_account.clone().key(), // Update Account
                authority_account.clone().key(), // Mint Account
                metadata_account.clone().key(),  // Metadata Account
                authority_account.clone().key(), // Payer Account
                Some(1),                         // Max Supply
            ),
            &[
                master_edition.clone(),       // Master Edition Account
                metadata_account.clone(),     // Metadata Account
                minter_account.clone(),       // Minter Account
                token_holder_account.clone(), // Token Holder Account
                authority_account.clone(),    // Authority Account
                rent_program.clone(),         // Rent Account
            ],
        )?;

        Ok(())
    }

    pub fn _update_metadata_account<'info>(
        metadata_account: AccountInfo<'info>,
        authority_account: AccountInfo<'info>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        invoke(
            &token_instruction::update_metadata_accounts_v2(
                TOKEN_METADATA_ID,               // Target Program Address
                metadata_account.clone().key(),  // Metadata Account
                authority_account.clone().key(), // Payer Account
                None,                            // Update Authority Account
                Some(mpl_token_metadata::state::DataV2 {
                    name: metadata_title,
                    symbol: metadata_symbol,
                    uri: metadata_uri,
                    seller_fee_basis_points: 1,
                    creators: None,
                    collection: None,
                    uses: None,
                }), // Data
                None,                            // Primary Sale
                None,                            // Is Mutable
            ),
            &[
                metadata_account.clone(),  // Metadata Account
                authority_account.clone(), // Authority Account
            ],
        )?;

        Ok(())
    }
}

#[program]
pub mod r21_metaplex_tests {
    use super::*;
    use crate::mpl_simplified_methods::*;

    // region: Unit Test Only --- Initialize NFT

    pub fn initialize_nft(ctx: Context<Initialize>) -> Result<()> {
        let owner_account = ctx.accounts.owner_account.to_account_info();
        let minter_account = ctx.accounts.minter_account.to_account_info();

        let token_program = ctx.accounts.token_program.to_account_info();
        let rent_program = ctx.accounts.rent.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        /*
         *  [Create Mint Account]
         *
         *    - Using the system program, we create a new account for the mint.
         *    - The target program would be the token program.
         *    - We pass the pubkey of the user as the from pubkey.
         *    - We pass the pubkey of the mint (account) as the to pubkey. (The account address we want to create)
         *    - The lamports is 1 SOL.
         *    - The space is 82 bytes (idk where this value comes from honestly).
         *    - The account owner is the token program.
         */

        _create_account(
            system_program.clone(),
            owner_account.clone(),
            minter_account.clone(),
            token_program.clone(),
        )?;
        msg!("Mint Account Created!!!");

        /*
         *  [Initialize Mint Account]
         *
         *   - We initialize the mint account.
         *   - The target program would be the token program.
         *   - We pass the pubkey of the mint (account) as the mint pubkey.
         *   - We pass the pubkey of the rent program as the rent pubkey.
         *   - The decimals is 0.
         *   - The mint authority is the user.
         *   - The freeze authority is the user.
         */

        _initialize_mint_account(
            owner_account.clone(),
            minter_account.clone(),
            token_program.clone(),
            rent_program.clone(),
        )?;
        msg!("Minter Initialized!!!");

        Ok(())
    }

    pub fn create_nft_holder(ctx: Context<CreateNFTHolder>) -> Result<()> {
        /*
         *  [Create Associate Token Account]
         *
         *   - We create an associate token account.
         *     - The target program would be the associated token program.
         *     - We pass the pubkey of the user as the payer pubkey.
         *     - We pass the pubkey of the token account as the associated token pubkey.
         *     - We pass the pubkey of the user as the authority pubkey.
         *     - We pass the pubkey of the mint as the mint pubkey.
         *     - We pass the pubkey of the system program as the system program pubkey.
         *     - We pass the pubkey of the token program as the token program pubkey.
         *     - We pass the pubkey of the rent program as the rent program pubkey.
         */

        // Either uncomment this or uncomment the macro of this context
        let user_account = ctx.accounts.user_account.to_account_info();
        let mint_account = ctx.accounts.minter_account.to_account_info();
        let token_holder_account = ctx.accounts.token_holder_account.to_account_info();

        let associated_token_program = ctx.accounts.associated_token_program.to_account_info();
        let token_program = ctx.accounts.token_program.to_account_info();
        let rent_program = ctx.accounts.rent.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        _initialize_token_holder_account(
            user_account.clone(),
            mint_account.clone(),
            token_holder_account.clone(),
            associated_token_program.clone(),
            token_program.clone(),
            rent_program.clone(),
            system_program.clone(),
        )?;
        msg!("Associate Token Account Created!!!");

        Ok(())
    }

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        let authority_account = ctx.accounts.authority_account.to_account_info();
        let minter_account = ctx.accounts.minter_account.to_account_info();
        let token_holder_account = ctx.accounts.token_holder_account.to_account_info();
        let metadata_account = ctx.accounts.metadata_account.to_account_info();
        let master_edition = ctx.accounts.master_edition_account.to_account_info();

        let token_program = ctx.accounts.token_program.to_account_info();
        let rent_program = ctx.accounts.rent.to_account_info();

        // Minting Token
        _mint_token_to_account(
            minter_account.clone(),
            token_holder_account.clone(),
            authority_account.clone(),
            token_program.clone(),
        )?;
        msg!("Token Minted!!!");

        _create_metadata_account(
            metadata_account.clone(),
            minter_account.clone(),
            authority_account.clone(),
            metadata_title,
            metadata_symbol,
            metadata_uri,
        )?;
        msg!("Metadata Minted!!!");

        // Creating Master Edition Metadata
        _create_master_edition_account(
            master_edition.clone(),
            minter_account.clone(),
            token_holder_account.clone(),
            metadata_account.clone(),
            authority_account.clone(),
            rent_program.clone(),
        )?;
        msg!("Master Edition Minted!!!");

        Ok(())
    }

    pub fn transfer_nft(ctx: Context<TransferNFT>) -> Result<()> {
        let authority_account = ctx.accounts.authority.to_account_info();
        let sender_account = ctx.accounts.sender.to_account_info();
        let receiver_account = ctx.accounts.recipient.to_account_info();

        let token_program = ctx.accounts.token_program.to_account_info();

        // Transfer Token
        token::transfer(
            CpiContext::new(
                token_program.clone(),
                Transfer {
                    from: sender_account.clone(),
                    to: receiver_account.clone(),
                    authority: authority_account.clone(),
                },
            ),
            1,
        )?;
        msg!("Token Transferred!!!");

        Ok(())
    }

    // endregion

    // region: Official Functions

    pub fn create_nft(
        ctx: Context<CreateNFT>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        let owner_account = ctx.accounts.authority_account.to_account_info();
        let minter_account = ctx.accounts.minter_account.to_account_info();
        let token_holder_account = ctx.accounts.token_holder_account.to_account_info();
        let metadata_account = ctx.accounts.metadata_account.to_account_info();
        let master_edition = ctx.accounts.master_edition_account.to_account_info();

        let associated_token_program = ctx.accounts.associated_token_program.to_account_info();
        let token_program = ctx.accounts.token_program.to_account_info();
        let rent_program = ctx.accounts.rent.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        _create_account(
            system_program.clone(),
            owner_account.clone(),
            minter_account.clone(),
            token_program.clone(),
        )?;
        msg!("Mint Account Created!!!");

        _initialize_mint_account(
            owner_account.clone(),
            minter_account.clone(),
            token_program.clone(),
            rent_program.clone(),
        )?;
        msg!("Minter Initialized!!!");

        _initialize_token_holder_account(
            owner_account.clone(),
            minter_account.clone(),
            token_holder_account.clone(),
            associated_token_program.clone(),
            token_program.clone(),
            rent_program.clone(),
            system_program.clone(),
        )?;
        msg!("Associate Token Account Created!!!");

        // Minting Token
        _mint_token_to_account(
            minter_account.clone(),
            token_holder_account.clone(),
            owner_account.clone(),
            token_program.clone(),
        )?;
        msg!("Token Minted!!!");

        _create_metadata_account(
            metadata_account.clone(),
            minter_account.clone(),
            owner_account.clone(),
            metadata_title,
            metadata_symbol,
            metadata_uri,
        )?;
        msg!("Metadata Minted!!!");

        // Creating Master Edition Metadata
        _create_master_edition_account(
            master_edition.clone(),
            minter_account.clone(),
            token_holder_account.clone(),
            metadata_account.clone(),
            owner_account.clone(),
            rent_program.clone(),
        )?;
        msg!("Master Edition Minted!!!");

        msg!("NFT Created!!!");

        Ok(())
    }

    pub fn update_nft_metadata(
        ctx: Context<UpdateNFTMetadata>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        let owner_account = ctx.accounts.authority_account.to_account_info();
        let metadata_account = ctx.accounts.metadata_account.to_account_info();

        _update_metadata_account(
            metadata_account.clone(),
            owner_account.clone(),
            metadata_title,
            metadata_symbol,
            metadata_uri,
        )?;
        msg!("Metadata Updated!!!");

        Ok(())
    }

    // endregion
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner_account: Signer<'info>, // This is you

    #[account(mut)]
    pub minter_account: Signer<'info>, // The mint account that will hold the token.

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateNFTHolder<'info> {
    #[account(mut)]
    pub user_account: Signer<'info>, // This is you

    #[account(mut)]
    pub minter_account: Account<'info, Mint>, // The mint account that will execute token logics.

    /// CHECK: Used by metaplex
    #[account(mut)]
    pub token_holder_account: UncheckedAccount<'info>, // This wallet will hold tokens representing the user
    // #[account(
    //     init_if_needed,
    //     payer = user_account,
    //     associated_token::mint = minter_account,
    //     associated_token::authority = user_account
    // )]
    // pub token_holder_account: Account<'info, TokenAccount>, // This wallet will hold tokens representing the user
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub authority_account: Signer<'info>, // This is you

    #[account(mut)]
    pub minter_account: Account<'info, Mint>,

    #[account(mut)]
    pub token_holder_account: Account<'info, TokenAccount>,

    /// CHECK: Created via metaplex. Should be replaced soon by its appropriate Account<> object
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Created via metaplex. Should be replaced soon by its appropriate Account<> object
    #[account(mut)]
    pub master_edition_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct TransferNFT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // This is you

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub sender: Account<'info, TokenAccount>,

    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CreateNFT<'info> {
    #[account(mut)]
    pub authority_account: Signer<'info>, // This is you

    #[account(mut)]
    pub minter_account: Signer<'info>, // The mint account that will hold the token.

    /// CHECK: Created via metaplex.
    #[account(mut)]
    pub token_holder_account: UncheckedAccount<'info>,

    /// CHECK: Created via metaplex.
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Created via metaplex. Should be replaced soon by its appropriate Account<> object
    #[account(mut)]
    pub master_edition_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct UpdateNFTMetadata<'info> {
    #[account(mut)]
    pub authority_account: Signer<'info>, // This is you

    /// CHECK: Created via metaplex.
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Checked by metaplex.
    pub token_metadata_program: UncheckedAccount<'info>,
}
