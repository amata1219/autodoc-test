use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

/// アプリケーション設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app: AppSettings,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub http: HttpConfig,
    pub security: SecurityConfig,
    pub logging: LoggingConfig,
    pub machine_learning: MachineLearningConfig,
    pub plugins: PluginConfig,
}

/// アプリケーション基本設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub debug: bool,
    pub host: String,
    pub port: u16,
}

/// データベース設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
}

/// Redis設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
    pub connection_timeout: u64,
    pub read_timeout: u64,
    pub write_timeout: u64,
}

/// HTTP設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    pub cors_origins: Vec<String>,
    pub rate_limit_requests: u32,
    pub rate_limit_window: u64,
    pub request_timeout: u64,
    pub max_body_size: usize,
}

/// セキュリティ設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub jwt_expiration: u64,
    pub bcrypt_cost: u32,
    pub api_key_required: bool,
    pub encryption_enabled: bool,
    pub allowed_ips: Vec<String>,
}

/// ログ設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: String,
    pub file_path: Option<String>,
    pub max_file_size: Option<usize>,
    pub max_files: Option<usize>,
}

/// 機械学習設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineLearningConfig {
    pub model_cache_size: usize,
    pub training_timeout: u64,
    pub inference_timeout: u64,
    pub gpu_enabled: bool,
    pub model_update_interval: u64,
}

/// プラグイン設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub plugin_directory: String,
    pub auto_load: bool,
    pub sandbox_enabled: bool,
    pub max_plugin_memory: usize,
}

impl AppConfig {
    /// 設定ファイルから設定を読み込む
    pub fn load() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            // デフォルト設定
            .add_source(File::from(Path::new("config/default.toml")).required(false))
            // 環境別設定
            .add_source(File::from(Path::new(&format!("config/{}.toml", run_mode))).required(false))
            // ローカル設定（gitignoreされる）
            .add_source(File::from(Path::new("config/local.toml")).required(false))
            // 環境変数
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        config.try_deserialize()
    }

    /// 開発環境用のデフォルト設定を取得
    pub fn development() -> Self {
        Self {
            app: AppSettings {
                name: "AI Agent System".to_string(),
                version: "0.1.0".to_string(),
                environment: "development".to_string(),
                debug: true,
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            database: DatabaseConfig {
                url: "postgresql://localhost/ai_agent_dev".to_string(),
                max_connections: 10,
                min_connections: 2,
                connection_timeout: 30,
                idle_timeout: 300,
                max_lifetime: 3600,
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                pool_size: 5,
                connection_timeout: 5,
                read_timeout: 3,
                write_timeout: 3,
            },
            http: HttpConfig {
                cors_origins: vec!["http://localhost:3000".to_string()],
                rate_limit_requests: 1000,
                rate_limit_window: 3600,
                request_timeout: 30,
                max_body_size: 10 * 1024 * 1024, // 10MB
            },
            security: SecurityConfig {
                jwt_secret: "dev-secret-key-change-in-production".to_string(),
                jwt_expiration: 86400, // 24 hours
                bcrypt_cost: 10,
                api_key_required: false,
                encryption_enabled: false,
                allowed_ips: vec!["127.0.0.1".to_string(), "::1".to_string()],
            },
            logging: LoggingConfig {
                level: "debug".to_string(),
                format: "json".to_string(),
                output: "console".to_string(),
                file_path: Some("logs/app.log".to_string()),
                max_file_size: Some(100 * 1024 * 1024), // 100MB
                max_files: Some(10),
            },
            machine_learning: MachineLearningConfig {
                model_cache_size: 100,
                training_timeout: 3600,
                inference_timeout: 30,
                gpu_enabled: false,
                model_update_interval: 86400,
            },
            plugins: PluginConfig {
                plugin_directory: "plugins".to_string(),
                auto_load: true,
                sandbox_enabled: true,
                max_plugin_memory: 100 * 1024 * 1024, // 100MB
            },
        }
    }

    /// 本番環境用のデフォルト設定を取得
    pub fn production() -> Self {
        Self {
            app: AppSettings {
                name: "AI Agent System".to_string(),
                version: "0.1.0".to_string(),
                environment: "production".to_string(),
                debug: false,
                host: "0.0.0.0".to_string(),
                port: 8080,
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL").unwrap_or_else(|_| "postgresql://localhost/ai_agent_prod".to_string()),
                max_connections: 50,
                min_connections: 10,
                connection_timeout: 30,
                idle_timeout: 300,
                max_lifetime: 3600,
            },
            redis: RedisConfig {
                url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
                pool_size: 20,
                connection_timeout: 5,
                read_timeout: 3,
                write_timeout: 3,
            },
            http: HttpConfig {
                cors_origins: vec![],
                rate_limit_requests: 100,
                rate_limit_window: 3600,
                request_timeout: 30,
                max_body_size: 10 * 1024 * 1024, // 10MB
            },
            security: SecurityConfig {
                jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
                jwt_expiration: 86400, // 24 hours
                bcrypt_cost: 12,
                api_key_required: true,
                encryption_enabled: true,
                allowed_ips: vec![],
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
                output: "file".to_string(),
                file_path: Some("/var/log/ai-agent/app.log".to_string()),
                max_file_size: Some(100 * 1024 * 1024), // 100MB
                max_files: Some(30),
            },
            machine_learning: MachineLearningConfig {
                model_cache_size: 1000,
                training_timeout: 7200,
                inference_timeout: 60,
                gpu_enabled: true,
                model_update_interval: 86400,
            },
            plugins: PluginConfig {
                plugin_directory: "/opt/ai-agent/plugins".to_string(),
                auto_load: false,
                sandbox_enabled: true,
                max_plugin_memory: 500 * 1024 * 1024, // 500MB
            },
        }
    }

    /// テスト環境用のデフォルト設定を取得
    pub fn test() -> Self {
        Self {
            app: AppSettings {
                name: "AI Agent System Test".to_string(),
                version: "0.1.0".to_string(),
                environment: "test".to_string(),
                debug: true,
                host: "127.0.0.1".to_string(),
                port: 0, // ランダムポート
            },
            database: DatabaseConfig {
                url: "sqlite::memory:".to_string(),
                max_connections: 1,
                min_connections: 1,
                connection_timeout: 5,
                idle_timeout: 60,
                max_lifetime: 300,
            },
            redis: RedisConfig {
                url: "redis://localhost:6379/1".to_string(),
                pool_size: 1,
                connection_timeout: 1,
                read_timeout: 1,
                write_timeout: 1,
            },
            http: HttpConfig {
                cors_origins: vec!["http://localhost:3000".to_string()],
                rate_limit_requests: 10000,
                rate_limit_window: 3600,
                request_timeout: 60,
                max_body_size: 100 * 1024 * 1024, // 100MB
            },
            security: SecurityConfig {
                jwt_secret: "test-secret-key".to_string(),
                jwt_expiration: 3600, // 1 hour
                bcrypt_cost: 4,
                api_key_required: false,
                encryption_enabled: false,
                allowed_ips: vec!["127.0.0.1".to_string()],
            },
            logging: LoggingConfig {
                level: "debug".to_string(),
                format: "text".to_string(),
                output: "console".to_string(),
                file_path: None,
                max_file_size: None,
                max_files: None,
            },
            machine_learning: MachineLearningConfig {
                model_cache_size: 10,
                training_timeout: 300,
                inference_timeout: 10,
                gpu_enabled: false,
                model_update_interval: 3600,
            },
            plugins: PluginConfig {
                plugin_directory: "test-plugins".to_string(),
                auto_load: false,
                sandbox_enabled: true,
                max_plugin_memory: 10 * 1024 * 1024, // 10MB
            },
        }
    }

    /// 環境に応じた設定を取得
    pub fn for_environment(env: &str) -> Result<Self, ConfigError> {
        match env {
            "development" => Ok(Self::development()),
            "production" => Ok(Self::production()),
            "test" => Ok(Self::test()),
            _ => Self::load(),
        }
    }

    /// 設定が有効かどうかを検証
    pub fn validate(&self) -> Result<(), String> {
        if self.app.name.is_empty() {
            return Err("App name cannot be empty".to_string());
        }

        if self.app.port == 0 {
            return Err("App port cannot be 0".to_string());
        }

        if self.database.url.is_empty() {
            return Err("Database URL cannot be empty".to_string());
        }

        if self.redis.url.is_empty() {
            return Err("Redis URL cannot be empty".to_string());
        }

        if self.security.jwt_secret.is_empty() {
            return Err("JWT secret cannot be empty".to_string());
        }

        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::development()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_development_config() {
        let config = AppConfig::development();
        assert_eq!(config.app.environment, "development");
        assert_eq!(config.app.debug, true);
        assert_eq!(config.app.port, 8080);
    }

    #[test]
    fn test_production_config() {
        let config = AppConfig::production();
        assert_eq!(config.app.environment, "production");
        assert_eq!(config.app.debug, false);
        assert_eq!(config.app.port, 8080);
    }

    #[test]
    fn test_test_config() {
        let config = AppConfig::test();
        assert_eq!(config.app.environment, "test");
        assert_eq!(config.app.debug, true);
        assert_eq!(config.app.port, 0);
    }

    #[test]
    fn test_config_validation() {
        let config = AppConfig::development();
        assert!(config.validate().is_ok());
    }
}
