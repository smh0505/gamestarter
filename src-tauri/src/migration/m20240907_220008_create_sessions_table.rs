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
                    .table(Session::Table)
                    .if_not_exists()
                    .col(pk_auto(Session::Id))
                    .col(integer(Session::GameId))
                    .col(date_time(Session::StartTime))
                    .col(date_time(Session::EndTime))
                    .col(integer(Session::Duration))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Game::Table, Game::Id)
                            .to(Session::Table, Session::GameId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Session {
    Table,
    Id,
    #[sea_orm(Iden = "game_id")]
    GameId,
    #[sea_orm(Iden = "start_time")]
    StartTime,
    #[sea_orm(Iden = "end_time")]
    EndTime,
    Duration,
}
