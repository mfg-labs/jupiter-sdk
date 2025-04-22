use anchor_lang::prelude::*;

declare_id!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

// Accounts
#[derive(Accounts)]
pub struct Route<'info> {
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub destination_token_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub destination_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub platform_fee_account: Option<AccountInfo<'info>>,
}

#[derive(Accounts)]
pub struct RouteWithTokenLedger<'info> {
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub destination_token_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub destination_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub platform_fee_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub token_ledger: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SharedAccountsRoute<'info> {
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program_authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub destination_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub source_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub destination_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub platform_fee_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub token_2022_program: Option<AccountInfo<'info>>,
}

#[derive(Accounts)]
pub struct SharedAccountsRouteWithTokenLedger<'info> {
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program_authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub destination_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub source_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub destination_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub platform_fee_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub token_2022_program: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub token_ledger: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ExactOutRoute<'info> {
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub destination_token_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub source_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub destination_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub platform_fee_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub token_2022_program: Option<AccountInfo<'info>>,
}

#[derive(Accounts)]
pub struct SharedAccountsExactOutRoute<'info> {
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program_authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub destination_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub source_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub destination_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub platform_fee_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub token_2022_program: Option<AccountInfo<'info>>,
}

#[derive(Accounts)]
pub struct SetTokenLedger<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub token_ledger: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateOpenOrders<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub token_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateProgramOpenOrders<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub wallet: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ClaimToken<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub wallet: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub destination_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateTokenLedger<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub token_ledger: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

// CPI
#[cfg(all(target_os = "solana", feature="cpi"))]
pub mod cpi {
    #![allow(unused)]
    use anchor_lang::Discriminator;
    use super::*;

    pub fn route<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Route<'info>>,
        route_plan: Vec<RoutePlanStep>,
        in_amount: u64,
        quoted_out_amount: u64,
        slippage_bps: u16,
        platform_fee_bps: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Route { route_plan, in_amount, quoted_out_amount, slippage_bps, platform_fee_bps };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Route::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn route_with_token_ledger<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, RouteWithTokenLedger<'info>>,
        route_plan: Vec<RoutePlanStep>,
        quoted_out_amount: u64,
        slippage_bps: u16,
        platform_fee_bps: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::RouteWithTokenLedger { route_plan, quoted_out_amount, slippage_bps, platform_fee_bps };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::RouteWithTokenLedger::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn shared_accounts_route<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SharedAccountsRoute<'info>>,
        id: u8,
        route_plan: Vec<RoutePlanStep>,
        in_amount: u64,
        quoted_out_amount: u64,
        slippage_bps: u16,
        platform_fee_bps: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SharedAccountsRoute { id, route_plan, in_amount, quoted_out_amount, slippage_bps, platform_fee_bps };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SharedAccountsRoute::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn shared_accounts_route_with_token_ledger<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SharedAccountsRouteWithTokenLedger<'info>>,
        id: u8,
        route_plan: Vec<RoutePlanStep>,
        quoted_out_amount: u64,
        slippage_bps: u16,
        platform_fee_bps: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SharedAccountsRouteWithTokenLedger { id, route_plan, quoted_out_amount, slippage_bps, platform_fee_bps };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SharedAccountsRouteWithTokenLedger::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn exact_out_route<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ExactOutRoute<'info>>,
        route_plan: Vec<RoutePlanStep>,
        out_amount: u64,
        quoted_in_amount: u64,
        slippage_bps: u16,
        platform_fee_bps: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ExactOutRoute { route_plan, out_amount, quoted_in_amount, slippage_bps, platform_fee_bps };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ExactOutRoute::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn shared_accounts_exact_out_route<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SharedAccountsExactOutRoute<'info>>,
        id: u8,
        route_plan: Vec<RoutePlanStep>,
        out_amount: u64,
        quoted_in_amount: u64,
        slippage_bps: u16,
        platform_fee_bps: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SharedAccountsExactOutRoute { id, route_plan, out_amount, quoted_in_amount, slippage_bps, platform_fee_bps };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SharedAccountsExactOutRoute::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn set_token_ledger<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SetTokenLedger<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SetTokenLedger {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SetTokenLedger::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn create_open_orders<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CreateOpenOrders<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CreateOpenOrders {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CreateOpenOrders::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn create_token_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CreateTokenAccount<'info>>,
        bump: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CreateTokenAccount { bump };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CreateTokenAccount::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn create_program_open_orders<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CreateProgramOpenOrders<'info>>,
        id: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CreateProgramOpenOrders { id };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CreateProgramOpenOrders::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn claim<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Claim<'info>>,
        id: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Claim { id };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Claim::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn claim_token<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ClaimToken<'info>>,
        id: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ClaimToken { id };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ClaimToken::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn create_token_ledger<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CreateTokenLedger<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CreateTokenLedger {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CreateTokenLedger::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }  
}

// RPC
#[cfg(all(not(target_os = "solana"), feature="cpi"))]
pub mod rpc {
    #![allow(unused)]
    use anchor_lang::prelude::*;
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct Route {
            pub token_program: Pubkey,
            pub user_transfer_authority: Pubkey,
            pub user_source_token_account: Pubkey,
            pub user_destination_token_account: Pubkey,
            pub destination_token_account: Pubkey,
            pub destination_mint: Pubkey,
            pub platform_fee_account: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for Route {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.token_program,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.user_transfer_authority,
                true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.user_source_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.user_destination_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.destination_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.destination_mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.platform_fee_account,
                false,
            ));
                account_metas
            }
        }
    
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct RouteWithTokenLedger {
            pub token_program: Pubkey,
            pub user_transfer_authority: Pubkey,
            pub user_source_token_account: Pubkey,
            pub user_destination_token_account: Pubkey,
            pub destination_token_account: Pubkey,
            pub destination_mint: Pubkey,
            pub platform_fee_account: Pubkey,
            pub token_ledger: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for RouteWithTokenLedger {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.token_program,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.user_transfer_authority,
                true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.user_source_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.user_destination_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.destination_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.destination_mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.platform_fee_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.token_ledger,
                false,
            ));
                account_metas
            }
        }
    
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct SharedAccountsRoute {
            pub token_program: Pubkey,
            pub program_authority: Pubkey,
            pub user_transfer_authority: Pubkey,
            pub source_token_account: Pubkey,
            pub program_source_token_account: Pubkey,
            pub program_destination_token_account: Pubkey,
            pub destination_token_account: Pubkey,
            pub source_mint: Pubkey,
            pub destination_mint: Pubkey,
            pub platform_fee_account: Pubkey,
            pub token_2022_program: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for SharedAccountsRoute {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.token_program,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.program_authority,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.user_transfer_authority,
                true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.source_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.program_source_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.program_destination_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.destination_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.source_mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.destination_mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.platform_fee_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.token_2022_program,
                false,
            ));
                account_metas
            }
        }
    
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct SharedAccountsRouteWithTokenLedger {
            pub token_program: Pubkey,
            pub program_authority: Pubkey,
            pub user_transfer_authority: Pubkey,
            pub source_token_account: Pubkey,
            pub program_source_token_account: Pubkey,
            pub program_destination_token_account: Pubkey,
            pub destination_token_account: Pubkey,
            pub source_mint: Pubkey,
            pub destination_mint: Pubkey,
            pub platform_fee_account: Pubkey,
            pub token_2022_program: Pubkey,
            pub token_ledger: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for SharedAccountsRouteWithTokenLedger {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.token_program,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.program_authority,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.user_transfer_authority,
                true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.source_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.program_source_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.program_destination_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.destination_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.source_mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.destination_mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.platform_fee_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.token_2022_program,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.token_ledger,
                false,
            ));
                account_metas
            }
        }
    
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct ExactOutRoute {
            pub token_program: Pubkey,
            pub user_transfer_authority: Pubkey,
            pub user_source_token_account: Pubkey,
            pub user_destination_token_account: Pubkey,
            pub destination_token_account: Pubkey,
            pub source_mint: Pubkey,
            pub destination_mint: Pubkey,
            pub platform_fee_account: Pubkey,
            pub token_2022_program: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for ExactOutRoute {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.token_program,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.user_transfer_authority,
                true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.user_source_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.user_destination_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.destination_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.source_mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.destination_mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.platform_fee_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.token_2022_program,
                false,
            ));
                account_metas
            }
        }
    
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct SharedAccountsExactOutRoute {
            pub token_program: Pubkey,
            pub program_authority: Pubkey,
            pub user_transfer_authority: Pubkey,
            pub source_token_account: Pubkey,
            pub program_source_token_account: Pubkey,
            pub program_destination_token_account: Pubkey,
            pub destination_token_account: Pubkey,
            pub source_mint: Pubkey,
            pub destination_mint: Pubkey,
            pub platform_fee_account: Pubkey,
            pub token_2022_program: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for SharedAccountsExactOutRoute {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.token_program,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.program_authority,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.user_transfer_authority,
                true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.source_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.program_source_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.program_destination_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.destination_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.source_mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.destination_mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.platform_fee_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.token_2022_program,
                false,
            ));
                account_metas
            }
        }
    
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct SetTokenLedger {
            pub token_ledger: Pubkey,
            pub token_account: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for SetTokenLedger {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.token_ledger,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.token_account,
                false,
            ));
                account_metas
            }
        }
    
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct CreateOpenOrders {
            pub open_orders: Pubkey,
            pub payer: Pubkey,
            pub dex_program: Pubkey,
            pub system_program: Pubkey,
            pub rent: Pubkey,
            pub market: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for CreateOpenOrders {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.open_orders,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.payer,
                true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.dex_program,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.system_program,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.rent,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.market,
                false,
            ));
                account_metas
            }
        }
    
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct CreateTokenAccount {
            pub token_account: Pubkey,
            pub user: Pubkey,
            pub mint: Pubkey,
            pub token_program: Pubkey,
            pub system_program: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for CreateTokenAccount {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.user,
                true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.token_program,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.system_program,
                false,
            ));
                account_metas
            }
        }
    
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct CreateProgramOpenOrders {
            pub open_orders: Pubkey,
            pub payer: Pubkey,
            pub program_authority: Pubkey,
            pub dex_program: Pubkey,
            pub system_program: Pubkey,
            pub rent: Pubkey,
            pub market: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for CreateProgramOpenOrders {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.open_orders,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.payer,
                true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.program_authority,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.dex_program,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.system_program,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.rent,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.market,
                false,
            ));
                account_metas
            }
        }
    
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct Claim {
            pub wallet: Pubkey,
            pub program_authority: Pubkey,
            pub system_program: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for Claim {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.wallet,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.program_authority,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.system_program,
                false,
            ));
                account_metas
            }
        }
    
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct ClaimToken {
            pub payer: Pubkey,
            pub wallet: Pubkey,
            pub program_authority: Pubkey,
            pub program_token_account: Pubkey,
            pub destination_token_account: Pubkey,
            pub mint: Pubkey,
            pub associated_token_token_program: Pubkey,
            pub associated_token_program: Pubkey,
            pub system_program: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for ClaimToken {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.payer,
                true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.wallet,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.program_authority,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.program_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.destination_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.associated_token_token_program,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.associated_token_program,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.system_program,
                false,
            ));
                account_metas
            }
        }
    
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct CreateTokenLedger {
            pub token_ledger: Pubkey,
            pub payer: Pubkey,
            pub system_program: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for CreateTokenLedger {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.token_ledger,
                true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.payer,
                true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.system_program,
                false,
            ));
                account_metas
            }
        }
}

// I11n
#[cfg(all(target_os = "solana", feature="i11n"))]
pub mod i11n {
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use anchor_lang::Discriminator;
    use super::{instructions::*, ID};

    // Route
    #[derive(TryFromInstruction, serde::Serialize)]
    pub struct RouteI11n<'info> {
        pub accounts: RouteAccountMetas<'info>,
        pub args: Route,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // RouteWithTokenLedger
    #[derive(TryFromInstruction, serde::Serialize)]
    pub struct RouteWithTokenLedgerI11n<'info> {
        pub accounts: RouteWithTokenLedgerAccountMetas<'info>,
        pub args: RouteWithTokenLedger,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SharedAccountsRoute
    #[derive(TryFromInstruction, serde::Serialize)]
    pub struct SharedAccountsRouteI11n<'info> {
        pub accounts: SharedAccountsRouteAccountMetas<'info>,
        pub args: SharedAccountsRoute,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SharedAccountsRouteWithTokenLedger
    #[derive(TryFromInstruction, serde::Serialize)]
    pub struct SharedAccountsRouteWithTokenLedgerI11n<'info> {
        pub accounts: SharedAccountsRouteWithTokenLedgerAccountMetas<'info>,
        pub args: SharedAccountsRouteWithTokenLedger,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ExactOutRoute
    #[derive(TryFromInstruction, serde::Serialize)]
    pub struct ExactOutRouteI11n<'info> {
        pub accounts: ExactOutRouteAccountMetas<'info>,
        pub args: ExactOutRoute,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SharedAccountsExactOutRoute
    #[derive(TryFromInstruction, serde::Serialize)]
    pub struct SharedAccountsExactOutRouteI11n<'info> {
        pub accounts: SharedAccountsExactOutRouteAccountMetas<'info>,
        pub args: SharedAccountsExactOutRoute,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SetTokenLedger
    #[derive(TryFromInstruction, serde::Serialize)]
    pub struct SetTokenLedgerI11n<'info> {
        pub accounts: SetTokenLedgerAccountMetas<'info>,
        pub args: SetTokenLedger,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CreateOpenOrders
    #[derive(TryFromInstruction, serde::Serialize)]
    pub struct CreateOpenOrdersI11n<'info> {
        pub accounts: CreateOpenOrdersAccountMetas<'info>,
        pub args: CreateOpenOrders,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CreateTokenAccount
    #[derive(TryFromInstruction, serde::Serialize)]
    pub struct CreateTokenAccountI11n<'info> {
        pub accounts: CreateTokenAccountAccountMetas<'info>,
        pub args: CreateTokenAccount,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CreateProgramOpenOrders
    #[derive(TryFromInstruction, serde::Serialize)]
    pub struct CreateProgramOpenOrdersI11n<'info> {
        pub accounts: CreateProgramOpenOrdersAccountMetas<'info>,
        pub args: CreateProgramOpenOrders,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // Claim
    #[derive(TryFromInstruction, serde::Serialize)]
    pub struct ClaimI11n<'info> {
        pub accounts: ClaimAccountMetas<'info>,
        pub args: Claim,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ClaimToken
    #[derive(TryFromInstruction, serde::Serialize)]
    pub struct ClaimTokenI11n<'info> {
        pub accounts: ClaimTokenAccountMetas<'info>,
        pub args: ClaimToken,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CreateTokenLedger
    #[derive(TryFromInstruction, serde::Serialize)]
    pub struct CreateTokenLedgerI11n<'info> {
        pub accounts: CreateTokenLedgerAccountMetas<'info>,
        pub args: CreateTokenLedger,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    //Accounts
    #[derive(TryFromAccountMetas, serde::Serialize)]
    pub struct RouteAccountMetas<'info> {
        pub token_program: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub user_source_token_account: &'info AccountMeta,
        pub user_destination_token_account: &'info AccountMeta,
        pub destination_token_account: Option<&'info AccountMeta>,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
    }

    #[derive(TryFromAccountMetas, serde::Serialize)]
    pub struct RouteWithTokenLedgerAccountMetas<'info> {
        pub token_program: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub user_source_token_account: &'info AccountMeta,
        pub user_destination_token_account: &'info AccountMeta,
        pub destination_token_account: Option<&'info AccountMeta>,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub token_ledger: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas, serde::Serialize)]
    pub struct SharedAccountsRouteAccountMetas<'info> {
        pub token_program: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub source_token_account: &'info AccountMeta,
        pub program_source_token_account: &'info AccountMeta,
        pub program_destination_token_account: &'info AccountMeta,
        pub destination_token_account: &'info AccountMeta,
        pub source_mint: &'info AccountMeta,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub token_2022_program: Option<&'info AccountMeta>,
    }

    #[derive(TryFromAccountMetas, serde::Serialize)]
    pub struct SharedAccountsRouteWithTokenLedgerAccountMetas<'info> {
        pub token_program: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub source_token_account: &'info AccountMeta,
        pub program_source_token_account: &'info AccountMeta,
        pub program_destination_token_account: &'info AccountMeta,
        pub destination_token_account: &'info AccountMeta,
        pub source_mint: &'info AccountMeta,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub token_2022_program: Option<&'info AccountMeta>,
        pub token_ledger: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas, serde::Serialize)]
    pub struct ExactOutRouteAccountMetas<'info> {
        pub token_program: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub user_source_token_account: &'info AccountMeta,
        pub user_destination_token_account: &'info AccountMeta,
        pub destination_token_account: Option<&'info AccountMeta>,
        pub source_mint: &'info AccountMeta,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub token_2022_program: Option<&'info AccountMeta>,
    }

    #[derive(TryFromAccountMetas, serde::Serialize)]
    pub struct SharedAccountsExactOutRouteAccountMetas<'info> {
        pub token_program: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub source_token_account: &'info AccountMeta,
        pub program_source_token_account: &'info AccountMeta,
        pub program_destination_token_account: &'info AccountMeta,
        pub destination_token_account: &'info AccountMeta,
        pub source_mint: &'info AccountMeta,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub token_2022_program: Option<&'info AccountMeta>,
    }

    #[derive(TryFromAccountMetas, serde::Serialize)]
    pub struct SetTokenLedgerAccountMetas<'info> {
        pub token_ledger: &'info AccountMeta,
        pub token_account: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas, serde::Serialize)]
    pub struct CreateOpenOrdersAccountMetas<'info> {
        pub open_orders: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub market: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas, serde::Serialize)]
    pub struct CreateTokenAccountAccountMetas<'info> {
        pub token_account: &'info AccountMeta,
        pub user: &'info AccountMeta,
        pub mint: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas, serde::Serialize)]
    pub struct CreateProgramOpenOrdersAccountMetas<'info> {
        pub open_orders: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub market: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas, serde::Serialize)]
    pub struct ClaimAccountMetas<'info> {
        pub wallet: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas, serde::Serialize)]
    pub struct ClaimTokenAccountMetas<'info> {
        pub payer: &'info AccountMeta,
        pub wallet: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub program_token_account: &'info AccountMeta,
        pub destination_token_account: &'info AccountMeta,
        pub mint: &'info AccountMeta,
        pub associated_token_token_program: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas, serde::Serialize)]
    pub struct CreateTokenLedgerAccountMetas<'info> {
        pub token_ledger: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    //Helper

    // Jupiter
    #[derive(serde::Serialize)]
    pub enum JupiterI11n<'info> {
        
        #[serde(rename = "route")]
        Route(RouteI11n<'info>),

        #[serde(rename = "routeWithTokenLedger")]
        RouteWithTokenLedger(RouteWithTokenLedgerI11n<'info>),

        #[serde(rename = "sharedAccountsRoute")]
        SharedAccountsRoute(SharedAccountsRouteI11n<'info>),

        #[serde(rename = "sharedAccountsRouteWithTokenLedger")]
        SharedAccountsRouteWithTokenLedger(SharedAccountsRouteWithTokenLedgerI11n<'info>),

        #[serde(rename = "exactOutRoute")]
        ExactOutRoute(ExactOutRouteI11n<'info>),

        #[serde(rename = "sharedAccountsExactOutRoute")]
        SharedAccountsExactOutRoute(SharedAccountsExactOutRouteI11n<'info>),

        #[serde(rename = "setTokenLedger")]
        SetTokenLedger(SetTokenLedgerI11n<'info>),

        #[serde(rename = "createOpenOrders")]
        CreateOpenOrders(CreateOpenOrdersI11n<'info>),

        #[serde(rename = "createTokenAccount")]
        CreateTokenAccount(CreateTokenAccountI11n<'info>),

        #[serde(rename = "createProgramOpenOrders")]
        CreateProgramOpenOrders(CreateProgramOpenOrdersI11n<'info>),

        #[serde(rename = "claim")]
        Claim(ClaimI11n<'info>),

        #[serde(rename = "claimToken")]
        ClaimToken(ClaimTokenI11n<'info>),

        #[serde(rename = "createTokenLedger")]
        CreateTokenLedger(CreateTokenLedgerI11n<'info>),

        #[serde(rename = "unknown")]
        Unknown(String),
    }




    //helper
    pub fn introspect(ix: &anchor_lang::solana_program::instruction::Instruction) -> (String, JupiterI11n) {
        let disc_data: &[u8] = &ix.data.as_slice()[..8];
        let discriminator: &[u8] = disc_data;

        
            if Route::discriminator().eq(discriminator) {
                return ("route".to_string(), JupiterI11n::Route(RouteI11n::try_from(ix).expect("Invalid instruction of Route")))
            }
        
			
            if RouteWithTokenLedger::discriminator().eq(discriminator) {
                return ("routeWithTokenLedger".to_string(), JupiterI11n::RouteWithTokenLedger(RouteWithTokenLedgerI11n::try_from(ix).expect("Invalid instruction of RouteWithTokenLedger")))
            }
        
			
            if SharedAccountsRoute::discriminator().eq(discriminator) {
                return ("sharedAccountsRoute".to_string(), JupiterI11n::SharedAccountsRoute(SharedAccountsRouteI11n::try_from(ix).expect("Invalid instruction of SharedAccountsRoute")))
            }
        
			
            if SharedAccountsRouteWithTokenLedger::discriminator().eq(discriminator) {
                return ("sharedAccountsRouteWithTokenLedger".to_string(), JupiterI11n::SharedAccountsRouteWithTokenLedger(SharedAccountsRouteWithTokenLedgerI11n::try_from(ix).expect("Invalid instruction of SharedAccountsRouteWithTokenLedger")))
            }
        
			
            if ExactOutRoute::discriminator().eq(discriminator) {
                return ("exactOutRoute".to_string(), JupiterI11n::ExactOutRoute(ExactOutRouteI11n::try_from(ix).expect("Invalid instruction of ExactOutRoute")))
            }
        
			
            if SharedAccountsExactOutRoute::discriminator().eq(discriminator) {
                return ("sharedAccountsExactOutRoute".to_string(), JupiterI11n::SharedAccountsExactOutRoute(SharedAccountsExactOutRouteI11n::try_from(ix).expect("Invalid instruction of SharedAccountsExactOutRoute")))
            }
        
			
            if SetTokenLedger::discriminator().eq(discriminator) {
                return ("setTokenLedger".to_string(), JupiterI11n::SetTokenLedger(SetTokenLedgerI11n::try_from(ix).expect("Invalid instruction of SetTokenLedger")))
            }
        
			
            if CreateOpenOrders::discriminator().eq(discriminator) {
                return ("createOpenOrders".to_string(), JupiterI11n::CreateOpenOrders(CreateOpenOrdersI11n::try_from(ix).expect("Invalid instruction of CreateOpenOrders")))
            }
        
			
            if CreateTokenAccount::discriminator().eq(discriminator) {
                return ("createTokenAccount".to_string(), JupiterI11n::CreateTokenAccount(CreateTokenAccountI11n::try_from(ix).expect("Invalid instruction of CreateTokenAccount")))
            }
        
			
            if CreateProgramOpenOrders::discriminator().eq(discriminator) {
                return ("createProgramOpenOrders".to_string(), JupiterI11n::CreateProgramOpenOrders(CreateProgramOpenOrdersI11n::try_from(ix).expect("Invalid instruction of CreateProgramOpenOrders")))
            }
        
			
            if Claim::discriminator().eq(discriminator) {
                return ("claim".to_string(), JupiterI11n::Claim(ClaimI11n::try_from(ix).expect("Invalid instruction of Claim")))
            }
        
			
            if ClaimToken::discriminator().eq(discriminator) {
                return ("claimToken".to_string(), JupiterI11n::ClaimToken(ClaimTokenI11n::try_from(ix).expect("Invalid instruction of ClaimToken")))
            }
        
			
            if CreateTokenLedger::discriminator().eq(discriminator) {
                return ("createTokenLedger".to_string(), JupiterI11n::CreateTokenLedger(CreateTokenLedgerI11n::try_from(ix).expect("Invalid instruction of CreateTokenLedger")))
            }
        

        ("unknown".to_string(), JupiterI11n::Unknown("Failed to resolve command".to_string()))
    }
    
}

// Instructions
pub mod instructions {
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use anchor_lang::Discriminator;
    use super::*;

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
    pub struct Route {
        pub route_plan: Vec<RoutePlanStep>,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    
    impl anchor_lang::InstructionData for Route {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
    pub struct RouteWithTokenLedger {
        pub route_plan: Vec<RoutePlanStep>,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    
    impl anchor_lang::InstructionData for RouteWithTokenLedger {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
    pub struct SharedAccountsRoute {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    
    impl anchor_lang::InstructionData for SharedAccountsRoute {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
    pub struct SharedAccountsRouteWithTokenLedger {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    
    impl anchor_lang::InstructionData for SharedAccountsRouteWithTokenLedger {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
    pub struct ExactOutRoute {
        pub route_plan: Vec<RoutePlanStep>,
        pub out_amount: u64,
        pub quoted_in_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    
    impl anchor_lang::InstructionData for ExactOutRoute {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
    pub struct SharedAccountsExactOutRoute {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub out_amount: u64,
        pub quoted_in_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    
    impl anchor_lang::InstructionData for SharedAccountsExactOutRoute {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
    pub struct SetTokenLedger {

    }
    
    impl anchor_lang::InstructionData for SetTokenLedger {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
    pub struct CreateOpenOrders {

    }
    
    impl anchor_lang::InstructionData for CreateOpenOrders {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
    pub struct CreateTokenAccount {
        pub bump: u8,
    }
    
    impl anchor_lang::InstructionData for CreateTokenAccount {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
    pub struct CreateProgramOpenOrders {
        pub id: u8,
    }
    
    impl anchor_lang::InstructionData for CreateProgramOpenOrders {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
    pub struct Claim {
        pub id: u8,
    }
    
    impl anchor_lang::InstructionData for Claim {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
    pub struct ClaimToken {
        pub id: u8,
    }
    
    impl anchor_lang::InstructionData for ClaimToken {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
    pub struct CreateTokenLedger {

    }
    
    impl anchor_lang::InstructionData for CreateTokenLedger {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
            
}

// Events
#[cfg(feature="events")]
pub mod events {
    use super::*;
    use anchor_i11n::AnchorDiscriminator;
    use anchor_lang::Discriminator;
    use anchor_lang::__private::base64;
    use anchor_lang::__private::base64::Engine;

    fn get_event_discriminator(event: &str) -> [u8; 8] {
        let preimage = format!("{}:{}", "event", event);
        let mut sighash = [0u8; 8];
        sighash.copy_from_slice(
            &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8],
        );
        sighash
    }

    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorDeserialize, serde::Serialize)]
    pub struct SwapEvent {
                pub amm: Pubkey,
                pub input_mint: Pubkey,
                pub input_amount: u64,
                pub output_mint: Pubkey,
                pub output_amount: u64
    }
    
    impl Discriminator for SwapEvent {
        const DISCRIMINATOR: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    
        fn discriminator() -> [u8; 8] {
            get_event_discriminator("SwapEvent")
        }
    }
    
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorDeserialize, serde::Serialize)]
    pub struct FeeEvent {
                pub account: Pubkey,
                pub mint: Pubkey,
                pub amount: u64
    }
    
    impl Discriminator for FeeEvent {
        const DISCRIMINATOR: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    
        fn discriminator() -> [u8; 8] {
            get_event_discriminator("FeeEvent")
        }
    }
}

// Accounts
pub mod accounts {
    #![allow(unused)]
    use super::*;

    #[account]
    pub struct TokenLedger {
        pub token_account: Pubkey,
        pub amount: u64
    }  
}
        
// Defined types
#[cfg_attr(not(target_os="solana"), derive(Debug))]
#[derive(Clone, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
pub struct AmountWithSlippage {
    pub amount: u64,
    pub slippage_bps: u16,
}

#[cfg_attr(not(target_os="solana"), derive(Debug))]
#[derive(Clone, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
pub struct RoutePlanStep {
    pub swap: Swap,
    pub percent: u8,
    pub input_index: u8,
    pub output_index: u8,
}

#[cfg_attr(not(target_os="solana"), derive(Debug))]
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq, serde::Serialize)]
pub enum PlatformFeeType {
    SourceMint,
    DestinationMint,
    Zero
}

#[cfg_attr(not(target_os="solana"), derive(Debug))]
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq, serde::Serialize)]
pub enum Side {
    Bid,
    Ask
}

#[cfg_attr(not(target_os="solana"), derive(Debug))]
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq, serde::Serialize)]
pub enum Swap {
    Saber,
    SaberAddDecimalsDeposit,
    SaberAddDecimalsWithdraw,
    TokenSwap,
    Sencha,
    Step,
    Cropper,
    Raydium,
    Crema,
    Lifinity,
    Mercurial,
    Cykura,
    Serum,
    MarinadeDeposit,
    MarinadeUnstake,
    Aldrin,
    AldrinV2,
    Whirlpool,
    Invariant,
    Meteora,
    GooseFx,
    DeltaFi,
    Balansol,
    MarcoPolo,
    Dradex,
    LifinityV2,
    RaydiumClmm,
    Openbook,
    Phoenix,
    Symmetry,
    TokenSwapV2,
    HeliumTreasuryManagementRedeemV0,
    StakeDexStakeWrappedSol,
    StakeDexSwapViaStake,
    GooseFxv2,
    Perps,
    PerpsAddLiquidity,
    PerpsRemoveLiquidity,
    MeteoraDlmm,
    OpenBookV2,
    RaydiumClmmV2,
    StakeDexPrefundWithdrawStakeAndDepositStake,
    Clone,
    SanctumS,
    SanctumSAddLiquidity,
    SanctumSRemoveLiquidity,
    RaydiumCp,
    WhirlpoolSwapV2,
    OneIntro,
    PumpdotfunWrappedBuy,
    PumpdotfunWrappedSell,
    PerpsV2,
    PerpsV2AddLiquidity,
    PerpsV2RemoveLiquidity,
    MoonshotWrappedBuy,
    MoonshotWrappedSell
}

#[cfg_attr(not(target_os="solana"), derive(Debug))]
#[derive(Clone, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
pub struct RemainingAccountsSlice {
    pub accounts_type: AccountsType,
    pub length: u8,
}

#[cfg_attr(not(target_os="solana"), derive(Debug))]
#[derive(Clone, AnchorSerialize, AnchorDeserialize, serde::Serialize)]
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}

#[cfg_attr(not(target_os="solana"), derive(Debug))]
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq, serde::Serialize)]
pub enum AccountsType {
    TransferHookA,
    TransferHookB
}