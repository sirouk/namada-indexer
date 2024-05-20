use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::Json;
use axum_macros::debug_handler;
use axum_trace_id::TraceId;

use crate::dto::governance::{ProposalQueryParams, ProposalVotesQueryparams};
use crate::error::api::ApiError;
use crate::error::governance::GovernanceError;
use crate::response::governance::{Proposal, ProposalVote};
use crate::response::utils::PaginatedResponse;
use crate::state::common::CommonState;

#[debug_handler]
pub async fn get_governance_proposals(
    _trace_id: TraceId<String>,
    _headers: HeaderMap,
    Query(query): Query<ProposalQueryParams>,
    State(state): State<CommonState>,
) -> Result<Json<PaginatedResponse<Vec<Proposal>>>, ApiError> {
    let page = query.pagination.map(|p| p.page).unwrap_or(0);
    let (proposals, total_proposals) = state
        .gov_service
        .find_governance_proposals(query.status, page)
        .await?;

    let response = PaginatedResponse::new(proposals, page, total_proposals);
    Ok(Json(response))
}

#[debug_handler]
pub async fn get_governance_proposal_by_id(
    _trace_id: TraceId<String>,
    _headers: HeaderMap,
    Path(proposal_id): Path<u64>,
    State(state): State<CommonState>,
) -> Result<Json<Proposal>, ApiError> {
    let proposal = state
        .gov_service
        .find_governance_proposal_by_id(proposal_id)
        .await?;

    if let Some(proposal) = proposal {
        Ok(Json(proposal))
    } else {
        Err(GovernanceError::NotFound(proposal_id).into())
    }
}

#[debug_handler]
pub async fn search_governance_proposals_by_pattern(
    _trace_id: TraceId<String>,
    _headers: HeaderMap,
    Path(pattern): Path<String>,
    State(state): State<CommonState>,
    Query(page): Query<Option<u64>>,
) -> Result<Json<PaginatedResponse<Vec<Proposal>>>, ApiError> {
    let page = page.unwrap_or(0);

    let (proposals, total_proposals) = state
        .gov_service
        .search_governance_proposals_by_pattern(pattern, page)
        .await?;

    let response = PaginatedResponse::new(proposals, page, total_proposals);
    Ok(Json(response))
}

#[debug_handler]
pub async fn get_governance_proposal_votes(
    _trace_id: TraceId<String>,
    _headers: HeaderMap,
    Path(proposal_id): Path<u64>,
    Query(query): Query<ProposalVotesQueryparams>,
    State(state): State<CommonState>,
) -> Result<Json<PaginatedResponse<Vec<ProposalVote>>>, ApiError> {
    let page = query.pagination.map(|p| p.page).unwrap_or(0);
    let (proposal_votes, total_votes) = state
        .gov_service
        .find_governance_proposal_votes(proposal_id, page)
        .await?;

    Ok(Json(PaginatedResponse::new(
        proposal_votes,
        page,
        total_votes,
    )))
}

#[debug_handler]
pub async fn get_governance_proposal_votes_by_address(
    _trace_id: TraceId<String>,
    _headers: HeaderMap,
    Path((proposal_id, address)): Path<(u64, String)>,
    State(state): State<CommonState>,
) -> Result<Json<Vec<ProposalVote>>, ApiError> {
    let proposal_votes = state
        .gov_service
        .find_governance_proposal_votes_by_address(proposal_id, address)
        .await?;

    Ok(Json(proposal_votes))
}
