use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Developer::Table)
                    .if_not_exists()
                    .col(pk_auto(Developer::Id))
                    .col(text(Developer::Name))
                    .col(text_null(Developer::Url))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Developer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Developer {
    Table,
    Id,
    Name,
    Url,
}
