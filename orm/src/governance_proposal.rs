use diesel::query_builder::AsChangeset;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use shared::proposal::{
    GovernanceProposal, GovernanceProposalKind, GovernanceProposalResult,
    GovernanceProposalStatus,
};

use crate::schema::governance_proposals;

#[derive(Debug, Clone, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::GovernanceKind"]
pub enum GovernanceProposalKindDb {
    PgfSteward,
    PgfFunding,
    Default,
    DefaultWithWasm,
}

impl From<GovernanceProposalKind> for GovernanceProposalKindDb {
    fn from(value: GovernanceProposalKind) -> Self {
        match value {
            GovernanceProposalKind::PgfSteward => Self::PgfSteward,
            GovernanceProposalKind::PgfFunding => Self::PgfFunding,
            GovernanceProposalKind::Default => Self::Default,
            GovernanceProposalKind::DefaultWithWasm => Self::DefaultWithWasm,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::GovernanceResult"]
pub enum GovernanceProposalResultDb {
    Passed,
    Rejected,
    Pending,
    Unknown,
    VotingPeriod,
}

impl From<GovernanceProposalResult> for GovernanceProposalResultDb {
    fn from(value: GovernanceProposalResult) -> Self {
        match value {
            GovernanceProposalResult::Passed => Self::Passed,
            GovernanceProposalResult::Rejected => Self::Rejected,
            GovernanceProposalResult::VotingPeriod => Self::VotingPeriod,
            GovernanceProposalResult::Pending => Self::Pending,
        }
    }
}

#[derive(Serialize, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = governance_proposals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GovernanceProposalDb {
    pub id: i32,
    pub content: String,
    pub data: Option<Vec<u8>>,
    pub kind: GovernanceProposalKindDb,
    pub author: String,
    pub start_epoch: i32,
    pub end_epoch: i32,
    pub activation_epoch: i32,
    pub yay_votes: String,
    pub nay_votes: String,
    pub abstain_votes: String,
    pub result: GovernanceProposalResultDb,
}

#[derive(Serialize, Insertable, Clone)]
#[diesel(table_name = governance_proposals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GovernanceProposalInsertDb {
    pub id: i32,
    pub content: String,
    pub data: Option<Vec<u8>>,
    pub kind: GovernanceProposalKindDb,
    pub author: String,
    pub start_epoch: i32,
    pub end_epoch: i32,
    pub activation_epoch: i32,
}

impl GovernanceProposalInsertDb {
    pub fn from_governance_proposal(proposal: GovernanceProposal) -> Self {
        Self {
            id: proposal.id as i32,
            content: proposal.content,
            data: proposal.data,
            kind: proposal.r#type.into(),
            author: proposal.author.to_string(),
            start_epoch: proposal.voting_start_epoch as i32,
            end_epoch: proposal.voting_end_epoch as i32,
            activation_epoch: proposal.activation_epoch as i32,
        }
    }
}

#[derive(Serialize, AsChangeset, Clone)]
#[diesel(table_name = governance_proposals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GovernanceProposalUpdateStatusDb {
    pub yay_votes: String,
    pub nay_votes: String,
    pub abstain_votes: String,
    pub result: GovernanceProposalResultDb,
}

impl From<GovernanceProposalStatus> for GovernanceProposalUpdateStatusDb {
    fn from(value: GovernanceProposalStatus) -> Self {
        Self {
            yay_votes: value.yay_votes,
            nay_votes: value.nay_votes,
            abstain_votes: value.abstain_votes,
            result: value.result.into(),
        }
    }
}

// impl GovernanceProposalUpdateStatusDb {
//     pub fn from_proposal_status(proposal_status: GovernanceProposalStatus) ->
// Self {         Self {
//             yay_votes: proposal_status.yay_votes,
//             nay_votes: proposal_status.nay_votes,
//             abstain_votes: proposal_status.abstain_votes,
//             result: proposal_status.result.into(),
//         }
//     }
// }
