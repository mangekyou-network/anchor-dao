use anchor_lang::prelude::*;
use crate::state::{Governor, Proposal, Vote, ProposalMeta};

#[derive(Accounts)]
pub struct CreateGovernor<'info> {
    #[account(
        init,
        payer = base,
        space = 8 + std::mem::size_of::<Governor>(),
        seeds = [b"governor", base.key().as_ref()],
        bump
    )]
    pub governor: Account<'info, Governor>,
    #[account(mut)]
    pub base: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        payer = proposer,
        space = 8 + std::mem::size_of::<Proposal>(),
        seeds = [b"proposal", governor.key().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    #[account(mut)]
    pub governor: Account<'info, Governor>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActivateProposal<'info> {
    #[account(mut, has_one = governor)]
    pub proposal: Account<'info, Proposal>,
    pub governor: Account<'info, Governor>,
}

#[derive(Accounts)]
pub struct CancelProposal<'info> {
    #[account(mut, has_one = proposer)]
    pub proposal: Account<'info, Proposal>,
    pub proposer: Signer<'info>,
}

#[derive(Accounts)]
pub struct QueueProposal<'info> {
    #[account(mut, has_one = governor)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub governor: Account<'info, Governor>,
    #[account(
        init,
        payer = payer,
        space = 8 + 8,  // Adjust this space according to what data you store
        seeds = [b"transaction", proposal.key().as_ref()],
        bump
    )]
    pub transaction: Account<'info, Proposal>, // Assuming this needs to be a custom struct
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NewVote<'info> {
    #[account(
        init,
        payer = voter,
        space = 8 + std::mem::size_of::<Vote>(),
        seeds = [b"vote", proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, Vote>,
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetVote<'info> {
    #[account(mut, has_one = proposal)]
    pub vote: Account<'info, Vote>,
    pub proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct SetGovernanceParams<'info> {
    #[account(mut)]
    pub governor: Account<'info, Governor>,
}

#[derive(Accounts)]
pub struct CreateProposalMeta<'info> {
    #[account(
        init,
        payer = proposer,
        space = 8 + std::mem::size_of::<ProposalMeta>(),
        seeds = [b"proposal_meta", proposal.key().as_ref()],
        bump
    )]
    pub proposal_meta: Account<'info, ProposalMeta>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub system_program: Program<'info, System>,
}
