use std::fmt::Display;

use orm::block_crawler_state::BlockCrawlerStateDb;
use orm::governance_proposal::{
    GovernanceProposalDb, GovernanceProposalKindDb, GovernanceProposalResultDb,
    GovernanceProposalTallyTypeDb,
};
use orm::governance_votes::{GovernanceProposalVoteDb, GovernanceVoteKindDb};
use serde::{Deserialize, Serialize};

use super::utils::{epoch_progress, time_between_epochs};

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
pub enum TallyType {
    TwoThirds,
    OneHalfOverOneThird,
    LessOneHalfOverOneThirdNay,
}

impl Display for TallyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TallyType::TwoThirds => write!(f, "two_thirds"),
            TallyType::OneHalfOverOneThird => {
                write!(f, "one_half_over_one_third")
            }
            TallyType::LessOneHalfOverOneThirdNay => {
                write!(f, "less_one_half_over_one_third_nay")
            }
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
    pub id: String,
    pub content: String,
    pub r#type: ProposalType,
    pub tally_type: TallyType,
    pub data: Option<String>,
    pub author: String,
    pub start_epoch: String,
    pub end_epoch: String,
    pub activation_epoch: String,
    pub start_time: String,
    pub end_time: String,
    pub current_time: String,
    pub status: ProposalStatus,
    pub yay_votes: String,
    pub nay_votes: String,
    pub abstain_votes: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposalVote {
    pub proposal_id: u64,
    pub vote: VoteType,
    pub voter_address: String,
}

impl Proposal {
    pub fn from_proposal_db(
        value: GovernanceProposalDb,
        chain_state: &BlockCrawlerStateDb,
        min_num_of_blocks: i32,
        min_duration: i32,
    ) -> Self {
        // TODO: It would be better to save first block height of current epoch
        // in state and use that here, otherwise if any epoch had
        // different number of blocks than min_num_of_blocks
        // this will be off
        let epoch_progress =
            epoch_progress(chain_state.height, min_num_of_blocks);

        let to_start = time_between_epochs(
            epoch_progress,
            chain_state.epoch,
            value.start_epoch,
            min_duration,
        );

        let to_end = time_between_epochs(
            epoch_progress,
            chain_state.epoch,
            value.end_epoch,
            min_duration,
        );

        let time_now = chain_state.timestamp;
        let start_time = time_now + i64::from(to_start);
        let end_time = time_now + i64::from(to_end);

        Self {
            id: value.id.to_string(),
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
            tally_type: match value.tally_type {
                GovernanceProposalTallyTypeDb::TwoThirds => {
                    TallyType::TwoThirds
                }
                GovernanceProposalTallyTypeDb::OneHalfOverOneThird => {
                    TallyType::OneHalfOverOneThird
                }
                GovernanceProposalTallyTypeDb::LessOneHalfOverOneThirdNay => {
                    TallyType::LessOneHalfOverOneThirdNay
                }
            },
            data: value.data,
            author: value.author,
            start_epoch: value.start_epoch.to_string(),
            end_epoch: value.end_epoch.to_string(),
            activation_epoch: value.activation_epoch.to_string(),

            start_time: start_time.to_string(),
            end_time: end_time.to_string(),
            current_time: time_now.to_string(),

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
            yay_votes: value.yay_votes,
            nay_votes: value.nay_votes,
            abstain_votes: value.abstain_votes,
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
