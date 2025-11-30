use actix_web::{
    get,
    http::StatusCode,
    web::{Data, Query},
};
use futures::StreamExt;
use serde::Deserialize;

use crate::{
    model::{Game, GameList, Response},
    GlobalState,
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

    /* Parallelize logo/hero fetching */
    futures::stream::iter(games.iter_mut())
        .map(|game| {
            let global = global_data.clone();
            async move {
                // create a per-task service instance (cheap, clones Arcs)
                let service = global.steamgriddb_service();

                if include_logo {
                    if let Ok(Some(url)) = service.get_first_logo_by_game_id(game.id).await {
                        game.logo_url = Some(url);
                    }
                }
                if include_hero {
                    if let Ok(Some(url)) = service.get_first_hero_by_game_id(game.id).await {
                        game.hero_url = Some(url);
                    }
                }
            }
        })
        .buffer_unordered(8)
        .for_each(|_| async {})
        .await;

    Response::success(GameList { games, is_final })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Game;

    #[tokio::test]
    async fn parallel_fetch_sets_logo() {
        let mut games = [
            Game {
                id: 1,
                name: "a".into(),
                logo_url: None,
                hero_url: None,
            },
            Game {
                id: 2,
                name: "b".into(),
                logo_url: None,
                hero_url: None,
            },
            Game {
                id: 3,
                name: "c".into(),
                logo_url: None,
                hero_url: None,
            },
        ];

        let include_logo = true;
        let include_hero = false;

        futures::stream::iter(games.iter_mut())
            .map(|game| async move {
                // simulate network delay per entry
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                if include_logo {
                    game.logo_url = Some(format!("https://example/{}/logo.png", game.id));
                }
                if include_hero {
                    game.hero_url = Some(format!("https://example/{}/hero.png", game.id));
                }
            })
            .buffer_unordered(8)
            .for_each(|_| async {})
            .await;

        // all games should have logo set, none should have hero
        for g in games.iter() {
            assert!(g.logo_url.is_some());
            assert!(g.hero_url.is_none());
        }
    }

    #[tokio::test]
    async fn parallel_fetch_sets_logo_and_hero() {
        let mut games = [
            Game {
                id: 10,
                name: "x".into(),
                logo_url: None,
                hero_url: None,
            },
            Game {
                id: 11,
                name: "y".into(),
                logo_url: None,
                hero_url: None,
            },
        ];

        let include_logo = true;
        let include_hero = true;

        futures::stream::iter(games.iter_mut())
            .map(|game| async move {
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                if include_logo {
                    game.logo_url = Some(format!("https://example/{}/logo.png", game.id));
                }
                if include_hero {
                    game.hero_url = Some(format!("https://example/{}/hero.png", game.id));
                }
            })
            .buffer_unordered(8)
            .for_each(|_| async {})
            .await;

        for g in games.iter() {
            assert!(g.logo_url.is_some());
            assert!(g.hero_url.is_some());
        }
    }
}
