use crate::m20220101_000001_users_table::Users;
use sea_orm_migration::{prelude::*, schema::*};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Cart::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Cart::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Cart::UserId)
                            .integer()
                            .not_null()
                            )
                            .foreign_key(
                                ForeignKey::create()
                                    .name("fk-cart-user_id")
                                    .from(Cart::Table, Cart::UserId)
                                    .to(Users::Table, Users::Id)
                                    .on_delete(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Cart::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Cart::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Cart {
    Table,
    Id,
    UserId,
    CreatedAt,
}
