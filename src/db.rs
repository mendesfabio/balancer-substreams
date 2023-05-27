use substreams::scalar::BigInt;
use substreams::store::{key_first_segment_in, DeltaBigInt, Deltas, StoreGet, StoreGetInt64};

use crate::key;
use crate::pb::balancer::{Pool, PoolToken, PoolTokens, Pools, Token, Vault};
use crate::tables::Tables;

pub fn vault_deployed_entity_change(tables: &mut Tables, vault: &Vault) {
    create_vault_entity(tables, vault);
}

fn create_vault_entity(tables: &mut Tables, vault: &Vault) {
    let bigint0 = BigInt::zero();

    tables
        .create_row("Vault", format!("0x{}", &vault.id))
        .set("address", format!("0x{}", &vault.address))
        .set("poolsCount", bigint0);
}

pub fn pools_registered_pool_entity_changes(tables: &mut Tables, pools: &Pools) {
    for pool in &pools.pools {
        create_pool_entity(tables, pool);
    }
}

fn create_pool_entity(tables: &mut Tables, pool: &Pool) {
    tables
        .create_row("Pool", format!("0x{}", &pool.id))
        .set("address", format!("0x{}", &pool.address));
}

pub fn pool_tokens_registered_pool_token_entity_changes(
    tables: &mut Tables,
    pool_tokens: &PoolTokens,
) {
    for pool_token in &pool_tokens.pool_tokens {
        create_pool_token_entity(tables, pool_token);
    }
}

fn create_pool_token_entity(tables: &mut Tables, pool_token: &PoolToken) {
    let bigint0 = BigInt::zero();

    tables
        .create_row("PoolToken", format!("0x{}", &pool_token.id))
        .set(
            "pool",
            format!("0x{}", &pool_token.pool.as_ref().unwrap().id),
        )
        .set(
            "token",
            format!("0x{}", &pool_token.token.as_ref().unwrap().address),
        )
        .set("balance", bigint0);
}

pub fn tokens_created_token_entity_changes(
    tables: &mut Tables,
    pool_tokens: &PoolTokens,
    tokens_store: StoreGetInt64,
) {
    for pool_token in &pool_tokens.pool_tokens {
        let token_id = &pool_token.token.as_ref().unwrap().id;

        match tokens_store.get_at(0, format!("token:{token_id}")) {
            Some(value) => {
                if value.eq(&1) {
                    create_token_entity(tables, pool_token.token.as_ref().unwrap());
                }
            }
            None => {
                panic!(
                    "pool contains token that doesn't exist {}",
                    token_id.as_str()
                )
            }
        }
    }
}

fn create_token_entity(tables: &mut Tables, token: &Token) {
    let token_id = &token.id;

    tables
        .create_row("Token", format!("0x{token_id}"))
        .set("address", &token.address)
        .set("symbol", &token.symbol)
        .set("decimals", token.decimals)
        .set("name", &token.name);
}

pub fn pool_token_balance_entity_change(
    tables: &mut Tables,
    pool_token_balances_store_deltas: &Deltas<DeltaBigInt>,
) {
    for delta in pool_token_balances_store_deltas
        .deltas
        .iter()
        .filter(key_first_segment_in("pool_token_balance"))
    {
        let pool_token_id = key::segment(&delta.key, 1);
        tables
            .update_row("PoolToken", &format!("0x{pool_token_id}"))
            .set("balance", &delta.new_value);
    }
}
