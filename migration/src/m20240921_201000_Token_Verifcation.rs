use crate::m20220101_000001_users_table::Users;
use sea_orm_migration::{prelude::*, schema::pk_auto,  };
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the "Post" table
        manager
            .create_table(
                
                Table::create()
                    .table(TokenVerifcation::Table)
                    .if_not_exists()
                    .col(pk_auto(TokenVerifcation::Id))
                    .col(
                        ColumnDef::new(TokenVerifcation::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TokenVerifcation::Token)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TokenVerifcation::ExpiresAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add a foreign key constraint for Post::UserId referencing Users::Id
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(TokenVerifcation::Table, TokenVerifcation::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the foreign key first
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(TokenVerifcation::Table)
                    .name("fk_post_userid")
                    .to_owned(),
            )
            .await?;

        // Drop the "Post" table
        manager
            .drop_table(Table::drop().table(TokenVerifcation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TokenVerifcation {
    Table,
    Id,
    UserId,
    Token,
    ExpiresAt,
}
