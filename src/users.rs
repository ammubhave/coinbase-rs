use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    /// Resource ID
    pub id: String,
    /// constant `"user"`
    pub resource: String,
    pub resource_path: String,
    /// User’s public name
    pub name: Option<String>,
    /// Payment method’s native currency
    pub username: Option<String>,
    /// Location for user’s public profile
    pub profile_location: Option<String>,
    /// Bio for user’s public profile
    pub profile_bio: Option<String>,
    /// Public profile location if user has one
    pub profile_url: Option<String>,
    /// User’s avatar url
    pub avatar_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub method: String,
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserUpdate<'a> {
    /// User’s public name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Time zone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<&'a str>,
    /// Local currency used to display amounts converted from BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_currency: Option<&'a str>,
}

impl Client {
    /// Show a user
    ///
    /// Get any user’s public information with their ID.
    pub async fn get_user(&self, user_id: &str) -> Result<Response<User>> {
        self.send_request(Method::GET, &format!("users/{}", user_id), None::<&()>)
            .await
    }

    /// Show current user
    ///
    /// Get current user’s public information.
    pub async fn get_current_user(&self) -> Result<Response<User>> {
        self.send_request(Method::GET, "user", None::<&()>).await
    }

    /// Show authorization information
    ///
    /// Get current user’s authorization information including granted scopes and send limits when using OAuth2 authentication.
    pub async fn get_auth_info(&self) -> Result<Response<Auth>> {
        self.send_request(Method::GET, "user/auth", None::<&()>)
            .await
    }

    /// Update current user
    ///
    /// Modify current user and their preferences.
    pub async fn update_user<'a>(&self, update: &'a UserUpdate<'a>) -> Result<Response<User>> {
        self.send_request(Method::PUT, "user", Some(update)).await
    }
}
