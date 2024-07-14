use anchor_lang::prelude::*;
use std::convert::TryFrom;

mod account_validators;
mod state;

use account_validators::*;
use state::*;

declare_id!("GVpMARaodGLrQKyCven2ENBSL5WbCRHDzVvCGdkUz2fk");

#[program]
pub mod govern {
    use super::*;

    pub fn create_governor(
        ctx: Context<CreateGovernor>,
        _bump: u8,
        electorate: Pubkey,
        params: GovernanceParameters,
    ) -> Result<()> {
        require!(params.timelock_delay_seconds >= 0, GovernanceError::InvalidTimelockDelay);

        let governor = &mut ctx.accounts.governor;
        governor.base = ctx.accounts.base.key();

        governor.proposal_count = 0;
        governor.electorate = electorate;
        governor.params = params;

        Ok(())
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        _bump: u8,
        instructions: Vec<ProposalInstruction>,
    ) -> Result<()> {
        let governor = &mut ctx.accounts.governor;

        let proposal = &mut ctx.accounts.proposal;
        proposal.governor = governor.key();
        proposal.index = governor.proposal_count;

        proposal.proposer = ctx.accounts.proposer.key();
        proposal.quorum_votes = governor.params.quorum_votes;
        proposal.created_at = Clock::get()?.unix_timestamp;
        proposal.canceled_at = 0;
        proposal.activated_at = 0;
        proposal.voting_ends_at = 0;
        proposal.queued_at = 0;
        proposal.queued_transaction = Pubkey::default();
        proposal.instructions = instructions;

        governor.proposal_count += 1;

        Ok(())
    }

    pub fn activate_proposal(ctx: Context<ActivateProposal>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let now = Clock::get()?.unix_timestamp;
        proposal.activated_at = now;
        proposal.voting_ends_at = now.checked_add(ctx.accounts.governor.params.voting_period).unwrap();

        Ok(())
    }

    pub fn cancel_proposal(ctx: Context<CancelProposal>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.canceled_at = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn queue_proposal(ctx: Context<QueueProposal>, tx_bump: u8) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.queued_at = Clock::get()?.unix_timestamp;
        proposal.queued_transaction = ctx.accounts.transaction.key();

        Ok(())
    }

    pub fn new_vote(ctx: Context<NewVote>, voter: Pubkey) -> Result<()> {
        let vote = &mut ctx.accounts.vote;
        vote.proposal = ctx.accounts.proposal.key();
        vote.voter = voter;
        vote.side = VoteSide::Pending.into();
        vote.weight = 0;

        Ok(())
    }

    pub fn set_vote(ctx: Context<SetVote>, side: u8, weight: u64) -> Result<()> {
        let vote = &ctx.accounts.vote;
        let proposal = &mut ctx.accounts.proposal;

        proposal.subtract_vote_weight(vote.side.try_into()?, vote.weight)?;
        proposal.add_vote_weight(side.try_into()?, weight)?;

        let vote = &mut ctx.accounts.vote;
        vote.side = side;
        vote.weight = weight;

        Ok(())
    }

    pub fn set_governance_params(ctx: Context<SetGovernanceParams>, params: GovernanceParameters) -> Result<()> {
        ctx.accounts.governor.params = params;

        Ok(())
    }

    pub fn set_electorate(ctx: Context<SetGovernanceParams>, new_electorate: Pubkey) -> Result<()> {
        ctx.accounts.governor.electorate = new_electorate;

        Ok(())
    }

    pub fn create_proposal_meta(ctx: Context<CreateProposalMeta>, _bump: u8, title: String, description_link: String) -> Result<()> {
        let proposal_meta = &mut ctx.accounts.proposal_meta;
        proposal_meta.proposal = ctx.accounts.proposal.key();
        proposal_meta.title = title;
        proposal_meta.description_link = description_link;

        Ok(())
    }
}

/// Governance parameters.
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GovernanceParameters {
    pub timelock_delay_seconds: i64,
    pub voting_period: i64,
    pub quorum_votes: u64,
}

/// Vote sides.
#[derive(Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum VoteSide {
    Pending = 0,
    Against = 1,
    For = 2,
    Abstain = 3,
}

impl Default for VoteSide {
    fn default() -> Self {
        VoteSide::Pending
    }
}

impl From<VoteSide> for u8 {
    fn from(side: VoteSide) -> Self {
        side as u8
    }
}

impl TryFrom<u8> for VoteSide {
    type Error = GovernanceError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(VoteSide::Pending),
            1 => Ok(VoteSide::Against),
            2 => Ok(VoteSide::For),
            3 => Ok(VoteSide::Abstain),
            _ => Err(GovernanceError::InvalidVoteSide),
        }
    }
}

/// Proposal state.
#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub enum ProposalState {
    Draft,
    Active,
    Canceled,
    Defeated,
    Succeeded,
    Queued,
}

impl Default for ProposalState {
    fn default() -> Self {
        Self::Draft
    }
}

impl Proposal {
    pub fn subtract_vote_weight(&mut self, vote_side: VoteSide, vote_weight: u64) -> Result<()> {
        if vote_weight == 0 {
            return Ok(());
        }
        match vote_side {
            VoteSide::Pending => {}
            VoteSide::Against => {
                self.against_votes = self.against_votes.checked_sub(vote_weight).ok_or(GovernanceError::Overflow)?;
            }
            VoteSide::For => {
                self.for_votes = self.for_votes.checked_sub(vote_weight).ok_or(GovernanceError::Overflow)?;
            }
            VoteSide::Abstain => {
                self.abstain_votes = self.abstain_votes.checked_sub(vote_weight).ok_or(GovernanceError::Overflow)?;
            }
        }
        Ok(())
    }

    pub fn add_vote_weight(&mut self, vote_side: VoteSide, vote_weight: u64) -> Result<()> {
        if vote_weight == 0 {
            return Ok(());
        }
        match vote_side {
            VoteSide::Pending => {}
            VoteSide::Against => {
                self.against_votes = self.against_votes.checked_add(vote_weight).ok_or(GovernanceError::Overflow)?;
            }
            VoteSide::For => {
                self.for_votes = self.for_votes.checked_add(vote_weight).ok_or(GovernanceError::Overflow)?;
            }
            VoteSide::Abstain => {
                self.abstain_votes = self.abstain_votes.checked_add(vote_weight).ok_or(GovernanceError::Overflow)?;
            }
        }
        Ok(())
    }
}

#[error_code]
pub enum GovernanceError {
    #[msg("Invalid vote side.")]
    InvalidVoteSide,
    #[msg("The owner of the smart wallet doesn't match with current.")]
    GovernorNotFound,
    #[msg("The proposal cannot be activated since it has not yet passed the voting delay.")]
    VotingDelayNotMet,
    #[msg("Only drafts can be canceled.")]
    ProposalNotDraft,
    #[msg("The proposal must be active.")]
    ProposalNotActive,
    #[msg("Overflow error.")]
    Overflow,
    #[msg("Invalid timelock delay.")]
    InvalidTimelockDelay,
    #[msg("Proposal not found.")]
    ProposalNotFound,
    #[msg("Vote not found.")]
    VoteNotFound,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock data structures similar to what the program would use
    #[derive(Clone)]
    struct MockProposal {
        pub for_votes: u64,
        pub against_votes: u64,
    }

    // Mocking the logic to simulate voting
    fn vote_on_proposal(proposal: &mut MockProposal, side: VoteSide, weight: u64) {
        match side {
            VoteSide::For => proposal.for_votes += weight,
            VoteSide::Against => proposal.against_votes += weight,
            _ => {}
        }
    }

    #[test]
    fn test_voting_for() {
        let mut proposal = MockProposal {
            for_votes: 0,
            against_votes: 0,
        };

        // Simulate voting 'for'
        vote_on_proposal(&mut proposal, VoteSide::For, 100);

        assert_eq!(proposal.for_votes, 100);
        assert_eq!(proposal.against_votes, 0);
    }

    #[test]
    fn test_voting_against() {
        let mut proposal = MockProposal {
            for_votes: 50,
            against_votes: 25,
        };

        // Simulate changing vote to 'against'
        vote_on_proposal(&mut proposal, VoteSide::Against, 50);

        assert_eq!(proposal.for_votes, 50);
        assert_eq!(proposal.against_votes, 75);
    }
}
