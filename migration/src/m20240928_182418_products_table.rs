use crate::m20240928_182418_categories_table::Category;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::Name).string_len(255).not_null())
                    .col(ColumnDef::new(Product::Description).text())
                    .col(ColumnDef::new(Product::Price).decimal_len(10, 2).not_null())
                    .col(ColumnDef::new(Product::Stock).integer().default(0))
                    .col(ColumnDef::new(Product::CategoryId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-product-category_id")
                            .from(Product::Table, Product::CategoryId)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(ColumnDef::new(Product::ImageUrl).string_len(255))
                    .col(
                        ColumnDef::new(Product::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Product::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-product-name")
                    .table(Product::Table)
                    .col(Product::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-product-category_id")
                    .table(Product::Table)
                    .col(Product::CategoryId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx-product-name")
                    .table(Product::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx-product-category_id")
                    .table(Product::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Product {
    Table,
    Id,
    Name,
    Description,
    Price,
    Stock,
    CategoryId,
    ImageUrl,
    CreatedAt,
    UpdatedAt,
}
