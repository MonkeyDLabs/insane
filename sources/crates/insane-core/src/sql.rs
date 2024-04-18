use lazy_static::lazy_static;
use regex::Regex;
use sea_orm::{
    ActiveModelTrait, ConnectOptions, ConnectionTrait, Database, DatabaseBackend,
    DatabaseConnection, EntityTrait, IntoActiveModel, Statement,
};
use std::{fs::File, path::Path, time::Duration};

use sea_orm_migration::MigratorTrait;

use super::error::Error;
use super::error::Result as InsaneResult;
use crate::config::SqlConfig;
use crate::hook::Hooks;

lazy_static! {
  // Getting the table name from the environment configuration.
  // For example:
  // postgres://loco:loco@localhost:5432/loco_app
  // mysql://loco:loco@localhost:3306/loco_app
  // the results will be loco_app
  pub static ref EXTRACT_DB_NAME: Regex = Regex::new(r"/([^/]+)$").unwrap();
}

///  Create a new database. This functionality is currently exclusive to Postgre
/// databases.
///
/// # Errors
///
/// Returns a [`sea_orm::DbErr`] if an error occurs during run migration up.
pub async fn create(db_uri: &str) -> InsaneResult<()> {
    if !db_uri.starts_with("postgres://") {
        return Err(Error::string(
            "Only Postgres databases are supported for table creation",
        ));
    }
    let db_name = EXTRACT_DB_NAME
        .captures(db_uri)
        .and_then(|cap| cap.get(1).map(|db| db.as_str()))
        .ok_or_else(|| {
            Error::string(
                "The specified table name was not found in the given Postgre database URI",
            )
        })?;

    let conn = EXTRACT_DB_NAME.replace(db_uri, "/postgres").to_string();
    let db = Database::connect(conn).await?;

    Ok(create_postgres_database(db_name, &db).await?)
}

/// Apply migrations to the database using the provided migrator.
///
/// # Errors
///
/// Returns a [`sea_orm::DbErr`] if an error occurs during run migration up.
pub async fn migrate<M: MigratorTrait>(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    M::up(db, None).await
}

/// Rollback the database by n steps.
///
/// # Errors
///
/// Returns a [`sea_orm::DbErr`] if an error occurs during run migration up.
pub async fn migrate_down<M: MigratorTrait>(
    db: &DatabaseConnection,
    steps: Option<u32>,
) -> Result<(), sea_orm::DbErr> {
    M::down(db, steps).await
}

/// Check the migration status of the database.
///
/// # Errors
///
/// Returns a [`sea_orm::DbErr`] if an error occurs during checking status
pub async fn status<M: MigratorTrait>(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    M::status(db).await
}

/// Reset the database, dropping and recreating the schema and applying
/// migrations.
///
/// # Errors
///
/// Returns a [`sea_orm::DbErr`] if an error occurs during reset databases.
pub async fn reset<M: MigratorTrait>(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    M::fresh(db).await?;
    migrate::<M>(db).await
}

/// Seed the database with data from a specified file.
/// Seeds open the file path and insert all file content into the DB.
///
/// The file content should be equal to the DB field parameters.
///
/// # Errors
///
/// Returns a [`InsaneResult`] if could not render the path content into
/// [`Vec<serde_json::Value>`] or could not inset the vector to DB.
#[allow(clippy::type_repetition_in_bounds)]
pub async fn seed<A>(db: &DatabaseConnection, path: &str) -> InsaneResult<()>
where
    <<A as ActiveModelTrait>::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    for<'de> <<A as ActiveModelTrait>::Entity as EntityTrait>::Model: serde::de::Deserialize<'de>,
    A: sea_orm::ActiveModelTrait + Send + Sync,
    sea_orm::Insert<A>: Send + Sync, // Add this Send bound
{
    let loader: Vec<serde_json::Value> = serde_yaml::from_reader(File::open(path)?)?;

    let mut users: Vec<A> = vec![];
    for user in loader {
        users.push(A::from_json(user)?);
    }

    <A as ActiveModelTrait>::Entity::insert_many(users)
        .exec(db)
        .await?;

    Ok(())
}

/// Truncate a table in the database, effectively deleting all rows.
///
/// # Errors
///
/// Returns a [`InsaneResult`] if an error occurs during truncate the given table
pub async fn truncate_table<T>(db: &DatabaseConnection, _: T) -> Result<(), sea_orm::DbErr>
where
    T: EntityTrait,
{
    T::delete_many().exec(db).await?;
    Ok(())
}

/// Execute seed from the given path
///
/// # Errors
///
/// when seed process is fails
pub async fn run_app_seed<H: Hooks>(db: &DatabaseConnection, path: &Path) -> InsaneResult<()> {
    H::seed(db, path).await
}

/// Create a Postgres table from the given table name.
///
/// To create the table with `LOCO_POSTGRES_TABLE_OPTIONS`
async fn create_postgres_database(
    table_name: &str,
    db: &DatabaseConnection,
) -> Result<(), sea_orm::DbErr> {
    let with_options = std::env::var("LOCO_POSTGRES_TABLE_OPTIONS")
        .map_or_else(|_| "ENCODING='UTF8'".to_string(), |options| options);

    let query = format!("CREATE DATABASE {table_name} WITH {with_options}");
    tracing::info!(query, "creating postgres table");

    db.execute(sea_orm::Statement::from_string(
        sea_orm::DatabaseBackend::Postgres,
        query,
    ))
    .await?;
    Ok(())
}

pub async fn connect(config: &SqlConfig) -> InsaneResult<sea_orm::DatabaseConnection> {
    let application_url = config.uri.to_string();

    let mut application_options = ConnectOptions::new(application_url);

    if let Some(value) = config.max_connections {
        application_options.max_connections(value);
    }
    if let Some(value) = config.min_connections {
        application_options.min_connections(value);
    }
    if let Some(value) = config.connect_timeout {
        application_options.connect_timeout(Duration::from_millis(value));
    }
    if let Some(value) = config.idle_timeout {
        application_options.idle_timeout(Duration::from_millis(value));
    }
    if let Some(value) = config.max_lifetime {
        application_options.max_lifetime(Duration::from_millis(value));
    }
    application_options.sqlx_logging(config.enable_logging.unwrap_or(false));

    Ok(Database::connect(application_options).await?)
}

/// Verifies a user has access to data within its database
///
/// # Errors
///
/// This function will return an error if IO fails
#[allow(clippy::match_wildcard_for_single_variants)]
pub async fn verify_access(db: &DatabaseConnection) -> InsaneResult<()> {
    match db {
        DatabaseConnection::SqlxPostgresPoolConnection(_) => {
            let res = db
                .query_all(Statement::from_string(
                    DatabaseBackend::Postgres,
                    "SELECT * FROM pg_catalog.pg_tables WHERE tableowner = current_user;",
                ))
                .await?;
            if res.is_empty() {
                return Err(Error::string(
                    "current user has no access to tables in the database",
                ));
            }
        }
        DatabaseConnection::Disconnected => {
            return Err(Error::string("connection to database has been closed"));
        }
        _ => {}
    }
    Ok(())
}

/// converge database logic
///
/// # Errors
///
///  an `AppResult`, which is an alias for `Result<(), AppError>`. It may
/// return an `AppError` variant representing different database operation
/// failures.
pub async fn prepare<H: Hooks, M: MigratorTrait>(
  db: &DatabaseConnection,
  config: &SqlConfig,
) -> InsaneResult<()> {
  if config.dangerously_recreate.unwrap_or(false) {
      tracing::info!("recreating schema");
      reset::<M>(db).await?;
      return Ok(());
  }

  if config.auto_migrate.unwrap_or(false) {
    tracing::info!("auto migrating");
      migrate::<M>(db).await?;
  }

  if config.dangerously_truncate.unwrap_or(false) {
    tracing::info!("truncating tables");
      H::truncate(db).await?;
  }
  Ok(())
}
