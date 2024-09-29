use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240928_182418_products_table::Product;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Discount::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Discount::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Discount::ProductId)
                            .integer()
                            .not_null()
                            )
                            .foreign_key(
                                ForeignKey::create()
                                    .name("fk-discount-product_id")
                                    .from(Discount::Table, Discount::ProductId)
                                    .to(Product::Table, Product::Id)
                                    .on_delete(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Discount::DiscountPercentage)
                            .decimal_len(5, 2)
                            // .check(
                            //     Expr::tbl(Discount::Table)
                            //         .col(Discount::DiscountPercentage)
                            //         .between(0, 100),
                            // ),
                    )
                    .col(
                        ColumnDef::new(Discount::ValidFrom)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Discount::ValidTo)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Discount::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Discount {
    Table,
    Id,
    ProductId,
    DiscountPercentage,
    ValidFrom,
    ValidTo,
}