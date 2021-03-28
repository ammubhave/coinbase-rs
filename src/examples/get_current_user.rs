use dotenv::dotenv;

use coinbase::client::PaginationOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let client = coinbase::Client::from_env().unwrap();

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
