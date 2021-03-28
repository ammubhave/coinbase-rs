use hmac::{Mac, NewMac};
use reqwest;
use serde::{Deserialize, Serialize};

use crate::errors::{CoinbaseError, Result};

#[derive(Debug, Clone)]
struct ApiKeyAuthData {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    auth: ApiKeyAuthData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Desc,
    Asc,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pagination {
    pub ending_before: Option<String>,
    pub starting_before: Option<String>,
    pub limit: i32,
    pub order: String,
    pub previous_uri: Option<String>,
    pub next_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagedResponse<T> {
    pub pagination: Pagination,
    pub data: Vec<T>,
}

impl<T> PagedResponse<T> {
    pub fn has_next_page(&self) -> bool {
        self.pagination.next_uri.is_some()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PaginationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

impl PaginationOptions {
    pub fn get_query(&self) -> String {
        let mut query = "".to_string();
        if let Some(ending_before) = &self.ending_before {
            query += "&ending_before=";
            query += &ending_before;
        }
        if let Some(starting_before) = &self.starting_before {
            query += "&starting_before=";
            query += &starting_before;
        }
        if let Some(limit) = self.limit {
            query += "&limit=";
            query += &limit.to_string();
        }
        if let Some(order) = &self.order {
            query += "&order=";
            query += &order;
        }
        query
    }
}

impl Client {
    pub fn from_api_key(
        api_key: String,
        api_secret: String,
    ) -> std::result::Result<Client, hmac::crypto_mac::InvalidKeyLength> {
        hmac::Hmac::<sha2::Sha256>::new_varkey(api_secret.as_bytes())?;

        Ok(Client {
            client: reqwest::Client::builder().build().unwrap(),
            auth: ApiKeyAuthData {
                api_key,
                api_secret,
            },
        })
    }

    pub fn from_env() -> std::result::Result<Client, hmac::crypto_mac::InvalidKeyLength> {
        Client::from_api_key(
            std::env::var("COINBASE_API_KEY")
                .expect("Missing environment variable: COINBASE_API_KEY")
                .to_string(),
            std::env::var("COINBASE_API_SECRET")
                .expect("Missing environment variable: COINBASE_API_SECRET")
                .to_string(),
        )
    }

    pub async fn send_request<T, U>(
        &self,
        method: reqwest::Method,
        url: &str,
        req: Option<&T>,
    ) -> Result<U>
    where
        T: serde::Serialize,
        U: for<'de> serde::Deserialize<'de>,
    {
        let body = if let Some(req) = req {
            serde_json::to_string(req)?
        } else {
            "".to_string()
        };
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let mut mac =
            hmac::Hmac::<sha2::Sha256>::new_varkey(self.auth.api_secret.as_bytes()).unwrap();
        mac.update(format!("{}{}/v2/{}{}", timestamp, method.as_str(), url, body).as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        let resp = self
            .client
            .request(
                method,
                reqwest::Url::parse("https://api.coinbase.com/v2/")
                    .unwrap()
                    .join(url)
                    .unwrap(),
            )
            .header("CB-ACCESS-SIGN", signature)
            .header("CB-ACCESS-TIMESTAMP", timestamp)
            .header("CB-ACCESS-KEY", &self.auth.api_key)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await?;
        if resp.status() == reqwest::StatusCode::OK {
            Ok(resp.json().await?)
            // let resp = resp.text().await?;
            // println!("{}", resp);
            // Ok(serde_json::from_str(&resp)?)
        } else {
            let status_code = resp.status();
            Err(CoinbaseError {
                status_code: status_code,
            }
            .into())
        }
    }
}
