use actix_web::{
    get,
    http::StatusCode,
    web::{Data, Query},
};
use serde::Deserialize;

use crate::{
    GlobalState, model::{Game, GameList, Response}
};

#[derive(Deserialize)]
pub struct SearchGamesQuery {
    pub query: String,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub include_hero: Option<bool>,
    pub include_logo: Option<bool>,
}

#[get("/games")]
pub async fn search_games(
    global_data: Data<GlobalState>,
    query: Query<SearchGamesQuery>,
) -> Response<GameList> {
    let Ok(results) = global_data.search_api(&query.query).await else {
        return Response::error(
            "Failed to fetch search results".into(),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
    };

    let is_final =
        results.len() <= query.limit.unwrap_or(15) as usize + query.offset.unwrap_or(0) as usize;

    let offset = query.offset.unwrap_or(0) as usize;
    let limit = query.limit.unwrap_or(15) as usize;

    let mut games: Vec<Game> = results
        .into_iter()
        .skip(offset)
        .take(limit)
        .map(|res| res.into())
        .collect();

    let include_logo = query.include_logo.unwrap_or(false);
    let include_hero = query.include_hero.unwrap_or(false);

    for game in &mut games {
        if include_logo {
            if let Ok(Some(url)) = global_data.get_first_logo_by_game_id(game.id).await {
                game.logo_url = Some(url);
            }
        }

        if include_hero {
            if let Ok(Some(url)) = global_data.get_first_hero_by_game_id(game.id).await {
                game.hero_url = Some(url);
            }
        }
    }

    Response::success(GameList { games, is_final })
}
