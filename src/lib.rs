//! Rust async client library for accessing the [Coinbase API](https://developers.coinbase.com/api/v2).
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! coinbase = "1"
//! ```
//!
//! To make API calls, you need to create an instance of the Coinbase **[`Client`][client]**. The client can be created by calling `coinbase::Client::new(api_key, api_secret)`, or by calling `coinbase::Client::from_env()` and passing the credentials in `COINBASE_API_KEY`, and `COINBASE_API_SECRET` environment variables.
//!
//! ## Examples
//!
//! The following example shows you how to connect to Coinbase, and retrieve basic information:
//!
//! ```ignore
//! use coinbase::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::from_env().unwrap();
//!
//!     println!(
//!         "Auth Info: {}",
//!         serde_json::to_string(&client.get_auth_info().await.unwrap()).unwrap()
//!     );
//!
//!     println!(
//!         "Current User: {}",
//!         serde_json::to_string(&client.get_current_user().await.unwrap()).unwrap()
//!     );
//!
//!     let accounts_resp = client.list_accounts(&Default::default()).await.unwrap();
//!     println!(
//!         "Accounts: {}",
//!         serde_json::to_string(&accounts_resp).unwrap()
//!     );
//!
//!     for account in &accounts_resp.data {
//!         let transactions_resp = client
//!             .list_transactions(&account.id, &PaginationOptions::default())
//!             .await
//!             .unwrap();
//!         println!(
//!             "Transactions for account {}: {}",
//!             &account.id,
//!             serde_json::to_string(&transactions_resp).unwrap()
//!         );
//!     }
//! }
//! ```

pub mod accounts;
pub mod client;
pub mod errors;
pub mod transactions;
pub mod users;

pub use client::Client;
