extern crate core;

pub mod abi;
mod contants;
mod db;
mod eth;
mod key;
mod pb;
mod rpc;
mod tables;

use std::borrow::Borrow;

use crate::contants::VAULT_ADDRESS;
use crate::ethpb::v2::Block;
use crate::pb::balancer::{
    Pool, PoolToken, PoolTokenBalanceChange, PoolTokenBalanceChanges, PoolTokens, Pools, Token,
};
use crate::tables::Tables;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams::prelude::*;
use substreams::store::{StoreAddBigInt, StoreSetProto};
use substreams::{log, Hex};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_ethereum::pb::eth as ethpb;

#[substreams::handlers::map]
pub fn map_pools_registered(block: Block) -> Result<Pools, Error> {
    use abi::vault::events::PoolRegistered;

    Ok(Pools {
        pools: block
            .events::<PoolRegistered>(&[&VAULT_ADDRESS])
            .filter_map(|(event, _log)| {
                log::info!("pool_id: {}", Hex(&event.pool_id));

                Some(Pool {
                    id: Hex(event.pool_id).to_string(),
                    address: Hex(event.pool_address).to_string(),
                    vault: Hex(VAULT_ADDRESS).to_string(),
                })
            })
            .collect(),
    })
}

#[substreams::handlers::store]
pub fn store_pools(pools: Pools, store: StoreSetProto<Pool>) {
    for pool in pools.pools {
        let pool_id = &pool.id;
        store.set(0, format!("pool:{pool_id}"), &pool);
    }
}

#[substreams::handlers::store]
pub fn store_vault_pool_count(pools: Pools, store: StoreAddBigInt) {
    for _pool in pools.pools {
        store.add(0, format!("vault:poolCount"), &BigInt::one())
    }
}

#[substreams::handlers::map]
pub fn map_pool_tokens_registered(
    block: Block,
    pool_store: StoreGetProto<Pool>,
) -> Result<PoolTokens, Error> {
    use abi::vault::events::TokensRegistered;

    Ok(PoolTokens {
        pool_tokens: block
            .events::<TokensRegistered>(&[&VAULT_ADDRESS])
            .flat_map(|(event, _log)| {
                log::info!("poolId: {}", Hex(&event.pool_id));

                let pool_id = Hex(&event.pool_id).to_string();
                let pool = pool_store.get_last(format!("pool:{pool_id}"));

                let tokens: Vec<Vec<u8>> = event.tokens.clone();

                tokens.into_iter().map(move |token| {
                    let id = format!("{}-{}", Hex(&event.pool_id), Hex(&token));

                    let token_address = Hex(&token).to_string();

                    PoolToken {
                        id: id.clone(),
                        pool: pool.clone(),
                        token: rpc::get_erc20_token(&token_address),
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
        let pool_token_id = &pool_token.id;
        store.set(0, format!("pool_token:{pool_token_id}"), &pool_token);
    }
}

#[substreams::handlers::store]
pub fn store_erc20_tokens(pool_tokens: PoolTokens, store: StoreAddInt64) {
    for pool_token in pool_tokens.pool_tokens {
        let token_id = &pool_token.token.as_ref().unwrap().id;
        store.add(0, format!("token:{token_id}"), 1);
    }
}

#[substreams::handlers::map]
pub fn map_join_exit_balance_changes(
    block: Block,
    pool_token_store: StoreGetProto<PoolToken>,
) -> Result<PoolTokenBalanceChanges, Error> {
    use abi::vault::events::PoolBalanceChanged;

    Ok(PoolTokenBalanceChanges {
        pool_token_balance_changes: block
            .events::<PoolBalanceChanged>(&[&VAULT_ADDRESS])
            .flat_map(|(event, _log)| {
                log::info!("poolId: {}", Hex(&event.pool_id));

                let pool_token_store_ref = pool_token_store.borrow();

                let deltas: Vec<BigInt> = event.deltas.clone();
                let tokens: Vec<Vec<u8>> = event.tokens.clone();
                let protocol_fee_amounts: Vec<BigInt> = event.protocol_fee_amounts.clone();

                tokens.into_iter().enumerate().map(move |(i, token)| {
                    let pool_token_id = format!("{}-{}", Hex(&event.pool_id), Hex(&token));

                    let pool_token =
                        pool_token_store_ref.get_last(format!("pool_token:{pool_token_id}"));

                    let delta_balance = deltas[i].clone() - protocol_fee_amounts[i].clone();

                    PoolTokenBalanceChange {
                        pool_token,
                        delta_balance: delta_balance.to_string(),
                    }
                })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
pub fn map_managed_balance_changes(
    block: Block,
    pool_token_store: StoreGetProto<PoolToken>,
) -> Result<PoolTokenBalanceChanges, Error> {
    use abi::vault::events::PoolBalanceManaged;

    Ok(PoolTokenBalanceChanges {
        pool_token_balance_changes: block
            .events::<PoolBalanceManaged>(&[&VAULT_ADDRESS])
            .filter_map(|(event, _log)| {
                log::info!("pool_id: {}", Hex(&event.pool_id));

                let pool_token_id = format!("{}-{}", Hex(&event.pool_id), Hex(&event.token));
                let pool_token = pool_token_store.get_last(format!("pool_token:{pool_token_id}"));
                let delta_balance = event.cash_delta.clone() + event.managed_delta.clone();

                Some(PoolTokenBalanceChange {
                    pool_token,
                    delta_balance: delta_balance.to_string(),
                })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
pub fn map_swap_balance_changes(
    block: Block,
    pool_token_store: StoreGetProto<PoolToken>,
) -> Result<PoolTokenBalanceChanges, Error> {
    use abi::vault::events::Swap;

    let balance_changes: Vec<PoolTokenBalanceChange> = block
        .events::<Swap>(&[&VAULT_ADDRESS])
        .flat_map(|(event, _log)| {
            log::info!("pool_id: {}", Hex(&event.pool_id));

            let id_in: String = format!("{}-{}", Hex(event.pool_id), Hex(event.token_in));
            let pool_token_in = pool_token_store.get_last(format!("pool_token:{id_in}"));

            let id_out: String = format!("{}-{}", Hex(event.pool_id), Hex(event.token_out));
            let pool_token_out = pool_token_store.get_last(format!("pool_token:{id_out}"));

            vec![
                PoolTokenBalanceChange {
                    pool_token: pool_token_in,
                    delta_balance: event.amount_in.to_string(),
                },
                PoolTokenBalanceChange {
                    pool_token: pool_token_out,
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
        let pool_token_id = &balance_change.pool_token.as_ref().unwrap().id;
        store.add(
            0,
            format!("pool_token_balance:{}", pool_token_id),
            &BigInt::try_from(&balance_change.delta_balance).unwrap(),
        );
    }

    for balance_change in &managed_balance_changes.pool_token_balance_changes {
        let pool_token_id = &balance_change.pool_token.as_ref().unwrap().id;
        store.add(
            0,
            format!("pool_token_balance:{}", pool_token_id),
            &BigInt::try_from(&balance_change.delta_balance).unwrap(),
        );
    }

    for balance_change in &swap_changes.pool_token_balance_changes {
        let pool_token_id = &balance_change.pool_token.as_ref().unwrap().id;
        store.add(
            0,
            format!("pool_token_balance:{}", pool_token_id),
            &BigInt::try_from(&balance_change.delta_balance).unwrap(),
        );
    }
}

#[substreams::handlers::map]
pub fn graph_out(
    clock: Clock,
    pools_registered: Pools,                /* map_pools_registered */
    pool_tokens_registered: PoolTokens,     /* map_pool_tokens_registered */
    tokens_store: StoreGetInt64,            /* store_erc20_tokens */
    pool_count_deltas: Deltas<DeltaBigInt>, /* store_vault_pool_count */
    pool_token_balances_deltas: Deltas<DeltaBigInt>, /* store_pool_token_balances */
) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    if clock.number == 12272146 {
        db::vault_deployed_entity_change(&mut tables);
    }

    db::pool_registered_vault_entity_change(&mut tables, &pool_count_deltas);
    db::pools_registered_pool_entity_changes(&mut tables, &pools_registered);
    db::pool_tokens_registered_pool_token_entity_changes(&mut tables, &pool_tokens_registered);
    db::tokens_created_token_entity_changes(&mut tables, &pool_tokens_registered, tokens_store);
    db::pool_token_balance_entity_change(&mut tables, &pool_token_balances_deltas);

    Ok(tables.to_entity_changes())
}
