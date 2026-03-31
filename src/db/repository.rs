use sea_orm::{Database, DatabaseConnection, ConnectionTrait};
use anyhow::Result;
use std::path::Path;
use std::fs;

pub struct Repository {
    db: DatabaseConnection,
}

impl Repository {
    pub async fn new(database_path: &str) -> Result<Self> {
        let db_dir = Path::new(database_path).parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid database path"))?;

        if !db_dir.exists() {
            fs::create_dir_all(db_dir)?;
        }

        let db_url = format!("sqlite://{}?mode=rwc", database_path);
        let db = Database::connect(&db_url).await?;

        let repo = Self { db };
        repo.create_tables().await?;

        Ok(repo)
    }

    async fn create_tables(&self) -> Result<()> {
        let statements = vec![
            r#"CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                purpose TEXT NOT NULL DEFAULT 'Default',
                state TEXT NOT NULL DEFAULT 'Idle',
                agent TEXT NOT NULL DEFAULT 'build',
                provider_name TEXT,
                created_at TEXT NOT NULL,
                started_at TEXT,
                finished_at TEXT,
                system_prompt TEXT DEFAULT '',
                parent_id TEXT,
                metadata TEXT DEFAULT '{}',
                input_tokens INTEGER DEFAULT 0,
                output_tokens INTEGER DEFAULT 0,
                has_compacted_history BOOLEAN DEFAULT 0,
                compacted_message_count INTEGER DEFAULT 0
            )"#,
            r#"CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY NOT NULL,
                session_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                tool_calls TEXT,
                metadata TEXT DEFAULT '{}',
                tool_name TEXT,
                tool_call_id TEXT,
                tool_success BOOLEAN
            )"#,
            r#"CREATE TABLE IF NOT EXISTS processes (
                id TEXT PRIMARY KEY NOT NULL,
                command TEXT NOT NULL,
                cwd TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'Running',
                exit_code INTEGER,
                started_at TEXT NOT NULL,
                finished_at TEXT,
                parent_session_id TEXT,
                unit_name TEXT NOT NULL,
                timeout_ms INTEGER DEFAULT 120000,
                output_dir TEXT NOT NULL,
                stdout_size INTEGER DEFAULT 0,
                stderr_size INTEGER DEFAULT 0,
                stdout_lines INTEGER DEFAULT 0,
                stderr_lines INTEGER DEFAULT 0
            )"#,
            r#"CREATE TABLE IF NOT EXISTS todos (
                id TEXT PRIMARY KEY NOT NULL,
                session_id TEXT NOT NULL,
                content TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                priority TEXT NOT NULL DEFAULT 'medium',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )"#,
            r#"CREATE TABLE IF NOT EXISTS providers (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL UNIQUE,
                base_url TEXT NOT NULL,
                api_key TEXT NOT NULL,
                model TEXT NOT NULL,
                max_tokens INTEGER DEFAULT 4096,
                temperature REAL DEFAULT 0.7,
                top_p REAL DEFAULT 1.0,
                top_k INTEGER DEFAULT 40,
                extra_params TEXT,
                is_default BOOLEAN DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                auto_compact_length INTEGER DEFAULT 20000
            )"#,
            r#"CREATE TABLE IF NOT EXISTS external_tools (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL UNIQUE,
                description TEXT NOT NULL,
                parameters_json TEXT NOT NULL,
                config_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )"#,
            r#"CREATE TABLE IF NOT EXISTS tool_variables (
                id TEXT PRIMARY KEY NOT NULL,
                tool_name TEXT NOT NULL,
                variable_name TEXT NOT NULL,
                value TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )"#,
            r#"CREATE TABLE IF NOT EXISTS file_reads (
                id TEXT PRIMARY KEY NOT NULL,
                session_id TEXT NOT NULL,
                file_path TEXT NOT NULL,
                modified_at TEXT NOT NULL,
                read_at TEXT NOT NULL
            )"#,
            r#"CREATE TABLE IF NOT EXISTS skill_registries (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL UNIQUE,
                url TEXT NOT NULL,
                api_key TEXT,
                created_at TEXT NOT NULL
            )"#,
            r#"CREATE TABLE IF NOT EXISTS skills (
                id TEXT PRIMARY KEY NOT NULL,
                registry_id TEXT NOT NULL,
                name TEXT NOT NULL,
                full_name TEXT NOT NULL UNIQUE,
                description TEXT NOT NULL,
                folder_path TEXT NOT NULL,
                installed_at TEXT NOT NULL
            )"#,
            r#"CREATE TABLE IF NOT EXISTS timers (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                message_template TEXT,
                timer_type TEXT NOT NULL,
                timer_spec TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'active',
                session_id TEXT,
                max_runs INTEGER DEFAULT 1,
                run_count INTEGER DEFAULT 0,
                last_run_at TEXT,
                next_run_at TEXT,
                created_at TEXT NOT NULL
            )"#,
        ];

        for stmt in statements {
            self.db.query_all(sea_orm::Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                stmt.to_string(),
            )).await?;
        }

        Ok(())
    }

    pub fn connection(&self) -> &DatabaseConnection {
        &self.db
    }
}