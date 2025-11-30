use cosy_gameapi::{GlobalState, routes::{get_assets_by_id, search_games}};
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Ok(auth_key) = std::env::var("COSY_GAMEAPI_SGDB_API_KEY") else {
        return Err("COSY_GAMEAPI_SGDB_API_KEY environment variable not set".into());
    };

    let global_state = web::Data::new(GlobalState::new(&auth_key)?);

    HttpServer::new(move || {
        App::new()
            .service(get_assets_by_id)
            .service(search_games)
            .app_data(global_state.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
    .map_err(|e| format!("failed to run server: {}", e))?;

    Ok(())
}
