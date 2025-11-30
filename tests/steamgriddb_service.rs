use cosy_gameapi::services::steamgriddb_service::SteamgriddbService;
use httpmock::Method::GET;
use httpmock::MockServer;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn logo_empty_data_returns_none() {
    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(GET).path("/logos/game/42").query_param("limit", "1");
        then.status(200)
            .body(r#"{"success":true,"page":1,"total":0,"limit":1,"data":[]}"#);
    });

    let client = Client::builder().timeout(Duration::from_secs(5)).build().unwrap();
    let service = SteamgriddbService::new(
        Arc::new(steamgriddb_api::Client::new("dummy")),
        Arc::new(client),
        server.base_url(),
    );

    let res = service.get_first_logo_by_game_id(42).await.unwrap();
    assert!(res.is_none());
}

#[tokio::test]
async fn logo_with_data_returns_url() {
    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(GET)
            .path("/logos/game/99")
            .query_param("limit", "1");
        then.status(200).body(r#"{"success":true,"page":1,"total":1,"limit":1,"data":[{"id":1,"url":"https://example.com/logo.png","thumb":"thumb","score":0,"style":"","width":1,"height":1,"nsfw":false,"humor":false,"mime":"image/png","language":"","lock":false,"epilepsy":false,"upvotes":0,"downvotes":0,"author":{"name":"","steam64":"","avatar":""}}]}"#);
    });

    let client = Client::builder().timeout(Duration::from_secs(5)).build().unwrap();
    let service = SteamgriddbService::new(
        Arc::new(steamgriddb_api::Client::new("dummy")),
        Arc::new(client),
        server.base_url(),
    );

    let res = service.get_first_logo_by_game_id(99).await.unwrap();
    assert_eq!(res.unwrap(), "https://example.com/logo.png");
}

#[tokio::test]
async fn logo_non_200_returns_err() {
    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(GET).path("/logos/game/500").query_param("limit", "1");
        then.status(500).body("internal error");
    });

    let client = Client::builder().timeout(Duration::from_secs(5)).build().unwrap();
    let service = SteamgriddbService::new(
        Arc::new(steamgriddb_api::Client::new("dummy")),
        Arc::new(client),
        server.base_url(),
    );

    let res = service.get_first_logo_by_game_id(500).await;
    assert!(res.is_err());
    let err = format!("{}", res.unwrap_err());
    assert!(err.contains("Failed to fetch"));
}

#[tokio::test]
async fn logo_success_false_returns_err() {
    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(GET).path("/logos/game/400").query_param("limit", "1");
        then.status(200).body(r#"{"success":false,"page":1,"total":0,"limit":1,"data":[]}"#);
    });

    let client = Client::builder().timeout(Duration::from_secs(5)).build().unwrap();
    let service = SteamgriddbService::new(
        Arc::new(steamgriddb_api::Client::new("dummy")),
        Arc::new(client),
        server.base_url(),
    );

    let res = service.get_first_logo_by_game_id(400).await;
    assert!(res.is_err());
    let err = format!("{}", res.unwrap_err());
    assert!(err.contains("success=false"));
}

// heroes

#[tokio::test]
async fn hero_empty_data_returns_none() {
    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(GET).path("/heroes/game/12").query_param("limit", "1");
        then.status(200).body(r#"{"success":true,"page":1,"total":0,"limit":1,"data":[]}"#);
    });

    let client = Client::builder().timeout(Duration::from_secs(5)).build().unwrap();
    let service = SteamgriddbService::new(
        Arc::new(steamgriddb_api::Client::new("dummy")),
        Arc::new(client),
        server.base_url(),
    );

    let res = service.get_first_hero_by_game_id(12).await.unwrap();
    assert!(res.is_none());
}

#[tokio::test]
async fn hero_with_data_returns_url() {
    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(GET).path("/heroes/game/7").query_param("limit", "1");
        then.status(200).body(r#"{"success":true,"page":1,"total":1,"limit":1,"data":[{"id":1,"url":"https://example.com/hero.png","thumb":"thumb","score":0,"style":"","width":1,"height":1,"nsfw":false,"humor":false,"mime":"image/png","language":"","lock":false,"epilepsy":false,"upvotes":0,"downvotes":0,"author":{"name":"","steam64":"","avatar":""}}]}"#);
    });

    let client = Client::builder().timeout(Duration::from_secs(5)).build().unwrap();
    let service = SteamgriddbService::new(
        Arc::new(steamgriddb_api::Client::new("dummy")),
        Arc::new(client),
        server.base_url(),
    );

    let res = service.get_first_hero_by_game_id(7).await.unwrap();
    assert_eq!(res.unwrap(), "https://example.com/hero.png");
}

#[tokio::test]
async fn hero_non_200_returns_err() {
    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(GET).path("/heroes/game/500").query_param("limit", "1");
        then.status(503).body("unavailable");
    });

    let client = Client::builder().timeout(Duration::from_secs(5)).build().unwrap();
    let service = SteamgriddbService::new(
        Arc::new(steamgriddb_api::Client::new("dummy")),
        Arc::new(client),
        server.base_url(),
    );

    let res = service.get_first_hero_by_game_id(500).await;
    assert!(res.is_err());
    let err = format!("{}", res.unwrap_err());
    assert!(err.contains("Failed to fetch"));
}

#[tokio::test]
async fn hero_success_false_returns_err() {
    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(GET).path("/heroes/game/400").query_param("limit", "1");
        then.status(200).body(r#"{"success":false,"page":1,"total":0,"limit":1,"data":[]}"#);
    });

    let client = Client::builder().timeout(Duration::from_secs(5)).build().unwrap();
    let service = SteamgriddbService::new(
        Arc::new(steamgriddb_api::Client::new("dummy")),
        Arc::new(client),
        server.base_url(),
    );

    let res = service.get_first_hero_by_game_id(400).await;
    assert!(res.is_err());
    let err = format!("{}", res.unwrap_err());
    assert!(err.contains("success=false"));
}
