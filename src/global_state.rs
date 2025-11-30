use std::{error::Error};

use reqwest::header::{HeaderMap, HeaderValue};
use steamgriddb_api::{search::SearchResult};
use tokio::sync::Mutex;

use crate::{steamgriddb_models};

pub struct GlobalState {
    steamgriddb_api_client: Mutex<steamgriddb_api::client::Client>,
    reqwest_client: Mutex<reqwest::Client>
}

impl GlobalState {
    pub fn new(auth_key: &str) -> Result<Self, Box<dyn Error>> {
        let mut client_headers: HeaderMap<HeaderValue> = reqwest::header::HeaderMap::default();
        client_headers.insert("Authorization", format!("Bearer {}", auth_key)
            .parse()
            .map_err(|e| format!("Failed to parse auth header: {}", e))?
        );

        let client = reqwest::Client::builder()
            .default_headers(client_headers)
            .build()
            .unwrap();

        Ok(Self {
            steamgriddb_api_client: Mutex::new(
                steamgriddb_api::Client::new(auth_key)
            ),
            reqwest_client: Mutex::new(
                client
            )
        })
    }

    pub async fn search_api(&self, query: &str) -> Result<Vec<SearchResult>, Box<dyn Error>> {
        let client = self.steamgriddb_api_client.lock().await;
        client.search(query).await
    }

    pub async fn fetch_assets_by_game_id(&self, game_id: usize) -> Result<Vec<steamgriddb_api::images::Image>, Box<dyn Error>> {
        let client = self.steamgriddb_api_client.lock().await;
        client.get_images_for_id(game_id, &steamgriddb_api::QueryType::Grid(None)).await
    }

    pub async fn get_first_logo_by_game_id(&self, game_id: usize) -> Result<Option<String>, Box<dyn Error>> {
        let client = self.reqwest_client.lock().await;
        let sgdbc = self.steamgriddb_api_client.lock().await;

        let base_url = sgdbc.base_url();
        let logos_url = format!("{}/logos/game/{}", base_url, game_id);

        let logos_resp = client.get(logos_url).query(&[
            ("limit", 1)
        ]).send().await?;

        if !logos_resp.status().is_success() {
            return Err("Failed to fetch logo".into());
        }

        let logos_resp_json: steamgriddb_models::LogosResponse = serde_json::from_str(&logos_resp.text().await?)?;
        if !logos_resp_json.success || logos_resp_json.data.is_empty() {
            return Err("No logo found".into());
        }

        let Some(first) = logos_resp_json.data.first() else {
            return Ok(None);
        };

        Ok(Some(first.url.to_owned()))
    }

    pub async fn get_first_hero_by_game_id(&self, game_id: usize) -> Result<Option<String>, Box<dyn Error>> {
        let client = self.reqwest_client.lock().await;
        let sgdbc = self.steamgriddb_api_client.lock().await;

        let base_url = sgdbc.base_url();
        let heroes_url = format!("{}/heroes/game/{}", base_url, game_id);

        let heroes_resp = client.get(heroes_url).query(&[
            ("limit", 1)
        ]).send().await?;

        if !heroes_resp.status().is_success() {
            return Err("Failed to fetch logo".into());
        }

        let heroes_resp_json: steamgriddb_models::HeroesResponse = serde_json::from_str(&heroes_resp.text().await?)?;

        if !heroes_resp_json.success || heroes_resp_json.data.is_empty() {
            return Err("No logo found".into());
        }

        let Some(first) = heroes_resp_json.data.first() else {
            return Ok(None);
        };

        Ok(Some(first.url.to_owned()))
    }
}