use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20220101_000001_users_table::Users, m20240928_182418_products_table::Product};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Review::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Review::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Review::ProductId).integer().not_null())
                    .col(ColumnDef::new(Review::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-review-product_id")
                            .from(Review::Table, Review::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-review-user_id")
                            .from(Review::Table, Review::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Review::Rating).integer())
                    .col(ColumnDef::new(Review::ReviewText).text())
                    .col(
                        ColumnDef::new(Review::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Review::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Review::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Review {
    Table,
    Id,
    ProductId,
    UserId,
    Rating,
    ReviewText,
    CreatedAt,
    UpdatedAt,
}
