# coinbase-rs

![Build Status](https://github.com/ammubhave/coinbase-rs/actions/workflows/rust.yml/badge.svg)
[![](http://meritbadge.herokuapp.com/coinbase)](https://crates.io/crates/coinbase)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Rust async client library for accessing the [Coinbase API](https://developers.coinbase.com/api/v2).

## Documentation

Please see the [documentation website](https://ammubhave.github.io/coinbase-rs), or at [docs.rs](https://docs.rs/coinbase/).

The source code can be found at [https://github.com/ammubhave/coinbase-rs](https://github.com/ammubhave/coinbase-rs).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
coinbase = "0.1"
```

To make API calls, you need to create an instance of the Coinbase Client. The client can be created by calling `coinbase::Client::new(api_key, api_secret)`, or by calling `coinbase::Client::from_env()` and passing the credentials in `COINBASE_API_KEY`, and `COINBASE_API_SECRET` environment variables.

## Examples

The following example shows you how to connect to Coinbase, and retrieve basic information:

```rust
use coinbase::Client;

#[tokio::main]
async fn main() {
    let client = Client::from_env().unwrap();

    println!(
        "Auth Info: {}",
        serde_json::to_string(&client.get_auth_info().await.unwrap()).unwrap()
    );

    println!(
        "Current User: {}",
        serde_json::to_string(&client.get_current_user().await.unwrap()).unwrap()
    );

    let accounts_resp = client.list_accounts(&Default::default()).await.unwrap();
    println!(
        "Accounts: {}",
        serde_json::to_string(&accounts_resp).unwrap()
    );

    for account in &accounts_resp.data {
        let transactions_resp = client
            .list_transactions(&account.id, &PaginationOptions::default())
            .await
            .unwrap();
        println!(
            "Transactions for account {}: {}",
            &account.id,
            serde_json::to_string(&transactions_resp).unwrap()
        );
    }
}
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](LICENSE).

## Disclaimer

This SOFTWARE PRODUCT is provided by THE PROVIDER "as is" and "with all faults."
THE PROVIDER makes no representations or warranties of any kind concerning the
safety, suitability, lack of viruses, inaccuracies, typographical errors, or
other harmful components of this SOFTWARE PRODUCT. There are inherent dangers
in the use of any software, and you are solely responsible for determining
whether this SOFTWARE PRODUCT is compatible with your equipment and other
software installed on your equipment. You are also solely responsible for the
protection of your equipment and backup of your data, and THE PROVIDER will not
be liable for any damages you may suffer in connection with using, modifying,
or distributing this SOFTWARE PRODUCT.
