use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("WeightedPool", "abi/pool.json")?
        .generate()?
        .write_to_file("src/abi/pool.rs")?;
    Abigen::new("WeightedPoolFactory", "abi/factory.json")?
        .generate()?
        .write_to_file("src/abi/factory.rs")?;

    Ok(())
}
