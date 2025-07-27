use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
    create_table(
      m,
      "links",
      &[
        ("id", ColType::PkAuto),
        ("user_id", ColType::Integer),
        ("url", ColType::String),
        ("short_url", ColType::StringUniq),
      ],
      &[]
    ).await
  }

  async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
    drop_table(m, "links").await
  }
}
