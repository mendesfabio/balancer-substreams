use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("Vault", "abi/vault.json")?
        .generate()?
        .write_to_file("src/abi/vault.rs")?;

    Ok(())
}
