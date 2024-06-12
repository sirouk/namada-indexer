use anyhow::Context;
use namada_core::storage::Epoch as NamadaEpoch;
use namada_parameters::EpochDuration;
use namada_sdk::address::Address as NamadaAddress;
use namada_sdk::arith::checked;
use namada_sdk::dec::Dec;
use namada_sdk::proof_of_stake::storage_key as pos_storage_key;
use namada_sdk::queries::RPC;
use namada_sdk::rpc::{
    self, get_token_total_supply, get_total_staked_tokens, query_storage_value,
};
use namada_sdk::token::Amount as NamadaSdkAmount;
use shared::block::Epoch;
use shared::parameters::Parameters;
use tendermint_rpc::HttpClient;

pub async fn get_parameters(
    client: &HttpClient,
    epoch: Epoch,
) -> anyhow::Result<Parameters> {
    let pos_parameters = rpc::get_pos_params(client)
        .await
        .with_context(|| "Failed to query pos parameters".to_string())?;

    let epochs_per_year_key =
        namada_parameters::storage::get_epochs_per_year_key();
    let epochs_per_year: u64 =
        rpc::query_storage_value(client, &epochs_per_year_key)
            .await
            .with_context(|| {
                "Failed to query epochs_per_year parameter".to_string()
            })?;

    let epoch_duration_key =
        namada_parameters::storage::get_epoch_duration_storage_key();
    let epoch_duration: EpochDuration =
        rpc::query_storage_value(client, &epoch_duration_key)
            .await
            .with_context(|| {
                "Failed to query epochs_per_year parameter".to_string()
            })?;

    let native_token_address = RPC
        .shell()
        .native_token(client)
        .await
        .context("Failed to query native token")?;
    tracing::info!("Native token address: {:?}", native_token_address);

    let apr = calc_apr(
        client,
        NamadaEpoch::from(epoch as u64),
        &native_token_address,
        epochs_per_year,
    )
    .await?;

    Ok(Parameters {
        epoch,
        unbonding_length: pos_parameters.unbonding_len,
        pipeline_length: pos_parameters.pipeline_len,
        epochs_per_year,
        min_num_of_blocks: epoch_duration.min_num_of_blocks,
        min_duration: epoch_duration.min_duration.0,
        apr,
        native_token_address: native_token_address.to_string(),
    })
}

pub async fn get_current_epoch(client: &HttpClient) -> anyhow::Result<Epoch> {
    let epoch = rpc::query_epoch(client)
        .await
        .context("Failed to query Namada's current epoch")?;

    Ok(epoch.0 as Epoch)
}

async fn calc_apr(
    client: &HttpClient,
    epoch: NamadaEpoch,
    native_token_address: &NamadaAddress,
    epochs_per_year: u64,
) -> anyhow::Result<String> {
    let bonded_tokens = get_total_staked_tokens(client, epoch)
        .await
        .expect("Bonded tokens should be valid");

    let total_supply = get_token_total_supply(client, native_token_address)
        .await
        .expect("Total supply should be written to storage.");

    let pos_inflation_key = pos_storage_key::last_pos_inflation_amount_key();
    let inflation_amount: NamadaSdkAmount =
        query_storage_value(client, &pos_inflation_key)
            .await
            .expect("Inflation amount should be written to storage.");

    // Total supply of native token
    let s_nam = Dec::try_from(total_supply).unwrap();

    // Stored inflation amount per epoch
    let i_pos_last = Dec::try_from(inflation_amount).unwrap();

    // Inflation rate per year
    let i_rate_pos = checked!(i_pos_last / s_nam * epochs_per_year).unwrap();

    // Total bonded tokens
    let l_pos = Dec::try_from(bonded_tokens).unwrap();

    // Annual provision
    let a_prov = i_rate_pos
        .checked_mul(s_nam)
        .expect("Annual provision should be valid");

    // Nominal APR
    let apr_nom = a_prov
        .checked_div(l_pos)
        .expect("Nominal APR should be valid");

    Ok(apr_nom.to_string())
}
