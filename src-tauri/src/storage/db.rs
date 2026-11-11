//! SQLite database connection management, configuration pragmas, and backup operations.
//! Complies with docs/business/be-core.md (§3.1, §3.4, §3.5) and docs/db.dbml.

use rusqlite::Connection;
use std::path::{Path, PathBuf};

/// Canonical error codes for storage and migration operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCode {
    MigrationFailed,
    UnsupportedSchemaVersion,
    ContractBlocked,
    ValidationFailed,
}

/// Rich database error type supporting contract requirements.
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Migration failed: {0}")]
    MigrationFailed(String),

    #[error("Unsupported schema version: {current} exceeds max supported {max_supported}")]
    UnsupportedSchemaVersion { current: u32, max_supported: u32 },

    #[error("Contract blocked: {0}")]
    ContractBlocked(String),

    #[error("Database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Integrity check failed: {0}")]
    IntegrityCheckFailed(String),

    #[error("Foreign key check failed: {0} violations found")]
    ForeignKeyViolation(usize),

    #[error("Schema verification failed: {0}")]
    SchemaVerificationFailed(String),
}

impl DbError {
    pub fn error_code(&self) -> ErrorCode {
        match self {
            DbError::MigrationFailed(_)
            | DbError::IntegrityCheckFailed(_)
            | DbError::ForeignKeyViolation(_)
            | DbError::SchemaVerificationFailed(_) => ErrorCode::MigrationFailed,
            DbError::UnsupportedSchemaVersion { .. } => ErrorCode::UnsupportedSchemaVersion,
            DbError::ContractBlocked(_) => ErrorCode::ContractBlocked,
            _ => ErrorCode::MigrationFailed,
        }
    }
}

/// Apply mandatory connection PRAGMAs (§3.1) to every opened SQLite connection.
///
/// MUST execute:
/// - PRAGMA journal_mode = WAL;
/// - PRAGMA synchronous = FULL;
/// - PRAGMA foreign_keys = ON;
/// - PRAGMA busy_timeout = 5000;
/// - PRAGMA temp_store = MEMORY;
pub fn configure_connection(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA synchronous = FULL;
         PRAGMA foreign_keys = ON;
         PRAGMA busy_timeout = 5000;
         PRAGMA temp_store = MEMORY;",
    )?;
    Ok(())
}

/// Directory path for canonical database location (§1.6, DB-P01).
pub fn canonical_db_dir(app_data_dir: &Path) -> PathBuf {
    if app_data_dir.ends_with(".merlink") {
        app_data_dir.join("db")
    } else {
        app_data_dir.join("merlink-harness").join("db")
    }
}

/// Canonical file path for the primary SQLite database (§1.6, DB-P01).
/// Resolves to `<app_data_dir>/merlink-harness/db/merlink.sqlite3`.
pub fn canonical_db_path(app_data_dir: &Path) -> PathBuf {
    canonical_db_dir(app_data_dir).join("merlink.sqlite3")
}

/// Directory path for database backup files (§3.4, DB-P02).
pub fn backup_dir(app_data_dir: &Path) -> PathBuf {
    if app_data_dir.ends_with(".merlink") {
        app_data_dir.join("backups")
    } else {
        app_data_dir.join("merlink-harness").join("backups")
    }
}

/// Canonical path for pre-migration backup files (§3.4, DB-P02).
/// Format: `<app_data_dir>/merlink-harness/backups/merlink_pre_migration_v{old}_{timestamp}.bak`.
pub fn backup_path(app_data_dir: &Path, old_version: u32, timestamp: &str) -> PathBuf {
    backup_dir(app_data_dir).join(format!(
        "merlink_pre_migration_v{}_{}.bak",
        old_version, timestamp
    ))
}

/// Perform physical database backup using SQLite Online Backup API (`rusqlite::backup::Backup`).
/// Copies source database to `dst_path` without using VACUUM INTO.
pub fn perform_online_backup(src_conn: &Connection, dst_path: &Path) -> Result<(), DbError> {
    if let Some(parent) = dst_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut dst_conn = Connection::open(dst_path)?;
    let backup = rusqlite::backup::Backup::new(src_conn, &mut dst_conn)?;
    backup.run_to_completion(100, std::time::Duration::from_millis(5), None)?;
    Ok(())
}

/// SQLite Connection Pool type based on r2d2 and r2d2_sqlite.
pub type DbPool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

/// Database Manager managing SQLite connection pools and migration bootstrapping.
pub struct DbManager {
    db_path: PathBuf,
    pool: DbPool,
}

impl DbManager {
    /// Open an existing database or create pool with connection PRAGMAs configured.
    pub fn open(db_path: impl Into<PathBuf>) -> Result<Self, DbError> {
        let db_path = db_path.into();
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let manager = r2d2_sqlite::SqliteConnectionManager::file(&db_path)
            .with_init(|conn| configure_connection(conn));
        let pool = r2d2::Pool::builder()
            .max_size(8)
            .build(manager)
            .map_err(|e| {
                DbError::MigrationFailed(format!("Failed to build connection pool: {}", e))
            })?;
        Ok(Self { db_path, pool })
    }

    /// Primary bootstrap and migration entry point (REQ-SPECTEST-DB-P01, §3.4).
    /// Opens the single authoritative database path `<app_data_dir>/merlink-harness/db/merlink.sqlite3`,
    /// runs migrations, executes schema & integrity checks, and returns a configured DbManager.
    pub fn initialize_and_migrate(app_data_dir: &Path) -> Result<Self, DbError> {
        let db_path = canonical_db_path(app_data_dir);
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Open single write connection for migration sequence
        let mut conn = Connection::open(&db_path)?;
        configure_connection(&conn)?;

        // Execute 7-step migration workflow with pre-migration backup if applicable
        super::migration::run_migrations_with_backup(&mut conn, app_data_dir)?;

        // Post-migration integrity & schema verification (§3.5)
        super::migration::verify_schema_and_integrity(&conn)?;

        // Return connection pool manager
        Self::open(db_path)
    }

    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    pub fn get_connection(
        &self,
    ) -> Result<r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>, DbError> {
        self.pool
            .get()
            .map_err(|e| DbError::MigrationFailed(format!("Pool checkout error: {}", e)))
    }
}
