mod abi;
mod contants;
mod pb;

use pb::balancer::{Pool, PoolToken, PoolTokens, Pools, Swap, Swaps};
use substreams::errors::Error;
use substreams::store::StoreGet;
use substreams::{log, proto, store, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event};

use crate::contants::VAULT_ADDRESS;

substreams_ethereum::init!();

#[substreams::handlers::map]
pub fn map_pools_registered(block: eth::Block) -> Result<Pools, Error> {
    let mut pools = vec![];

    for log in block.logs() {
        if let Some(event) = abi::vault::events::PoolRegistered::match_and_decode(log) {
            log::info!("Pool ID: {}", Hex(&event.pool_id));

            if log.address() != VAULT_ADDRESS {
                continue;
            };

            pools.push(Pool {
                id: Hex(&event.pool_id).to_string(),
                address: Hex(&event.pool_address).to_string(),
                swap_fee: "0".to_string(),
                pool_type: "".to_string(),
            });
        }
    }

    Ok(Pools { pools })
}

#[substreams::handlers::store]
fn store_pools(pools: Pools, output: store::StoreSet) {
    log::info!("Pools created: {}", pools.pools.len());

    for pool in pools.pools {
        output.set(0, Hex::encode(&pool.id), &proto::encode(&pool).unwrap());
    }
}

#[substreams::handlers::map]
pub fn map_swap_fee_percentage_changed(
    block: eth::Block,
    pools_store: StoreGet,
) -> Result<Pools, Error> {
    let mut pools = vec![];

    for log in block.logs() {
        if let Some(event) = abi::vault::events::PoolRegistered::match_and_decode(log) {
            log::info!("Pool ID: {}", Hex(&event.pool_id));

            if log.address() != VAULT_ADDRESS {
                continue;
            };

            pools.push(Pool {
                id: Hex(&event.pool_id).to_string(),
                address: Hex(&event.pool_address).to_string(),
                swap_fee: "0".to_string(),
                pool_type: "".to_string(),
            });
        }
    }

    Ok(Pools { pools })
}

#[substreams::handlers::map]
pub fn map_tokens_registered(block: eth::Block) -> Result<PoolTokens, Error> {
    let mut pool_tokens = vec![];

    for log in block.logs() {
        if let Some(event) = abi::vault::events::TokensRegistered::match_and_decode(log) {
            log::info!("Pool ID: {}", Hex(&event.pool_id));

            if log.address() != VAULT_ADDRESS {
                continue;
            };

            for pool_token in &event.tokens {
                pool_tokens.push(PoolToken {
                    address: Hex(pool_token).to_string(),
                    pool_id: Hex(&event.pool_id).to_string(),
                });
            }
        }
    }

    Ok(PoolTokens { pool_tokens })
}

#[substreams::handlers::store]
fn store_pool_tokens(pool_tokens: PoolTokens, output: store::StoreSet) {
    log::info!("Tokens registered: {}", pool_tokens.pool_tokens.len());

    for pool_token in pool_tokens.pool_tokens {
        let token_id = [pool_token.pool_id.clone(), pool_token.address.clone()].join("-");

        output.set(
            0,
            Hex::encode(&token_id),
            &proto::encode(&pool_token).unwrap(),
        );
    }
}

#[substreams::handlers::map]
pub fn map_swaps(block: eth::Block) -> Result<Swaps, Error> {
    let mut swaps = vec![];

    for log in block.logs() {
        if let Some(event) = abi::vault::events::Swap::match_and_decode(log) {
            log::info!("Swap found: {}", Hex(&log.receipt.transaction.hash));

            if log.address() != VAULT_ADDRESS {
                continue;
            };

            swaps.push(Swap {
                id: Hex(&log.receipt.transaction.hash).to_string(),
                caller: Hex(&log.receipt.transaction.from).to_string(),
                user_address: Hex(&log.receipt.transaction.from).to_string(),
                pool_id: Hex(&event.pool_id).to_string(),
                token_in: Hex(event.token_in).to_string(),
                token_in_sym: "TKN".to_string(),
                token_amount_in: event.amount_in.to_string(),
                token_out: Hex(event.token_out).to_string(),
                token_out_sym: "TKN".to_string(),
                token_amount_out: event.amount_out.to_string(),
                value_usd: "0".to_string(),
                tx: Hex(&log.receipt.transaction.hash).to_string(),
                timestamp: block
                    .header
                    .as_ref()
                    .unwrap()
                    .timestamp
                    .as_ref()
                    .unwrap()
                    .seconds as u64,
            });
        }
    }

    Ok(Swaps { swaps })
}
