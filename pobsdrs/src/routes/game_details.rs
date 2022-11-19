use crate::models::HtmlTemplate;
use askama::Template;
use std::sync::Arc;
use pobsdlib::{DataBase, Game};
use axum::extract::{Extension, Path};
use axum::response::IntoResponse;



#[derive(Template)]
#[template(path = "game_details.html")]
struct GameDetailsTemplate {
    game: Option<Game>,
}

pub async fn game_details(
    Extension(db): Extension<Arc<DataBase>>,
    Path(game_id): Path<usize>,
) -> impl IntoResponse {
    let template = GameDetailsTemplate {
            game: db.get_game_by_id(game_id)
        };
    HtmlTemplate(template)
}
