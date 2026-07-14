use crate::utils::vault::Vault;
use zeroize::Zeroizing;
use rpassword::prompt_password;
use anyhow::Result;

pub fn cmd_list() -> Result<()> {

    let vault = Vault::load()?;
    let password = Zeroizing::new(prompt_password("Master Password: ")?);
    let _ = vault.unlock(&password)?;

    let keys = vault.list_all_keys().unwrap();
    eprintln!("\nStored-Keys");
    eprintln!("---------------");
    for key in keys {
        eprintln!("{key}");
    }
        Ok(())
}