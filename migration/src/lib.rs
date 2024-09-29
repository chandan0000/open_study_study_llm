pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users_table;
mod m20240921_201000_Token_Verifcation;
mod m20240922_122042_passwordresettokens;
mod m20240928_182418_categories_table;
mod m20240928_182418_products_table;
mod m20240928_182418_orders_table;
mod m20240928_182418_cart_table;
mod m20240928_182418_reviews_table;
mod m20240928_182418_discounts_table;
mod m20240928_182418_create_audit_logs_table;
 
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users_table::Migration),
            Box::new(m20240921_201000_Token_Verifcation::Migration),
            Box::new(m20240922_122042_passwordresettokens::Migration),
            Box::new(m20240928_182418_categories_table::Migration),
            Box::new(m20240928_182418_products_table::Migration),
            Box::new(m20240928_182418_orders_table::Migration),
            Box::new(m20240928_182418_cart_table::Migration),
            Box::new(m20240928_182418_reviews_table::Migration),
            Box::new(m20240928_182418_discounts_table::Migration),
            Box::new(m20240928_182418_create_audit_logs_table::Migration),
        ]
    }
}
