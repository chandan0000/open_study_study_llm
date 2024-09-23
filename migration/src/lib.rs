pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users_table;
mod m20240921_201000_Token_Verifcation;
mod m20240922_122042_passwordresettokens;
 
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users_table::Migration),
            Box::new(m20240921_201000_Token_Verifcation::Migration),
            Box::new(m20240922_122042_passwordresettokens::Migration),
        ]
    }
}
