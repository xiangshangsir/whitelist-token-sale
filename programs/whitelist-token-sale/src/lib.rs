use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod whitelist_token_sale {
    use super::*;

    /// 初始化代币销售配置
    pub fn initialize_sale(
        ctx: Context<InitializeSale>,
        start_time: i64,
        end_time: i64,
        max_allocation_per_user: u64,
        token_price: u64, // 每个代币的价格（以 USDC 计，带 decimals）
        total_supply: u64,
    ) -> Result<()> {
        require!(start_time < end_time, ErrorCode::InvalidTimeRange);
        require!(start_time > Clock::get()?.unix_timestamp, ErrorCode::StartTimeMustBeFuture);
        require!(max_allocation_per_user > 0, ErrorCode::InvalidAllocation);
        require!(token_price > 0, ErrorCode::InvalidPrice);

        let sale_config = &mut ctx.accounts.sale_config;
        sale_config.admin = ctx.accounts.admin.key();
        sale_config.token_mint = ctx.accounts.token_mint.key();
        sale_config.usdc_mint = ctx.accounts.usdc_mint.key();
        sale_config.start_time = start_time;
        sale_config.end_time = end_time;
        sale_config.max_allocation_per_user = max_allocation_per_user;
        sale_config.token_price = token_price;
        sale_config.total_supply = total_supply;
        sale_config.tokens_sold = 0;
        sale_config.usdc_collected = 0;
        sale_config.is_active = true;

        emit!(SaleInitialized {
            admin: ctx.accounts.admin.key(),
            start_time,
            end_time,
            max_allocation_per_user,
            token_price,
            total_supply,
        });

        Ok(())
    }

    /// 添加用户到白名单
    pub fn add_to_whitelist(
        ctx: Context<ManageWhitelist>,
        allocation: u64,
    ) -> Result<()> {
        require!(
            ctx.accounts.admin.key() == ctx.accounts.sale_config.admin,
            ErrorCode::Unauthorized
        );

        let whitelist_account = &mut ctx.accounts.whitelist_account;
        whitelist_account.user = ctx.accounts.user.key();
        whitelist_account.sale_config = ctx.accounts.sale_config.key();
        whitelist_account.allocation = allocation;
        whitelist_account.claimed = 0;
        whitelist_account.is_active = true;

        emit!(UserAddedToWhitelist {
            user: ctx.accounts.user.key(),
            allocation,
        });

        Ok(())
    }

    /// 从白名单移除用户
    pub fn remove_from_whitelist(ctx: Context<ManageWhitelist>) -> Result<()> {
        require!(
            ctx.accounts.admin.key() == ctx.accounts.sale_config.admin,
            ErrorCode::Unauthorized
        );

        let whitelist_account = &mut ctx.accounts.whitelist_account;
        whitelist_account.is_active = false;

        emit!(UserRemovedFromWhitelist {
            user: ctx.accounts.user.key(),
        });

        Ok(())
    }

    /// 用户 Claim 代币
    pub fn claim_tokens(
        ctx: Context<ClaimTokens>,
        amount: u64,
    ) -> Result<()> {
        let sale_config = &ctx.accounts.sale_config;
        let whitelist_account = &mut ctx.accounts.whitelist_account;

        // 验证销售正在进行中
        let clock = Clock::get()?;
        require!(sale_config.is_active, ErrorCode::SaleNotActive);
        require!(
            clock.unix_timestamp >= sale_config.start_time,
            ErrorCode::SaleNotStarted
        );
        require!(
            clock.unix_timestamp <= sale_config.end_time,
            ErrorCode::SaleEnded
        );

        // 验证用户在白名单中
        require!(whitelist_account.is_active, ErrorCode::NotOnWhitelist);
        require!(
            whitelist_account.user == ctx.accounts.user.key(),
            ErrorCode::WhitelistUserMismatch
        );

        // 验证 Claim 数量
        let available_amount = whitelist_account.allocation.saturating_sub(whitelist_account.claimed);
        require!(amount > 0, ErrorCode::InvalidClaimAmount);
        require!(amount <= available_amount, ErrorCode::ExceedsAllocation);

        // 验证销售还有足够代币
        let remaining_tokens = sale_config.total_supply.saturating_sub(sale_config.tokens_sold);
        require!(amount <= remaining_tokens, ErrorCode::InsufficientTokens);

        // 计算需要支付的 USDC
        let usdc_amount = amount.checked_mul(sale_config.token_price).unwrap();

        // 验证用户 USDC 余额足够
        require!(
            ctx.accounts.user_usdc_account.amount >= usdc_amount,
            ErrorCode::InsufficientUsdcBalance
        );

        // 转账 USDC 从用户到销售账户
        let transfer_usdc_ix = Transfer {
            from: ctx.accounts.user_usdc_account.to_account_info(),
            to: ctx.accounts.sale_usdc_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let transfer_usdc_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_usdc_ix,
        );

        token::transfer(transfer_usdc_ctx, usdc_amount)?;

        // 转账代币从销售账户到用户
        let transfer_token_ix = Transfer {
            from: ctx.accounts.sale_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.sale_config.to_account_info(),
        };

        // 使用 PDA 签名
        let seeds = &[
            b"sale_config",
            ctx.accounts.sale_config.token_mint.as_ref(),
            &[ctx.accounts.sale_config.bump],
        ];
        let signer = &[&seeds[..]];

        let transfer_token_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_token_ix,
            signer,
        );

        token::transfer(transfer_token_ctx, amount)?;

        // 更新状态
        whitelist_account.claimed = whitelist_account.claimed.checked_add(amount).unwrap();
        let sale_config_mut = &mut ctx.accounts.sale_config;
        sale_config_mut.tokens_sold = sale_config_mut.tokens_sold.checked_add(amount).unwrap();
        sale_config_mut.usdc_collected = sale_config_mut.usdc_collected.checked_add(usdc_amount).unwrap();

        emit!(TokensClaimed {
            user: ctx.accounts.user.key(),
            amount,
            usdc_paid: usdc_amount,
            remaining_allocation: whitelist_account.allocation - whitelist_account.claimed,
        });

        Ok(())
    }

    /// 管理员提取未售出的代币
    pub fn withdraw_unsold_tokens(ctx: Context<WithdrawUnsold>) -> Result<()> {
        require!(
            ctx.accounts.admin.key() == ctx.accounts.sale_config.admin,
            ErrorCode::Unauthorized
        );

        let sale_config = &ctx.accounts.sale_config;
        let clock = Clock::get()?;

        // 只能在销售结束后提取
        require!(
            clock.unix_timestamp > sale_config.end_time,
            ErrorCode::SaleNotEnded
        );

        let unsold_tokens = sale_config.total_supply.saturating_sub(sale_config.tokens_sold);
        require!(unsold_tokens > 0, ErrorCode::NoUnsoldTokens);

        // 转账未售出代币回管理员
        let seeds = &[
            b"sale_config",
            sale_config.token_mint.as_ref(),
            &[sale_config.bump],
        ];
        let signer = &[&seeds[..]];

        let transfer_ix = Transfer {
            from: ctx.accounts.sale_token_account.to_account_info(),
            to: ctx.accounts.admin_token_account.to_account_info(),
            authority: ctx.accounts.sale_config.to_account_info(),
        };

        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_ix,
            signer,
        );

        token::transfer(transfer_ctx, unsold_tokens)?;

        emit!(UnsoldTokensWithdrawn {
            admin: ctx.accounts.admin.key(),
            amount: unsold_tokens,
        });

        Ok(())
    }

    /// 管理员提取已收集的 USDC
    pub fn withdraw_usdc(ctx: Context<WithdrawUsdc>, amount: u64) -> Result<()> {
        require!(
            ctx.accounts.admin.key() == ctx.accounts.sale_config.admin,
            ErrorCode::Unauthorized
        );

        let sale_config = &ctx.accounts.sale_config;
        require!(
            sale_config.usdc_collected >= amount,
            ErrorCode::InsufficientUsdcInSale
        );

        let seeds = &[
            b"sale_config",
            sale_config.token_mint.as_ref(),
            &[sale_config.bump],
        ];
        let signer = &[&seeds[..]];

        let transfer_ix = Transfer {
            from: ctx.accounts.sale_usdc_account.to_account_info(),
            to: ctx.accounts.admin_usdc_account.to_account_info(),
            authority: ctx.accounts.sale_config.to_account_info(),
        };

        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_ix,
            signer,
        );

        token::transfer(transfer_ctx, amount)?;

        sale_config.usdc_collected = sale_config.usdc_collected.saturating_sub(amount);

        emit!(UsdcWithdrawn {
            admin: ctx.accounts.admin.key(),
            amount,
        });

        Ok(())
    }
}

// ============ 账户结构 ============

#[account]
pub struct SaleConfig {
    pub admin: Pubkey,
    pub token_mint: Pubkey,
    pub usdc_mint: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub max_allocation_per_user: u64,
    pub token_price: u64,
    pub total_supply: u64,
    pub tokens_sold: u64,
    pub usdc_collected: u64,
    pub is_active: bool,
    pub bump: u8,
}

#[account]
pub struct WhitelistAccount {
    pub user: Pubkey,
    pub sale_config: Pubkey,
    pub allocation: u64,
    pub claimed: u64,
    pub is_active: bool,
}

// ============ Context 结构 ============

#[derive(Accounts)]
pub struct InitializeSale<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + SaleConfig::INIT_SPACE,
        seeds = [b"sale_config", token_mint.key().as_ref()],
        bump
    )]
    pub sale_config: Account<'info, SaleConfig>,

    pub token_mint: Account<'info, Mint>,
    pub usdc_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        token::mint = token_mint,
        token::authority = sale_config,
        seeds = [b"sale_token_account", token_mint.key().as_ref()],
        bump
    )]
    pub sale_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        token::mint = usdc_mint,
        token::authority = sale_config,
        seeds = [b"sale_usdc_account", usdc_mint.key().as_ref()],
        bump
    )]
    pub sale_usdc_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ManageWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        has_one = admin @ ErrorCode::Unauthorized
    )]
    pub sale_config: Account<'info, SaleConfig>,

    /// CHECK: 这是要添加到白名单的用户
    pub user: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + WhitelistAccount::INIT_SPACE,
        seeds = [b"whitelist", sale_config.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub whitelist_account: Account<'info, WhitelistAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"sale_config", token_mint.key().as_ref()],
        bump = sale_config.bump
    )]
    pub sale_config: Account<'info, SaleConfig>,

    #[account(
        mut,
        seeds = [b"whitelist", sale_config.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub whitelist_account: Account<'info, WhitelistAccount>,

    pub token_mint: Account<'info, Mint>,
    pub usdc_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"sale_token_account", token_mint.key().as_ref()],
        bump
    )]
    pub sale_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"sale_usdc_account", usdc_mint.key().as_ref()],
        bump
    )]
    pub sale_usdc_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = usdc_mint,
        token::authority = user
    )]
    pub user_usdc_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawUnsold<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        has_one = admin @ ErrorCode::Unauthorized,
        seeds = [b"sale_config", token_mint.key().as_ref()],
        bump = sale_config.bump
    )]
    pub sale_config: Account<'info, SaleConfig>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"sale_token_account", token_mint.key().as_ref()],
        bump
    )]
    pub sale_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = admin
    )]
    pub admin_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawUsdc<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        has_one = admin @ ErrorCode::Unauthorized,
        seeds = [b"sale_config", usdc_mint.key().as_ref()],
        bump = sale_config.bump
    )]
    pub sale_config: Account<'info, SaleConfig>,

    pub usdc_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"sale_usdc_account", usdc_mint.key().as_ref()],
        bump
    )]
    pub sale_usdc_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = usdc_mint,
        token::authority = admin
    )]
    pub admin_usdc_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

// ============ 错误码 ============

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid time range")]
    InvalidTimeRange,
    #[msg("Start time must be in the future")]
    StartTimeMustBeFuture,
    #[msg("Invalid allocation")]
    InvalidAllocation,
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Sale not active")]
    SaleNotActive,
    #[msg("Sale not started")]
    SaleNotStarted,
    #[msg("Sale ended")]
    SaleEnded,
    #[msg("Not on whitelist")]
    NotOnWhitelist,
    #[msg("Whitelist user mismatch")]
    WhitelistUserMismatch,
    #[msg("Invalid claim amount")]
    InvalidClaimAmount,
    #[msg("Exceeds allocation")]
    ExceedsAllocation,
    #[msg("Insufficient tokens")]
    InsufficientTokens,
    #[msg("Insufficient USDC balance")]
    InsufficientUsdcBalance,
    #[msg("Sale not ended")]
    SaleNotEnded,
    #[msg("No unsold tokens")]
    NoUnsoldTokens,
    #[msg("Insufficient USDC in sale")]
    InsufficientUsdcInSale,
}

// ============ 事件 ============

#[event]
pub struct SaleInitialized {
    pub admin: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub max_allocation_per_user: u64,
    pub token_price: u64,
    pub total_supply: u64,
}

#[event]
pub struct UserAddedToWhitelist {
    pub user: Pubkey,
    pub allocation: u64,
}

#[event]
pub struct UserRemovedFromWhitelist {
    pub user: Pubkey,
}

#[event]
pub struct TokensClaimed {
    pub user: Pubkey,
    pub amount: u64,
    pub usdc_paid: u64,
    pub remaining_allocation: u64,
}

#[event]
pub struct UnsoldTokensWithdrawn {
    pub admin: Pubkey,
    pub amount: u64,
}

#[event]
pub struct UsdcWithdrawn {
    pub admin: Pubkey,
    pub amount: u64,
}
