use sea_orm::*;

use crate::{
    entities::{game, prelude::Game},
    error::BackendError,
    AppState, Response,
};

pub struct GameQuery;

impl GameQuery {
    async fn create(db: &DbConn, form_data: game::Model) -> Result<game::ActiveModel, DbErr> {
        game::ActiveModel {
            name: Set(form_data.name.to_owned()),
            directory: Set(form_data.directory.to_owned()),
            installed: Set(form_data.installed.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    async fn read_page(
        db: &DbConn,
        page: u64,
        posts: u64,
    ) -> Result<(Vec<game::Model>, u64), DbErr> {
        let paginator = Game::find()
            .order_by_asc(game::Column::Id)
            .paginate(db, posts);
        let num_pages = paginator.num_pages().await?;
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}

#[tauri::command]
pub async fn add_game(
    state: tauri::State<'_, AppState>,
    form: game::Model,
) -> Result<Response, BackendError> {
    GameQuery::create(&state.conn, form).await?;

    Ok(Response {
        kind: "success".to_owned(),
        message: "GameQuery::Create successful".to_owned(),
    })
}

#[tauri::command]
pub async fn list_games(
    state: tauri::State<'_, AppState>,
    page: u64,
    posts: u64,
) -> Result<Vec<game::Model>, BackendError> {
    let (posts, num_pages) = GameQuery::read_page(&state.conn, page, posts).await?;

    println!("num_pages: {}", num_pages);
    Ok(posts)
}
