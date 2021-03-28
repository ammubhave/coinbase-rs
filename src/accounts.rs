use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::{Client, PagedResponse, PaginationOptions, Response};
use crate::errors::Result;
use crate::transactions::Money;

/// Account resource represents all of a user’s accounts, including bitcoin, bitcoin cash, litecoin and ethereum wallets, fiat currency accounts, and vaults. This is represented in the type field. It’s important to note that new types can be added over time so you want to make sure this won’t break your implementation.
/// User can only have one primary account and its type can only be wallet.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    /// Resource ID
    pub id: String,
    /// constant `"account"`
    pub resource: String,
    pub resource_path: String,
    /// User or system defined name
    pub name: String,
    /// Primary account
    pub primary: bool,
    /// Account’s type. Available values: wallet, fiat, vault
    pub r#type: AccountType,
    /// Account’s currency
    pub currency: AccountCurrency,
    /// Balance in BTC or ETH
    pub balance: Money,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Wallet,
    Fiat,
    Vault,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountCurrency {
    pub code: String,
    pub name: String,
    pub color: String,
    pub sort_index: i32,
    pub exponent: i32,
    pub r#type: String,
    pub address_regex: String,
    pub asset_id: String,
    pub slug: String,
}

impl Client {
    /// List accounts
    ///
    /// Lists current user’s accounts to which the authentication method has access to.
    pub async fn list_accounts(
        &self,
        pagination: &PaginationOptions,
    ) -> Result<PagedResponse<Account>> {
        self.send_request(
            Method::GET,
            &format!("accounts{}", pagination.get_query()),
            None::<&()>,
        )
        .await
    }

    /// Show an account
    ///
    /// Show current user’s account. To access the primary account for a given currency, a currency string (BTC or ETH) can be used instead of the account id in the URL.
    pub async fn get_account(&self, account_id: &str) -> Result<Response<Account>> {
        self.send_request(
            Method::GET,
            &format!("accounts/{}", account_id),
            None::<&()>,
        )
        .await
    }
}
