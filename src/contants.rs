use substreams::hex;

type Address = [u8; 20];

pub const VAULT_ADDRESS: Address = hex!("ba12222222228d8ba445958a75a0704d566bf2c8");

const WEIGHTED_V2_FACTORY: Address = hex!("cc508a455f5b0073973107db6a878ddbdab957bc");

#[allow(dead_code)]
pub fn get_pool_type_from_factory(address: Address) -> &'static str {
    match address {
        WEIGHTED_V2_FACTORY => return "Weighted",
        _ => return "Unknown",
    }
}
