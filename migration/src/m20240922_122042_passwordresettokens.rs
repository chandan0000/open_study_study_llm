use crate::m20220101_000001_users_table::Users;
use sea_orm_migration::{prelude::*, schema::pk_auto};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Write your migration here
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Password Reset Tokens table
        manager
            .create_table(
                Table::create()
                    .table(PasswordResetTokens::Table)
                    .if_not_exists()
                    .col(pk_auto(PasswordResetTokens::Id))
                    .col(
                        ColumnDef::new(PasswordResetTokens::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PasswordResetTokens::Token)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PasswordResetTokens::ExpiresAt)
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
                    .from(PasswordResetTokens::Table, PasswordResetTokens::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback the migration
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the table on rollback
        manager
            .drop_table(Table::drop().table(PasswordResetTokens::Table).to_owned())
            .await
    }
}

// Define the columns of the 'PasswordResetTokens' table
#[derive(Iden)]
pub enum PasswordResetTokens {
    Table,
    Id,
    UserId,
    Token,
    ExpiresAt,
}
