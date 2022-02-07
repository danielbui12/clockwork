use anchor_lang::prelude::*;

pub const SEED_FRAME: &[u8] = b"frame";

#[account]
#[derive(Debug)]
pub struct Frame {
    pub timestamp: u64,
    pub bump: u8,
}
