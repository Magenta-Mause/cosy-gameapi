use std::{error::Error, sync::Arc};

use reqwest::header::{HeaderMap, HeaderValue};
use steamgriddb_api::{search::SearchResult};

use crate::{steamgriddb_models};

pub struct GlobalState {
    steamgriddb_api_client: Arc<steamgriddb_api::client::Client>,
    reqwest_client: Arc<reqwest::Client>
}

impl GlobalState {
    pub fn new(auth_key: &str) -> Result<Self, Box<dyn Error>> {
        let mut client_headers: HeaderMap<HeaderValue> = reqwest::header::HeaderMap::default();
        client_headers.insert("Authorization", format!("Bearer {}", auth_key)
            .parse()
            .map_err(|e| format!("Failed to parse auth header: {}", e))?
        );

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .default_headers(client_headers)
            .build()?;

        Ok(Self {
            steamgriddb_api_client: Arc::new(
                steamgriddb_api::Client::new(auth_key)
            ),
            reqwest_client: Arc::new(
                client
            )
        })
    }

    pub async fn search_api(&self, query: &str) -> Result<Vec<SearchResult>, Box<dyn Error>> {
        self.steamgriddb_api_client.search(query).await
    }

    pub async fn fetch_assets_by_game_id(&self, game_id: usize) -> Result<Vec<steamgriddb_api::images::Image>, Box<dyn Error>> {
        self.steamgriddb_api_client.get_images_for_id(game_id, &steamgriddb_api::QueryType::Grid(None)).await
    }

    pub async fn get_first_logo_by_game_id(&self, game_id: usize) -> Result<Option<String>, Box<dyn Error>> {
        let base_url = self.steamgriddb_api_client.base_url();
        let logos_url = format!("{}/logos/game/{}", base_url, game_id);

        let logos_resp = self.reqwest_client.get(logos_url).query(&[
            ("limit", 1)
        ]).send().await?;

        if !logos_resp.status().is_success() {
            return Err("Failed to fetch logo".into());
        }

        let logos_resp_json: steamgriddb_models::LogosResponse = serde_json::from_str(&logos_resp.text().await?)?;

        if !logos_resp_json.success {
            return Err("steamgriddb API returned success=false for logos".into());
        }

        if logos_resp_json.data.is_empty() {
            return Ok(None);
        }

        let first = logos_resp_json.data.first().expect("checked non-empty above");

        Ok(Some(first.url.to_owned()))
    }

    pub async fn get_first_hero_by_game_id(&self, game_id: usize) -> Result<Option<String>, Box<dyn Error>> {
        let base_url = self.steamgriddb_api_client.base_url();
        let heroes_url = format!("{}/heroes/game/{}", base_url, game_id);

        let heroes_resp = self.reqwest_client.get(heroes_url).query(&[
            ("limit", 1)
        ]).send().await?;

        if !heroes_resp.status().is_success() {
            return Err("Failed to fetch heroes".into());
        }

        let heroes_resp_json: steamgriddb_models::HeroesResponse = serde_json::from_str(&heroes_resp.text().await?)?;

        if !heroes_resp_json.success {
            return Err("steamgriddb API returned success=false for heroes".into());
        }

        if heroes_resp_json.data.is_empty() {
            return Ok(None);
        }

        let first = heroes_resp_json.data.first().expect("checked non-empty above");

        Ok(Some(first.url.to_owned()))
    }
}