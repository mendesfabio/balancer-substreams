extern crate core;

pub mod abi;
mod contants;
mod db;
mod key;
mod pb;
mod tables;

use crate::contants::VAULT_ADDRESS;
use crate::ethpb::v2::Block;
use crate::pb::balancer::{
    Pool, PoolToken, PoolTokenBalanceChange, PoolTokenBalanceChanges, PoolTokens, Pools,
};
use crate::tables::Tables;
use pb::balancer::Vault;
use substreams::errors::Error;
use substreams::prelude::*;
use substreams::store::{StoreAddBigInt, StoreSetProto};
use substreams::{log, Hex};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_ethereum::pb::eth as ethpb;

#[substreams::handlers::map]
pub fn map_vault_deployed(block: Block) -> Result<Vault, Error> {
    match block.number {
        12272146 => Ok(Vault {
            id: Hex(VAULT_ADDRESS).to_string(),
            address: Hex(VAULT_ADDRESS).to_string(),
            pools_count: "0".to_string(),
        }),
        _ => Ok(Vault::default()),
    }
}

#[substreams::handlers::store]
pub fn store_vault(vault: Vault, store: StoreSetProto<Vault>) {
    let vault_id = &vault.id;
    store.set(0, format!("vault:{vault_id}"), &vault);
}

#[substreams::handlers::map]
pub fn map_pools_registered(block: Block) -> Result<Pools, Error> {
    use abi::vault::events::PoolRegistered;

    Ok(Pools {
        pools: block
            .events::<PoolRegistered>(&[&VAULT_ADDRESS])
            .filter_map(|(event, log)| {
                log::info!("pool_id: {}", Hex(&event.pool_id));

                Some(Pool {
                    id: Hex(event.pool_id).to_string(),
                    address: Hex(event.pool_address).to_string(),
                })
            })
            .collect(),
    })
}

#[substreams::handlers::store]
pub fn store_pools(pools: Pools, store: StoreSetProto<Pool>) {
    for pool in pools.pools {
        let pool_id = &pool.id;
        store.set(0, format!("pool_id:{pool_id}"), &pool);
    }
}

#[substreams::handlers::map]
pub fn map_pool_tokens_registered(block: Block) -> Result<PoolTokens, Error> {
    use abi::vault::events::TokensRegistered;

    Ok(PoolTokens {
        pool_tokens: block
            .events::<TokensRegistered>(&[&VAULT_ADDRESS])
            .flat_map(|(event, log)| {
                log::info!("poolId: {}", Hex(&event.pool_id));

                let tokens: Vec<Vec<u8>> = event.tokens.clone();

                tokens.into_iter().map(move |token| {
                    let id = format!("{}-{}", Hex(&event.pool_id), Hex(&token));

                    PoolToken {
                        id: id.clone(),
                        address: Hex(&token).to_string(),
                        pool_id: Hex(&event.pool_id).to_string(),
                        balance: "0".to_string(),
                    }
                })
            })
            .collect(),
    })
}

#[substreams::handlers::store]
pub fn store_pool_tokens(pool_tokens: PoolTokens, store: StoreSetProto<PoolToken>) {
    for pool_token in pool_tokens.pool_tokens {
        let pool_token_address = &pool_token.address;
        store.set(0, format!("pool_token:{pool_token_address}"), &pool_token);
    }
}

#[substreams::handlers::map]
pub fn map_join_exit_balance_changes(block: Block) -> Result<PoolTokenBalanceChanges, Error> {
    use abi::vault::events::PoolBalanceChanged;

    Ok(PoolTokenBalanceChanges {
        pool_token_balance_changes: block
            .events::<PoolBalanceChanged>(&[&VAULT_ADDRESS])
            .flat_map(|(event, log)| {
                log::info!("poolId: {}", Hex(&event.pool_id));

                let deltas: Vec<BigInt> = event.deltas.clone();
                let tokens: Vec<Vec<u8>> = event.tokens.clone();
                let protocol_fee_amounts: Vec<BigInt> = event.protocol_fee_amounts.clone();

                tokens.into_iter().enumerate().map(move |(i, token)| {
                    let pool_token_id = format!("{}-{}", Hex(&event.pool_id), Hex(&token));
                    let delta_balance = deltas[i].clone() - protocol_fee_amounts[i].clone();

                    PoolTokenBalanceChange {
                        pool_token_id,
                        delta_balance: delta_balance.to_string(),
                    }
                })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
pub fn map_managed_balance_changes(block: Block) -> Result<PoolTokenBalanceChanges, Error> {
    use abi::vault::events::PoolBalanceManaged;

    Ok(PoolTokenBalanceChanges {
        pool_token_balance_changes: block
            .events::<PoolBalanceManaged>(&[&VAULT_ADDRESS])
            .filter_map(|(event, log)| {
                log::info!("pool_id: {}", Hex(&event.pool_id));

                let pool_token_id = format!("{}-{}", Hex(&event.pool_id), Hex(&event.token));
                let delta_balance = event.cash_delta.clone() + event.managed_delta.clone();

                Some(PoolTokenBalanceChange {
                    pool_token_id,
                    delta_balance: delta_balance.to_string(),
                })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
pub fn map_swap_balance_changes(block: Block) -> Result<PoolTokenBalanceChanges, Error> {
    use abi::vault::events::Swap;

    let balance_changes: Vec<PoolTokenBalanceChange> = block
        .events::<Swap>(&[&VAULT_ADDRESS])
        .flat_map(|(event, log)| {
            log::info!("pool_id: {}", Hex(&event.pool_id));

            let id_in: String = format!("{}-{}", Hex(event.pool_id), Hex(event.token_in));
            let id_out: String = format!("{}-{}", Hex(event.pool_id), Hex(event.token_out));

            vec![
                PoolTokenBalanceChange {
                    pool_token_id: id_in,
                    delta_balance: event.amount_in.to_string(),
                },
                PoolTokenBalanceChange {
                    pool_token_id: id_out,
                    delta_balance: event.amount_out.neg().to_string(),
                },
            ]
        })
        .collect();

    Ok(PoolTokenBalanceChanges {
        pool_token_balance_changes: balance_changes,
    })
}

#[substreams::handlers::store]
pub fn store_pool_token_balances(
    join_exit_changes: PoolTokenBalanceChanges,
    managed_balance_changes: PoolTokenBalanceChanges,
    swap_changes: PoolTokenBalanceChanges,
    store: StoreAddBigInt,
) {
    for balance_change in &join_exit_changes.pool_token_balance_changes {
        let pool_token_id = &balance_change.pool_token_id;
        store.add(
            0,
            format!("pool_token_balance:{}", pool_token_id),
            &BigInt::try_from(&balance_change.delta_balance).unwrap(),
        );
    }

    for balance_change in &managed_balance_changes.pool_token_balance_changes {
        let pool_token_id = &balance_change.pool_token_id;
        store.add(
            0,
            format!("pool_token_balance:{}", pool_token_id),
            &BigInt::try_from(&balance_change.delta_balance).unwrap(),
        );
    }

    for balance_change in &swap_changes.pool_token_balance_changes {
        let pool_token_id = &balance_change.pool_token_id;
        store.add(
            0,
            format!("pool_token_balance:{}", pool_token_id),
            &BigInt::try_from(&balance_change.delta_balance).unwrap(),
        );
    }
}

#[substreams::handlers::map]
pub fn graph_out(
    pools_registered: Pools,                         /* map_pools_registered */
    pool_tokens_registered: PoolTokens,              /* map_pool_tokens_registered */
    pool_token_balances_deltas: Deltas<DeltaBigInt>, /* store_pool_token_balances */
) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    db::pools_registered_pool_entity_changes(&mut tables, &pools_registered);
    db::pool_tokens_registered_pool_token_entity_changes(&mut tables, &pool_tokens_registered);
    db::pool_token_balance_entity_change(&mut tables, &pool_token_balances_deltas);

    Ok(tables.to_entity_changes())
}
