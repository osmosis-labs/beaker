use std::str::FromStr;

use anyhow::anyhow;
use cosmrs::AccountId;

pub fn compute_admin(
    admin: Option<&String>,
    signer_account_id: AccountId,
) -> Result<Option<AccountId>, anyhow::Error> {
    Ok(if admin == Some(&"signer".to_string()) {
        Some(signer_account_id)
    } else if let Some(addr) = admin {
        Some(AccountId::from_str(addr).map_err(|e: cosmrs::ErrorReport| anyhow!(e))?)
    } else {
        None
    })
}
