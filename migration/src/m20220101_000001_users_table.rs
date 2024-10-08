use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{ConnectionTrait, DbBackend};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221007_000001_create_users_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Apply the migration
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Step 1: Create an ENUM type for the 'role' field
        manager
            .get_connection()
            .execute_unprepared(r#"CREATE TYPE "user_role" AS ENUM ('admin', 'user');"#)
            .await?;
        manager
            .get_connection()
            .execute_unprepared(r#"CREATE EXTENSION IF NOT EXISTS "uuid-ossp";"#)
            .await?;

        // Step 2: Create the 'users' table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Uuid)
                            .uuid() // Change from integer to UUID type
                            .not_null()
                            .default(Expr::cust("uuid_generate_v4()")), // Set default to auto-generate UUID
                    )
                    .col(
                        ColumnDef::new(Users::Role)
                            .custom(Alias::new("user_role"))
                            .not_null()
                            .default(Expr::cust("'user'::user_role")), // Set default role to 'user'
                    )
                    .col(
                        ColumnDef::new(Users::Fullname)
                            .string()
                            .not_null()
                            .string_len(255),
                    )
                    .col(
                        ColumnDef::new(Users::EmailId)
                            .string()
                            .not_null()
                            .unique_key()
                            .string_len(255),
                    )
                    .col(
                        ColumnDef::new(Users::Password)
                            .string()
                            .not_null()
                            .string_len(255),
                    )
                    .col(ColumnDef::new(Users::ProfilePic).string().string_len(255))
                    .col(ColumnDef::new(Users::GithubLink).string().string_len(255))
                    .col(ColumnDef::new(Users::LinkdinLink).string().string_len(255))
                    .col(
                        ColumnDef::new(Users::IsVerdified)
                            .boolean()
                            .default(Expr::value(false)),
                    )
                    .col(ColumnDef::new(Users::DeleteAccountDate).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Users::UpdateDate)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::CreateDate)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-users-email")
                    .table(Users::Table)
                    .col(Users::EmailId)
                    .unique()
                    .to_owned(),
            )
            .await?;
        // Step 3: Create a function to update 'update_date' on record modification
        let create_function = match manager.get_database_backend() {
            DbBackend::Postgres => {
                r#"
                CREATE OR REPLACE FUNCTION update_modified_column()
                RETURNS TRIGGER AS $$
                BEGIN
                   NEW.update_date = NOW();
                   RETURN NEW;
                END;
                $$ LANGUAGE plpgsql;
                "#
            }
            _ => return Err(DbErr::Migration("Unsupported database backend".to_owned())),
        };
        manager
            .get_connection()
            .execute_unprepared(create_function)
            .await?;

        // Step 4: Create a trigger that calls the function before each update
        let create_trigger = r#"
            CREATE TRIGGER update_users_modtime
            BEFORE UPDATE ON users
            FOR EACH ROW
            EXECUTE PROCEDURE update_modified_column();
        "#;
        manager
            .get_connection()
            .execute_unprepared(create_trigger)
            .await?;

        Ok(())
    }

    // Revert the migration
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx-users-email")
                    .table(Users::Table)
                    .to_owned(),
            )
            .await?;

        // Step 4: Drop the trigger
        manager
            .get_connection()
            .execute_unprepared(r#"DROP TRIGGER IF EXISTS update_users_modtime ON users;"#)
            .await?;

        // Step 3: Drop the function
        manager
            .get_connection()
            .execute_unprepared(r#"DROP FUNCTION IF EXISTS update_modified_column;"#)
            .await?;

        // Step 2: Drop the 'users' table
        manager
            .drop_table(Table::drop().table(Users::Table).if_exists().to_owned())
            .await?;

        // Step 1: Drop the ENUM type
        manager
            .get_connection()
            .execute_unprepared(r#"DROP TYPE IF EXISTS "user_role";"#)
            .await?;
        manager
            .get_connection()
            .execute_unprepared(r#"DROP EXTENSION IF EXISTS "uuid-ossp";"#)
            .await?;

        Ok(())
    }
}

// Define the columns of the 'users' table
#[derive(Iden)]
pub  enum Users {
    Table,
    Id,

    Uuid,
    Role,
    Fullname,
    EmailId,
    Password,
    ProfilePic,
    GithubLink,
    LinkdinLink,
    IsVerdified,
    DeleteAccountDate,
    UpdateDate,
    CreateDate,
}
