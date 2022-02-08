use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    daemon_bump: u8,
    fee_bump: u8,
)]
pub struct AdminCreateDaemon<'info> {
    #[account(mut, address = config.admin)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [SEED_AUTHORITY], 
        bump = authority.bump, 
        owner = crate::ID
    )]
    pub authority: Account<'info, Authority>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID,
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        seeds = [
            SEED_DAEMON, 
            authority.key().as_ref()
        ],
        bump = daemon_bump,
        payer = admin,
        space = 8 + size_of::<Daemon>(),
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        init,
        seeds = [
            SEED_FEE, 
            daemon.key().as_ref()
        ],
        bump = fee_bump,
        payer = admin,
        space = 8 + size_of::<Fee>(),
    )]
    pub fee: Account<'info, Fee>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AdminCreateDaemon>, daemon_bump: u8, fee_bump: u8) -> ProgramResult {
    // Get accounts.
    let authority = &ctx.accounts.authority;
    let daemon = &mut ctx.accounts.daemon;
    let fee = &mut ctx.accounts.fee;

    // Initialize daemon account.
    daemon.owner = authority.key();
    daemon.task_count = 0;
    daemon.bump = daemon_bump;

    // Initialize fee account.
    fee.daemon = daemon.key();
    fee.balance = 0;
    fee.bump = fee_bump;

    Ok(())
}
