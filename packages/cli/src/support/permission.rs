use std::str::FromStr;

use anyhow::anyhow;
use cosmrs::{cosmwasm::AccessConfig, AccountId};

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

pub fn compute_instantiate_permission(
    permit_instantiate_only: &Option<String>,
    signer_account_id: AccountId,
) -> Result<Option<AccessConfig>, anyhow::Error> {
    let instantiate_permission = permit_instantiate_only
        .as_ref()
        .map(|permitted_account| {
            let address = if permitted_account == "signer" {
                signer_account_id
            } else {
                permitted_account
                    .parse()
                    .map_err(|e: cosmrs::ErrorReport| anyhow::anyhow!(e))?
            };

            anyhow::Ok(AccessConfig {
                permission: cosmrs::cosmwasm::AccessType::OnlyAddress,
                address,
            })
        })
        .transpose()?;
    Ok(instantiate_permission)
}
