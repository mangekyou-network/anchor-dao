/*
use anchor_lang::prelude::*;
use solana_program_test::{ProgramTest, BanksClient};
use solana_sdk::{
    transaction::Transaction, 
    signature::{Keypair, Signer},
};
use anchor_lang::ToAccountMetas;
use anchor_spl::token::{self, Token, Mint, TokenAccount};

#[tokio::test]
async fn test_voting_functionality() {
    let program = ProgramTest::new(
        "anchor_governance_program",
        govern::id(),
        processor!(govern::entry), // this entry function needs to be generated by the #[program] macro
    );

    // Initialize test environment
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    // Create test accounts
    let governor = Keypair::new();
    let proposal = Keypair::new();
    let voter = Keypair::new();

    // Prepare CreateProposal instruction
    let create_proposal_ix = govern::instruction::CreateProposal {
        governor: governor.pubkey(),
        proposer: payer.pubkey(),
        instructions: vec![], // mock instructions data
    };

    // Sign and send the transaction
    let mut transaction = Transaction::new_with_payer(
        &[create_proposal_ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &governor], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Prepare a new vote
    let new_vote_ix = govern::instruction::NewVote {
        voter: voter.pubkey(),
        proposal: proposal.pubkey(),
        side: govern::VoteSide::For as u8,  // Example: voting for
        weight: 10,  // Example vote weight
    };

    let mut vote_transaction = Transaction::new_with_payer(
        &[new_vote_ix],
        Some(&payer.pubkey()),
    );
    vote_transaction.sign(&[&payer, &voter], recent_blockhash);
    banks_client.process_transaction(vote_transaction).await.unwrap();

    // Verify the vote
    let proposal_account = banks_client.get_account(proposal.pubkey()).await.unwrap().expect("proposal account not found");
    let proposal_data = govern::state::Proposal::try_from_slice(&proposal_account.data).unwrap();
    
    assert_eq!(proposal_data.for_votes, 10, "The votes should match the vote weight submitted");

    // TODO: Continue with more tests like changing the vote, etc.
}
*/