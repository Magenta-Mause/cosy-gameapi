use actix_web::{get, http::StatusCode, web::{self, Data, Query}};

use crate::{GlobalState, model::{AssetList, Response}};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FetchAssetsQuery {
    limit: Option<u32>,
    offset: Option<u32>,
}

#[get("/assets/{game_id}")]
pub async fn get_assets_by_id(global_data: Data<GlobalState>, path: web::Path<usize>, query: Query<FetchAssetsQuery>) -> Response<AssetList> {
    let game_id = path.into_inner();
    let Ok(results) = global_data.fetch_assets_by_game_id(game_id).await else {
        return Response::error(
            "Failed to fetch assets".into(),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
    };

    let is_final = results.len() <= query.limit.unwrap_or(15) as usize + query.offset.unwrap_or(0) as usize;

    Response::success(
        AssetList {
            assets: results
                .into_iter()
                .skip(query.offset.unwrap_or(0) as usize)
                .take(query.limit.unwrap_or(15) as usize)
                .map(|img| img.into())
                .collect(),
            is_final
        }
    )
}
