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
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(pk_auto(Tag::Id))
                    .col(text(Tag::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GameTag::Table)
                    .if_not_exists()
                    .col(integer(GameTag::GameId))
                    .col(integer(GameTag::TagId))
                    .primary_key(Index::create().col(GameTag::GameId).col(GameTag::TagId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(GameTag::Table, GameTag::GameId)
                            .to(Game::Table, Game::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GameTag::Table, GameTag::TagId)
                            .to(Tag::Table, Tag::Id)
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
                    .table(Tag::Table)
                    .table(GameTag::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
#[sea_orm(iden = "game_tags")]
enum GameTag {
    Table,
    #[sea_orm(iden = "game_id")]
    GameId,
    #[sea_orm(iden = "tag_id")]
    TagId,
}
