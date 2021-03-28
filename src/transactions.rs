use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::{Client, PagedResponse, PaginationOptions, Response};
use crate::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    /// Resource ID
    pub id: String,
    /// constant `"transaction"`
    pub resource: String,
    pub resource_path: String,
    /// Transaction type
    pub r#type: TransactionType,
    /// Status
    pub status: TransactionStatus,
    /// Amount in bitcoin, bitcoin cash, litecoin or ethereum
    pub amount: Money,
    /// Amount in user’s native currency
    pub native_amount: Money,
    /// User defined description
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    /// Sent bitcoin/bitcoin cash/litecoin/ethereum to a bitcoin/bitcoin cash/litecoin/ethereum address or email
    Send,
    /// Requested bitcoin/bitcoin cash/litecoin/ethereum from a user or email
    Request,
    /// Transfered funds between two of a user’s accounts
    Transfer,
    /// Bought bitcoin, bitcoin cash, litecoin or ethereum
    Buy,
    /// Sold bitcoin, bitcoin cash, litecoin or ethereum
    Sell,
    /// Deposited funds into a fiat account from a financial institution
    FiatDeposit,
    /// Withdrew funds from a fiat account
    FiatWithdrawal,
    /// Deposited money into Coinbase Pro
    ExchangeDeposit,
    /// Withdrew money from Coinbase Pro
    ExchangeWithdrawal,
    /// Withdrew funds from a vault account
    VaultWithdrawal,
    /// Deposited money into Coinbase Pro
    ProDeposit,
    /// Withdrew money from Coinbase Pro
    ProWithdrawal,
    InflationReward,
}

/// Transactions statuses vary based on the type of the transaction.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    /// Pending transactions (e.g. a send or a buy)
    Pending,
    /// Completed transactions (e.g. a send or a buy)
    Completed,
    /// Failed transactions (e.g. failed buy)
    Failed,
    /// Conditional transaction expired due to external factors
    Expired,
    /// Transaction was canceled
    Canceled,
    /// Vault withdrawal is waiting for approval
    WaitingForSignature,
    /// Vault withdrawal is waiting to be cleared
    WaitingForClearing,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Money {
    pub amount: String,
    pub currency: String,
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
    /// List transactions
    ///
    /// Lists account’s transactions. See transaction resource for more information.
    pub async fn list_transactions(
        &self,
        account_id: &str,
        pagination: &PaginationOptions,
    ) -> Result<PagedResponse<Transaction>> {
        self.send_request(
            Method::GET,
            &format!(
                "accounts/{}/transactions{}",
                account_id,
                pagination.get_query()
            ),
            None::<&()>,
        )
        .await
    }

    /// Show a transaction
    ///
    /// Show an individual transaction for an account. See transaction resource for more information.
    pub async fn get_transaction(
        &self,
        account_id: &str,
        transaction_id: &str,
    ) -> Result<Response<Transaction>> {
        self.send_request(
            Method::GET,
            &format!("accounts/{}/transactions/{}", account_id, transaction_id),
            None::<&()>,
        )
        .await
    }
}
