use std::fmt::Display;

use orm::governance_proposal::{
    GovernanceProposalDb, GovernanceProposalKindDb, GovernanceProposalResultDb,
};
use orm::governance_votes::{GovernanceProposalVoteDb, GovernanceVoteKindDb};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProposalType {
    Default,
    DefaultWithWasm,
    PgfSteward,
    PgfFunding,
}

impl Display for ProposalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalType::Default => write!(f, "default"),
            ProposalType::DefaultWithWasm => write!(f, "default_with_wasm"),
            ProposalType::PgfSteward => write!(f, "pgf_steward"),
            ProposalType::PgfFunding => write!(f, "pgf_funding"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VoteType {
    Yay,
    Nay,
    Abstain,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProposalStatus {
    Pending,
    Rejected,
    Passed,
    Voting,
    Unknown,
}

impl Display for ProposalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalStatus::Pending => write!(f, "Pending"),
            ProposalStatus::Rejected => write!(f, "Rejected"),
            ProposalStatus::Passed => write!(f, "Passed"),
            ProposalStatus::Voting => write!(f, "Voting"),
            ProposalStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Proposal {
    pub id: u64,
    pub content: String,
    pub r#type: ProposalType,
    pub data: Option<String>,
    pub author: String,
    pub start_epoch: u64,
    pub end_epoch: u64,
    pub activation_epoch: u64,
    pub status: ProposalStatus,
    pub yay_votes: u64,
    pub nay_votes: u64,
    pub abstain_votes: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposalVote {
    pub proposal_id: u64,
    pub vote: VoteType,
    pub voter_address: String,
}

impl From<GovernanceProposalDb> for Proposal {
    fn from(value: GovernanceProposalDb) -> Self {
        Self {
            id: value.id as u64,
            content: value.content,
            r#type: match value.kind {
                GovernanceProposalKindDb::PgfSteward => {
                    ProposalType::PgfSteward
                }
                GovernanceProposalKindDb::PgfFunding => {
                    ProposalType::PgfFunding
                }
                GovernanceProposalKindDb::Default => ProposalType::Default,
                GovernanceProposalKindDb::DefaultWithWasm => {
                    ProposalType::DefaultWithWasm
                }
            },
            data: value.data,
            author: value.author,
            start_epoch: value.start_epoch as u64,
            end_epoch: value.end_epoch as u64,
            activation_epoch: value.activation_epoch as u64,
            status: match value.result {
                GovernanceProposalResultDb::Passed => ProposalStatus::Passed,
                GovernanceProposalResultDb::Rejected => {
                    ProposalStatus::Rejected
                }
                GovernanceProposalResultDb::Pending => ProposalStatus::Pending,
                GovernanceProposalResultDb::Unknown => ProposalStatus::Unknown,
                GovernanceProposalResultDb::VotingPeriod => {
                    ProposalStatus::Voting
                }
            },
            yay_votes: value.yay_votes.parse::<u64>().unwrap_or_default(),
            nay_votes: value.nay_votes.parse::<u64>().unwrap_or_default(),
            abstain_votes: value
                .abstain_votes
                .parse::<u64>()
                .unwrap_or_default(),
        }
    }
}

impl From<GovernanceProposalVoteDb> for ProposalVote {
    fn from(value: GovernanceProposalVoteDb) -> Self {
        Self {
            proposal_id: value.proposal_id as u64,
            vote: match value.kind {
                GovernanceVoteKindDb::Nay => VoteType::Nay,
                GovernanceVoteKindDb::Yay => VoteType::Yay,
                GovernanceVoteKindDb::Abstain => VoteType::Abstain,
            },
            voter_address: value.voter_address,
        }
    }
}
