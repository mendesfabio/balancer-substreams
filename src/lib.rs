mod abi;
mod pb;

use hex_literal::hex;
use pb::balancer::{Pool, Pools};
use substreams::errors::Error;
use substreams::{log, proto, store, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event};

const WEIGHTED_V2_FACTORY: [u8; 20] = hex!("cc508a455f5b0073973107db6a878ddbdab957bc");

substreams_ethereum::init!();

#[substreams::handlers::map]
pub fn map_pools_created(block: eth::Block) -> Result<Pools, Error> {
    let mut pools = vec![];

    for log in block.logs() {
        if let Some(event) = abi::factory::events::PoolCreated::match_and_decode(log) {
            log::info!("Pool address: {}", Hex(&event.pool));

            if log.address() != WEIGHTED_V2_FACTORY {
                continue;
            };

            pools.push(Pool {
                address: Hex(&event.pool).to_string(),
            });
        }
    }

    Ok(Pools { pools })
}

#[substreams::handlers::store]
fn store_pools_created(pools_created: Pools, output: store::StoreSet) {
    log::info!("Pools created: {}", pools_created.pools.len());

    for pool in pools_created.pools {
        output.set(
            0,
            Hex::encode(&pool.address),
            &proto::encode(&pool).unwrap(),
        );
    }
}
