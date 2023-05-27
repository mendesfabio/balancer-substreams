use crate::{abi, eth, Token};
use substreams::log;
use substreams::Hex;
use substreams_ethereum::rpc::RpcBatch;

pub fn get_erc20_token(token_address: &String) -> Option<Token> {
    let batch = RpcBatch::new();
    let responses = batch
        .add(
            abi::erc20::functions::Decimals {},
            hex::decode(token_address).unwrap(),
        )
        .add(
            abi::erc20::functions::Name {},
            hex::decode(token_address).unwrap(),
        )
        .add(
            abi::erc20::functions::Symbol {},
            hex::decode(token_address).unwrap(),
        )
        .execute()
        .unwrap()
        .responses;

    let decimals: u64;
    match RpcBatch::decode::<_, abi::erc20::functions::Decimals>(&responses[0]) {
        Some(decoded_decimals) => {
            decimals = decoded_decimals.to_u64();
        }
        None => {
            log::debug!(
                "{} is not a an ERC20 token contract decimal `eth_call` failed",
                Hex(&token_address),
            );

            return None;
        }
    };
    log::debug!("decoded_decimals ok");

    let name: String;
    match RpcBatch::decode::<_, abi::erc20::functions::Name>(&responses[1]) {
        Some(decoded_name) => {
            name = decoded_name;
        }
        None => {
            log::debug!(
                "{} is not a an ERC20 token contract name `eth_call` failed",
                &token_address,
            );
            name = eth::read_string_from_bytes(responses[1].raw.as_ref());
        }
    };
    log::debug!("decoded_name ok");

    let symbol: String;
    match RpcBatch::decode::<_, abi::erc20::functions::Symbol>(&responses[2]) {
        Some(decoded_symbol) => {
            symbol = decoded_symbol;
        }
        None => {
            log::debug!(
                "{} is not a an ERC20 token contract symbol `eth_call` failed",
                &token_address,
            );
            symbol = eth::read_string_from_bytes(responses[2].raw.as_ref());
        }
    };
    log::debug!("decoded_symbol ok");

    return Some(Token {
        id: token_address.clone(),
        address: token_address.clone(),
        name,
        symbol,
        decimals,
    });
}
