use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct EmbyClient {
    client: Client,
    server_url: String,
    api_key: String,
    user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub user_id: String,
}

impl EmbyClient {
    pub fn new(server_url: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            server_url,
            api_key,
            user_id: String::new(),
        }
    }

    pub async fn authenticate(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/Users/AuthenticateByName", self.server_url);
        let params = [("username", username), ("pw", password)];

        let response = self
            .client
            .post(&url)
            .header("X-Emby-Token", &self.api_key)
            .form(&params)
            .send()
            .await?
            .json::<AuthResponse>()
            .await?;

        self.user_id = response.user_id;
        Ok(())
    }

    pub async fn get_playback_url(&self, item_id: &str) -> Result<String, Box<dyn Error>> {
        let url = format!(
            "{}/Items/{}/PlaybackInfo?UserId={}&StartTimeTicks=0",
            self.server_url, item_id, self.user_id
        );

        let response = self
            .client
            .get(&url)
            .header("X-Emby-Token", &self.api_key)
            .send()
            .await?;

        // TODO: Parse response and return playback URL
        Ok(String::new())
    }
}
