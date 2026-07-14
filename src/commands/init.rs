use crate::utils::vault::Vault;
use zeroize::Zeroizing;
use rpassword::prompt_password;
use anyhow::Result;

pub fn cmd_init() -> Result<()> {
    if Vault::exists() {
        anyhow::bail!("vault already exists at {}", Vault::path()?.display());
    }

    eprintln!("\n{}\n", r#" /$$$$$$$$                       /$$$$$$                      /$$
| $$_____/                      /$$__  $$                    | $$
| $$       /$$$$$$$  /$$    /$$| $$  \__/  /$$$$$$   /$$$$$$ | $$
| $$$$$   | $$__  $$|  $$  /$$/|  $$$$$$  /$$__  $$ |____  $$| $$
| $$__/   | $$  \ $$ \  $$/$$/  \____  $$| $$$$$$$$  /$$$$$$$| $$
| $$      | $$  | $$  \  $$$/   /$$  \ $$| $$_____/ /$$__  $$| $$
| $$$$$$$$| $$  | $$   \  $/   |  $$$$$$/|  $$$$$$$|  $$$$$$$| $$
|________/|__/  |__/    \_/     \______/  \_______/ \_______/|__/
                                                                 
                                                                   "#
            );
 
    let password = Zeroizing::new(prompt_password("Set a Master Password: ")?);
    let confirm = Zeroizing::new(prompt_password("Confirm Master Password: ")?);
    if *password != *confirm {
        anyhow::bail!("passwords did not match");
    }
    
    Vault::init(&password)?;
    eprintln!("Seal created at {}", Vault::path()?.display());
    Ok(())
}