use substreams::scalar::BigDecimal;
use substreams::store::{key_first_segment_in, DeltaBigInt, Deltas};

use crate::key;
use crate::pb::balancer::{Pool, PoolToken, PoolTokens, Pools};
use crate::tables::Tables;

pub fn pools_registered_pool_entity_changes(tables: &mut Tables, pools: &Pools) {
    for pool in &pools.pools {
        create_pool_entity(tables, pool);
    }
}

fn create_pool_entity(tables: &mut Tables, pool: &Pool) {
    tables
        .create_row("Pool", format!("0x{}", &pool.id))
        .set("address", format!("0x{}", &pool.address))
        .set("log_ordinal", format!("0x{}", &pool.log_ordinal));
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
    let bigdecimal0 = BigDecimal::from(0);

    tables
        .create_row("PoolToken", format!("0x{}", &pool_token.id))
        .set("address", format!("0x{}", &pool_token.address))
        .set("pool_id", format!("0x{}", &pool_token.pool_id))
        .set("balance", bigdecimal0)
        .set("log_ordinal", format!("0x{}", &pool_token.log_ordinal));
}

pub fn pool_token_balance_entity_change(
    tables: &mut Tables,
    pool_token_balances_store_deltas: &Deltas<DeltaBigInt>,
) {
    for delta in pool_token_balances_store_deltas
        .deltas
        .iter()
        .filter(key_first_segment_in("PoolToken"))
    {
        let pool_token_id = key::segment(&delta.key, 1);
        tables
            .update_row("PoolToken", &format!("0x{pool_token_id}"))
            .set("balance", &delta.new_value);
    }
}
