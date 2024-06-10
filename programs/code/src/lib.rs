use anchor_lang::prelude::*;

#[program]
mod my_smart_contract {
    use super::*;

    #[state]
    pub struct MyState {
        pub mint: Account<Mint>,
        pub token_account: Account<TokenAccount>,
    }

    impl MyState {
        pub fn new(ctx: Context<Initialize>, authority: Pubkey) -> Result<Self> {
            let mint = Mint::new(ctx.accounts.into_account(), &authority)?;
            let token_account = mint.create_account(ctx.accounts.into_account())?;
            Ok(Self {
                mint,
                token_account,
            })
        }

        pub fn mint_token(&mut self, ctx: Context<MintToken>, amount: u64) -> Result<()> {
            self.mint.mint_to(ctx.accounts.into_account(), amount)?;
            Ok(())
        }

        pub fn transfer_token(&mut self, ctx: Context<TransferToken>, amount: u64) -> Result<()> {
            self.mint
                .transfer(ctx.accounts.into_account(), ctx.accounts.to_account, amount)?;
            Ok(())
        }
    }

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(init)]
        pub state: ProgramAccount<'info, MyState>,
        #[account(signer)]
        pub authority: AccountInfo<'info>,
        pub rent: Sysvar<'info, Rent>,
    }

    #[derive(Accounts)]
    pub struct MintToken<'info> {
        #[account(mut)]
        pub state: ProgramAccount<'info, MyState>,
        #[account(mut)]
        pub mint: Account<'info, Mint>,
        #[account(signer)]
        pub authority: AccountInfo<'info>,
    }

    #[derive(Accounts)]
    pub struct TransferToken<'info> {
        #[account(mut)]
        pub state: ProgramAccount<'info, MyState>,
        #[account(mut)]
        pub mint: Account<'info, Mint>,
        #[account(signer)]
        pub from: AccountInfo<'info>,
        #[account(mut)]
        pub to: Account<'info, TokenAccount>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::clock::Epoch;

    #[tokio::test]
    async fn test_initialize() {
        // Test initialization function
        // Ensure state is initialized correctly
        let program_id = Pubkey::new_unique();
        let mut context = TestContext::new(program_id);
        let rent = Rent::default();
        let authority = context.create_account::<AccountInfo>(&[]);

        let mut cpi_ctx = context.create_mock_cpi_context();
        let accounts = Initialize {
            state: context.create_account::<MyState>(&[]),
            authority,
            rent: Rent::default(),
        };
        assert_ok!(my_smart_contract::initialize(
            context.accounts.clone(),
            authority,
            rent
        ));

        // Check state
        let state = MyState::try_from_init_account(&context.accounts.state).unwrap();
        assert_eq!(state.mint, mint.pubkey());
        assert_eq!(state.token_account, token_account.pubkey());
    }

    #[tokio::test]
    async fn test_mint_token() {
        // Test mint_token function
        // Ensure tokens are minted correctly
        let program_id = Pubkey::new_unique();
        let mut context = TestContext::new(program_id);
        let mut cpi_ctx = context.create_mock_cpi_context();
        let authority = context.create_account::<AccountInfo>(&[]);
        let state = context.create_account::<MyState>(&[]);
        let mint = context.create_account::<Mint>(&[]);
        let accounts = MintToken {
            state,
            mint: mint.clone(),
            authority,
        };

        assert_ok!(my_smart_contract::mint_token(
            context.accounts.clone(),
            100 // Assuming 100 tokens are minted
        ));

        // Check if tokens were minted correctly
        let mint_balance = mint.try_borrow_mut_data(&mut cpi_ctx).unwrap();
        assert_eq!(100, *mint_balance);
    }

    #[tokio::test]
    async fn test_transfer_token() {
        // Test transfer_token function
        // Ensure tokens are transferred correctly
        let program_id = Pubkey::new_unique();
        let mut context = TestContext::new(program_id);
        let mut cpi_ctx = context.create_mock_cpi_context();
        let authority = context.create_account::<AccountInfo>(&[]);
        let from = context.create_account::<AccountInfo>(&[]);
        let to = context.create_account::<AccountInfo>(&[]);
        let state = context.create_account::<MyState>(&[]);
        let mint = context.create_account::<Mint>(&[]);
        let token_account = context.create_account::<TokenAccount>(&[]);
        let accounts = TransferToken {
            state,
            mint: mint.clone(),
            from,
            to,
        };

        assert_ok!(my_smart_contract::transfer_token(
            context.accounts.clone(),
            50 // Assuming 50 tokens are transferred
        ));

        // Check if tokens were transferred correctly
        let from_balance = from.try_borrow_mut_data(&mut cpi_ctx).unwrap();
        let to_balance = to.try_borrow_mut_data(&mut cpi_ctx).unwrap();
        assert_eq!(50, *from_balance);
        assert_eq!(50, *to_balance);
    }

   
}
