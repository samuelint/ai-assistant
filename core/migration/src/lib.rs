use sea_orm::DatabaseConnection;
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_configuration_table;
mod m20220101_000002_create_thread_table;
mod m20220101_000003_create_run_table;
mod m20220101_000004_create_message_table;
mod m20220101_000005_seed_default_llm_configuration;
mod m20220101_000006_create_profile_table;
mod m20220101_000007_seed_profile_software_engineer;

pub use m20220101_000007_seed_profile_software_engineer::SOFTWARE_ENGINEER_PROFILE_NAME;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_configuration_table::Migration),
            Box::new(m20220101_000002_create_thread_table::Migration),
            Box::new(m20220101_000003_create_run_table::Migration),
            Box::new(m20220101_000004_create_message_table::Migration),
            Box::new(m20220101_000005_seed_default_llm_configuration::Migration),
            Box::new(m20220101_000006_create_profile_table::Migration),
            Box::new(m20220101_000007_seed_profile_software_engineer::Migration),
        ]
    }
}

pub async fn migrate_database(db: &DatabaseConnection) -> Result<(), DbErr> {
    let schema_manager = SchemaManager::new(db);

    Migrator::up(db, None).await?;
    assert!(schema_manager.has_table("configuration").await?);

    Ok(())
}
