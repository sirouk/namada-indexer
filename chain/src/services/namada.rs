use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{anyhow, Context};
use namada_core::storage::{
    BlockHeight as NamadaSdkBlockHeight, Epoch as NamadaSdkEpoch,
};
use namada_sdk::address::Address as NamadaSdkAddress;
use namada_sdk::queries::RPC;
use namada_sdk::rpc::bonds_and_unbonds;
use namada_sdk::{rpc, token};
use shared::balance::{Amount, Balance, Balances};
use shared::block::{BlockHeight, Epoch};
use shared::bond::{Bond, BondAddresses, Bonds};
use shared::bond::{Bond, BondAddresses, Bonds};
use shared::id::Id;
use shared::unbond::{Unbond, UnbondAddresses, Unbonds};
use shared::utils::BalanceChange;
use tendermint_rpc::HttpClient;

use super::utils::query_storage_prefix;

pub async fn is_block_committed(
    client: &HttpClient,
    block_height: BlockHeight,
) -> anyhow::Result<bool> {
    let block_height = to_block_height(block_height);
    let last_block = RPC
        .shell()
        .last_block(client)
        .await
        .context("Failed to query Namada's last committed block")?;
    Ok(last_block
        .map(|b| block_height <= b.height)
        .unwrap_or(false))
}

pub async fn get_native_token(client: &HttpClient) -> anyhow::Result<Id> {
    let native_token = RPC
        .shell()
        .native_token(client)
        .await
        .context("Failed to query native token")?;
    Ok(Id::from(native_token))
}

pub async fn get_epoch_at_block_height(
    client: &HttpClient,
    block_height: BlockHeight,
) -> anyhow::Result<Epoch> {
    let block_height = to_block_height(block_height);
    let epoch = rpc::query_epoch_at_height(client, block_height)
        .await
        .with_context(|| {
            format!("Failed to query Namada's epoch at height {block_height}")
        })?
        .ok_or_else(|| {
            anyhow!("No Namada epoch found for height {block_height}")
        })?;
    Ok(epoch.0 as Epoch)
}

// TODO: remove unwraps
pub async fn query_balance(
    client: &HttpClient,
    balance_changes: &HashSet<BalanceChange>,
) -> anyhow::Result<Balances> {
    let mut res: Balances = vec![];

    for balance_change in balance_changes {
        let owner =
            NamadaSdkAddress::from_str(&balance_change.address.to_string())
                .context("Failed to parse owner address")?;
        let token =
            NamadaSdkAddress::from_str(&balance_change.token.to_string())
                .context("Failed to parse token address")?;

        let amount = rpc::get_token_balance(client, &token, &owner)
            .await
            .unwrap_or_default();

        res.push(Balance {
            owner: Id::from(owner),
            token: Id::from(token),
            amount: Amount::from(amount),
        });
    }

    anyhow::Ok(res)
}

pub async fn query_all_balances(
    client: &HttpClient,
) -> anyhow::Result<Balances> {
    let token_addr = RPC
        .shell()
        .native_token(client)
        .await
        .context("Failed to query native token")?;

    let balance_prefix = namada_token::storage_key::balance_prefix(&token_addr);

    let balances =
        query_storage_prefix::<token::Amount>(client, &balance_prefix)
            .await
            //TODO: unwrap
            .unwrap();

    let mut all_balances: Balances = vec![];

    if let Some(balances) = balances {
        for (key, balance) in balances {
            let (t, o, b) =
                match namada_token::storage_key::is_any_token_balance_key(&key)
                {
                    Some([tok, owner]) => (tok.clone(), owner.clone(), balance),
                    None => continue,
                };

            all_balances.push(Balance {
                owner: Id::from(o),
                token: Id::from(t),
                amount: Amount::from(b),
            });
        }
    }

    anyhow::Ok(all_balances)
}

pub async fn query_last_block_height(
    client: &HttpClient,
) -> anyhow::Result<BlockHeight> {
    let last_block = RPC
        .shell()
        .last_block(client)
        .await
        .context("Failed to query Namada's last committed block")?;

    Ok(last_block
        .map(|b| b.height.0 as BlockHeight)
        .unwrap_or_default())
}

pub async fn query_all_bonds_and_unbonds(
    client: &HttpClient,
    epoch: Epoch,
) -> anyhow::Result<(Bonds, Unbonds)> {
    let asd = bonds_and_unbonds(client, &None, &None)
        .await
        .context("Failed to query all bonds and unbonds")?;
    let mut bonds = vec![];
    let mut unbonds = vec![];

    for (id, detials) in asd {
        for bond_details in detials.bonds {
            bonds.push(Bond {
                source: Id::from(id.source.clone()),
                target: Id::from(id.validator.clone()),
                amount: Amount::from(bond_details.amount),
            });
        }

        for unbond_details in detials.unbonds {
            unbonds.push(Unbond {
                source: Id::from(id.source.clone()),
                target: Id::from(id.validator.clone()),
                amount: Amount::from(unbond_details.amount),
                withdraw_at: unbond_details.withdraw.0 as Epoch,
            });
        }
    }

    let bonds = Bonds {
        epoch,
        values: bonds,
    };

    let unbonds = Unbonds {
        epoch,
        values: unbonds,
    };

    Ok((bonds, unbonds))
}

pub async fn query_next_governance_id(
    client: &HttpClient,
    block_height: BlockHeight,
) -> anyhow::Result<u64> {
    let proposal_counter_key =
        namada_sdk::governance::storage::keys::get_counter_key();
    let block_height = to_block_height(block_height);

    let query_result = RPC
        .shell()
        .storage_value(
            client,
            None,
            Some(block_height),
            false,
            &proposal_counter_key,
        )
        .await
        .context("Failed to get the next proposal id")?;
    namada_sdk::borsh::BorshDeserialize::try_from_slice(&query_result.data)
        .context("Failed to deserialize proposal id")
}

pub async fn query_bonds(
    client: &HttpClient,
    addresses: Vec<BondAddresses>,
    epoch: Epoch,
) -> anyhow::Result<Bonds> {
    let mut bonds = vec![];

    for BondAddresses { source, target } in addresses {
        let source = NamadaSdkAddress::from_str(&source.to_string())
            .expect("Failed to parse source address");
        let target = NamadaSdkAddress::from_str(&target.to_string())
            .expect("Failed to parse target address");

        let amount = RPC
            .vp()
            .pos()
            .bond_with_slashing(
                client,
                &source,
                &target,
                // TODO: + 2 is hardcoded pipeline len
                &Some(to_epoch(epoch + 2)),
            )
            .await
            .context("Failed to query bond amount")?;

        bonds.push(Bond {
            source: Id::from(source),
            target: Id::from(target),
            amount: Amount::from(amount),
        });
    }

    anyhow::Ok(Bonds {
        epoch,
        values: bonds,
    })
}

pub async fn query_unbonds(
    client: &HttpClient,
    addresses: Vec<UnbondAddresses>,
    epoch: Epoch,
) -> anyhow::Result<Unbonds> {
    let mut unbonds = vec![];

    for UnbondAddresses { source, validator } in addresses {
        let source = NamadaSdkAddress::from_str(&source.to_string())
            .context("Failed to parse source address")?;
        let validator = NamadaSdkAddress::from_str(&validator.to_string())
            .context("Failed to parse validator address")?;

        let res = rpc::query_unbond_with_slashing(client, &source, &validator)
            .await
            .context("Failed to query unbond amount")?;

        tracing::info!("unbonds {:?}", res);

        let ((_, withdraw_epoch), amount) =
            res.last().context("Unbonds are empty")?;

        unbonds.push(Unbond {
            source: Id::from(source),
            target: Id::from(validator),
            amount: Amount::from(*amount),
            withdraw_at: withdraw_epoch.0 as Epoch,
        });
    }

    anyhow::Ok(Unbonds {
        epoch,
        values: unbonds,
    })
}

pub async fn get_current_epoch(client: &HttpClient) -> anyhow::Result<Epoch> {
    let epoch = rpc::query_epoch(client)
        .await
        .context("Failed to query Namada's current epoch")?;

    Ok(epoch.0 as Epoch)
}

fn to_block_height(block_height: u32) -> NamadaSdkBlockHeight {
    NamadaSdkBlockHeight::from(block_height as u64)
}

fn to_epoch(epoch: u32) -> NamadaSdkEpoch {
    NamadaSdkEpoch::from(epoch as u64)
}
