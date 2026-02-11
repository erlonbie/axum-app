pub use sea_orm_migration::prelude::*;
mod common;
mod m20220101_000001_create_user;
mod m20260211_021635_create_posts;
mod m20260211_175842_oauth_providers;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user::Migration),
            Box::new(m20260211_021635_create_posts::Migration),
            Box::new(m20260211_175842_oauth_providers::Migration),
        ]
    }
}
