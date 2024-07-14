use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

#[account]
pub struct Governor {
    pub base: Pubkey,
    pub bump: u8,
    pub proposal_count: u64,
    pub electorate: Pubkey,
    pub params: crate::GovernanceParameters,
}

#[account]
pub struct Proposal {
    pub governor: Pubkey,
    pub index: u64,
    pub bump: u8,
    pub proposer: Pubkey,
    pub quorum_votes: u64,
    pub for_votes: u64,
    pub against_votes: u64,
    pub abstain_votes: u64,
    pub canceled_at: i64,
    pub created_at: i64,
    pub activated_at: i64,
    pub voting_ends_at: i64,
    pub queued_at: i64,
    pub queued_transaction: Pubkey,
    pub instructions: Vec<crate::ProposalInstruction>,
}

#[account]
pub struct ProposalMeta {
    pub proposal: Pubkey,
    pub title: String,
    pub description_link: String,
}

#[account]
pub struct Vote {
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub bump: u8,
    pub side: u8,
    pub weight: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default, PartialEq)]
pub struct ProposalInstruction {
    pub program_id: Pubkey,
    pub keys: Vec<ProposalAccountMeta>,
    pub data: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default, PartialEq)]
pub struct ProposalAccountMeta {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}
