// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, std::fmt::Debug, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "governance_kind"))]
    pub struct GovernanceKind;

    #[derive(diesel::query_builder::QueryId, std::fmt::Debug, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "governance_result"))]
    pub struct GovernanceResult;

    #[derive(diesel::query_builder::QueryId, std::fmt::Debug, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "governance_tally_type"))]
    pub struct GovernanceTallyType;

    #[derive(diesel::query_builder::QueryId, std::fmt::Debug, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "validator_state"))]
    pub struct ValidatorState;

    #[derive(diesel::query_builder::QueryId, std::fmt::Debug, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "vote_kind"))]
    pub struct VoteKind;
}

diesel::table! {
    balances (id) {
        id -> Int4,
        owner -> Varchar,
        token -> Varchar,
        raw_amount -> Varchar,
    }
}

diesel::table! {
    block_crawler_state (id) {
        id -> Int4,
        height -> Int4,
        epoch -> Int4,
    }
}

diesel::table! {
    blocks (id) {
        #[max_length = 32]
        id -> Varchar,
        block_height -> Int4,
        epoch -> Int4,
        #[max_length = 32]
        app_hash -> Varchar,
        validator_id -> Int4,
    }
}

diesel::table! {
    bonds (id) {
        id -> Int4,
        address -> Varchar,
        validator_id -> Int4,
        raw_amount -> Varchar,
    }
}

diesel::table! {
    chain_parameters (epoch) {
        epoch -> Int4,
        unbonding_length -> Int4,
        pipeline_length -> Int4,
        epochs_per_year -> Int4,
        min_num_of_blocks -> Int4,
        min_duration -> Int4,
    }
}

diesel::table! {
    consensus_keys (id) {
        #[max_length = 32]
        id -> Varchar,
        validator_id -> Int4,
        consensus_key -> Int4,
        epoch -> Int4,
    }
}

diesel::table! {
    epoch_crawler_state (id) {
        id -> Int4,
        epoch -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::GovernanceKind;
    use super::sql_types::GovernanceTallyType;
    use super::sql_types::GovernanceResult;

    governance_proposals (id) {
        id -> Int4,
        content -> Varchar,
        data -> Nullable<Varchar>,
        kind -> GovernanceKind,
        tally_type -> GovernanceTallyType,
        author -> Varchar,
        start_epoch -> Int4,
        end_epoch -> Int4,
        activation_epoch -> Int4,
        result -> GovernanceResult,
        yay_votes -> Varchar,
        nay_votes -> Varchar,
        abstain_votes -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::VoteKind;

    governance_votes (id) {
        id -> Int4,
        kind -> VoteKind,
        voter_address -> Varchar,
        proposal_id -> Int4,
    }
}

diesel::table! {
    inner_transactions (id) {
        #[max_length = 32]
        id -> Varchar,
        #[max_length = 32]
        wrapper_id -> Varchar,
        kind -> Varchar,
        data -> Varchar,
        memo -> Nullable<Varchar>,
        exit_code -> Int4,
    }
}

diesel::table! {
    pos_rewards (id) {
        id -> Int4,
        owner -> Varchar,
        validator_id -> Int4,
        raw_amount -> Varchar,
    }
}

diesel::table! {
    revealed_pk (id) {
        id -> Int4,
        address -> Varchar,
        pk -> Varchar,
    }
}

diesel::table! {
    unbonds (id) {
        id -> Int4,
        address -> Varchar,
        validator_id -> Int4,
        raw_amount -> Varchar,
        withdraw_epoch -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ValidatorState;

    validators (id) {
        id -> Int4,
        namada_address -> Varchar,
        voting_power -> Int4,
        max_commission -> Varchar,
        commission -> Varchar,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        discord_handle -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        state -> ValidatorState,
    }
}

diesel::table! {
    wrapper_transactions (id) {
        #[max_length = 32]
        id -> Varchar,
        fee_amount_per_gas_unit_amount -> Varchar,
        fee_amount_per_gas_unit_denomination -> Varchar,
        #[max_length = 32]
        fee_token -> Varchar,
        gas_limit -> Varchar,
        block_height -> Int4,
        atomic -> Bool,
    }
}

diesel::joinable!(blocks -> validators (validator_id));
diesel::joinable!(bonds -> validators (validator_id));
diesel::joinable!(consensus_keys -> validators (validator_id));
diesel::joinable!(governance_votes -> governance_proposals (proposal_id));
diesel::joinable!(inner_transactions -> wrapper_transactions (wrapper_id));
diesel::joinable!(pos_rewards -> validators (validator_id));
diesel::joinable!(unbonds -> validators (validator_id));

diesel::allow_tables_to_appear_in_same_query!(
    balances,
    block_crawler_state,
    blocks,
    bonds,
    chain_parameters,
    consensus_keys,
    epoch_crawler_state,
    governance_proposals,
    governance_votes,
    inner_transactions,
    pos_rewards,
    revealed_pk,
    unbonds,
    validators,
    wrapper_transactions,
);
