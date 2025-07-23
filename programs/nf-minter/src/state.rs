use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::{create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::{instructions::CreateMetadataAccountV3, types::{Creator, DataV2}}, CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata}, token::{mint_to, Mint, MintTo, Token, TokenAccount}};



#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub payer:Signer<'info>,


    #[account(
        mut,
        seeds = [b"metadata",metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub metadata_account:UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"metadata",metadata_program.key().as_ref(), mint_account.key().as_ref(), b"edition"],
        bump,
        seeds::program = metadata_program.key(),
    )]
    pub edition_account : UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer.key(),
        mint::freeze_authority = payer.key(),
    )]
    pub mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

// user ata,
//mint_account,payer,edition,metadata_account


impl CreateToken<'_> {
    
    pub fn process(
        &mut self,
        nft_name:String,
        nft_symbol:String,
        nft_uri:String,
        bumps:&CreateTokenBumps
    ) -> Result<()> {
        msg!("Minting Token");

        let cpi_context = CpiContext::new(
            self.token_program.to_account_info(), 
            MintTo{
                    mint:self.mint_account.to_account_info(),
                    to:self.associated_token_account.to_account_info(),
                    authority:self.payer.to_account_info()
            });

        mint_to(
            cpi_context
            , 1)?;

        msg!("Creating metadata account");



        let cpi_context_2 = CpiContext::new(
            self.metadata_program.to_account_info(), 
            CreateMetadataAccountsV3{
                metadata:self.metadata_account.to_account_info(),
                mint:self.mint_account.to_account_info(),
                mint_authority:self.payer.to_account_info(),
                payer:self.payer.to_account_info(),
                update_authority:self.payer.to_account_info(),
                system_program:self.system_program.to_account_info(),
                rent:self.rent.to_account_info(),
            },
        );

        let data = DataV2 {
            name:nft_name,
            symbol:nft_symbol,
            uri:nft_uri,
            seller_fee_basis_points:0,
            creators:Some(vec![
                Creator{
                    address:self.payer.key(),
                    verified:true,
                    share:100
            }]),
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(
            cpi_context_2, 
            data, 
            false, 
            true, 
            None
        )?;


        msg!("Creating master edition account");

        let cpi_context_2 = CpiContext::new(
            self.metadata_program.to_account_info(), 
            CreateMasterEditionV3{
                    edition:self.edition_account.to_account_info(),
                    mint:self.mint_account.to_account_info(),
                    update_authority:self.payer.to_account_info(),
                    mint_authority:self.payer.to_account_info(),
                    payer:self.payer.to_account_info(),
                    metadata:self.metadata_account.to_account_info(),
                    token_program:self.token_program.to_account_info(),
                    system_program:self.system_program.to_account_info(),
                    rent:self.rent.to_account_info()
            }
        );

        create_master_edition_v3(
            cpi_context_2, 
            None
        )?;

        msg!("NFT minted successfully.");

        Ok(())
    }
}