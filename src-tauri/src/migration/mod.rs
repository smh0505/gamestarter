pub use sea_orm_migration::prelude::*;

mod m20240905_153422_create_games_table;
mod m20240905_155059_create_tags_table;
mod m20240907_213015_create_genres_table;
mod m20240907_214452_create_developers_table;
mod m20240907_220008_create_sessions_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240907_214452_create_developers_table::Migration),
            Box::new(m20240905_153422_create_games_table::Migration),
            Box::new(m20240905_155059_create_tags_table::Migration),
            Box::new(m20240907_213015_create_genres_table::Migration),
            Box::new(m20240907_220008_create_sessions_table::Migration),
        ]
    }
}
