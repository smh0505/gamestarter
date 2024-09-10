use sea_orm_migration::{prelude::*, schema::*};

use crate::migration::m20240905_153422_create_games_table::Game;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Genre::Table)
                    .if_not_exists()
                    .col(pk_auto(Genre::Id))
                    .col(text(Genre::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GameGenre::Table)
                    .if_not_exists()
                    .col(integer(GameGenre::GameId))
                    .col(integer(GameGenre::GenreId))
                    .primary_key(
                        Index::create()
                            .col(GameGenre::GameId)
                            .col(GameGenre::GenreId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GameGenre::Table, GameGenre::GameId)
                            .to(Game::Table, Game::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GameGenre::Table, GameGenre::GenreId)
                            .to(Genre::Table, Genre::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Genre::Table)
                    .table(GameGenre::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Genre {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
#[sea_orm(Iden = "game_genres")]
enum GameGenre {
    Table,
    #[sea_orm(Iden = "game_id")]
    GameId,
    #[sea_orm(Iden = "genre_id")]
    GenreId,
}
