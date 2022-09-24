mod abi;
mod pb;

use hex_literal::hex;
use pb::balancer;
use substreams::{log, store, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, NULL_ADDRESS};

const WEIGHTED_V2_FACTORY: [u8; 20] = hex!("cc508a455f5b0073973107db6a878ddbdab957bc");

substreams_ethereum::init!();
