use sea_orm_migration::{prelude::*, schema::*};

use crate::migration::m20240907_214452_create_developers_table::Developer;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(pk_auto(Game::Id))
                    .col(text(Game::Name).not_null())
                    .col(string_len_null(Game::Caption, 20))
                    .col(text_null(Game::Description))
                    .col(integer_null(Game::Developer))
                    .col(text_null(Game::Image))
                    .col(text(Game::Directory))
                    .col(text_null(Game::Executive))
                    .col(date_time(Game::Installed))
                    .col(big_integer(Game::Playtime))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Game::Table, Game::Developer)
                            .to(Developer::Table, Developer::Id)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Game {
    Table,
    Id,
    Name,
    Caption,
    Description,
    Developer,
    Image,
    Directory,
    Executive,
    Installed,
    Playtime,
}
