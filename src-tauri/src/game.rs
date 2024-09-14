use std::path::Path;

use chrono::Local;
use sea_orm::*;

use crate::{
    entities::{game, prelude::Game},
    error::BackendError,
    AppState, Response,
};

struct GameQuery;

impl GameQuery {
    async fn create(
        db: &DbConn,
        form_data: game::Model,
    ) -> Result<InsertResult<game::ActiveModel>, DbErr> {
        let model = game::ActiveModel {
            name: Set(form_data.name.to_owned()),
            directory: Set(form_data.directory.to_owned()),
            installed: Set(form_data.installed.to_owned()),
            playtime: Set(form_data.playtime.to_owned()),
            ..Default::default()
        };
        let result = Game::insert(model.clone()).exec(db).await?;
        Ok(result)
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

    async fn delete(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let game: game::ActiveModel = Game::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom(format!("Cannot find game with id {}", id)))
            .map(Into::into)?;
        game.delete(db).await
    }
}

#[tauri::command]
pub async fn add_game(
    state: tauri::State<'_, AppState>,
    file: &str,
) -> Result<Response<i32>, BackendError> {
    let dir = Path::new(file);
    let form = game::Model {
        id: 0,
        name: dir
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap(),
        caption: None,
        description: None,
        developer: None,
        image: None,
        directory: dir.to_str().unwrap().to_owned(),
        executive: None,
        installed: Local::now().naive_local(),
        playtime: 0,
    };

    let result = GameQuery::create(&state.conn, form).await?;

    Ok(Response {
        kind: "success".to_owned(),
        message: "GameQuery::Create successful".to_owned(),
        result: Some(result.last_insert_id),
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
