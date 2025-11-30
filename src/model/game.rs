use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Game {
    pub id: usize,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero_url: Option<String>,
}

impl From<steamgriddb_api::search::SearchResult> for Game {
    fn from(result: steamgriddb_api::search::SearchResult) -> Self {
        Game {
            id: result.id,
            name: result.name,
            logo_url: None,
            hero_url: None,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct GameList {
    pub games: Vec<Game>,
    pub is_final: bool
}

