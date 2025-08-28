use thiserror::Error;
use std::fmt;

/// アプリケーション全体で使用するエラー型
#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("External service error: {0}")]
    ExternalServiceError(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Resource unavailable: {0}")]
    ResourceUnavailable(String),

    #[error("Plugin error: {0}")]
    PluginError(String),

    #[error("Machine learning error: {0}")]
    MachineLearningError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("File I/O error: {0}")]
    FileIOError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Business logic error: {0}")]
    BusinessLogicError(String),
}

impl Error {
    /// エラーがクライアントエラー（4xx）かどうかを判定
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            Error::ValidationError(_)
                | Error::AuthenticationError(_)
                | Error::AuthorizationError(_)
                | Error::NotFound(_)
                | Error::Conflict(_)
                | Error::RateLimitExceeded(_)
                | Error::InvalidInput(_)
                | Error::ResourceUnavailable(_)
        )
    }

    /// エラーがサーバーエラー（5xx）かどうかを判定
    pub fn is_server_error(&self) -> bool {
        matches!(
            self,
            Error::DatabaseError(_)
                | Error::RedisError(_)
                | Error::InternalServerError(_)
                | Error::ExternalServiceError(_)
                | Error::FileIOError(_)
                | Error::EncryptionError(_)
                | Error::DecryptionError(_)
                | Error::MachineLearningError(_)
                | Error::NetworkError(_)
        )
    }

    /// エラーのHTTPステータスコードを取得
    pub fn http_status_code(&self) -> u16 {
        match self {
            Error::ValidationError(_) => 400,
            Error::AuthenticationError(_) => 401,
            Error::AuthorizationError(_) => 403,
            Error::NotFound(_) => 404,
            Error::Conflict(_) => 409,
            Error::RateLimitExceeded(_) => 429,
            Error::InvalidInput(_) => 400,
            Error::ResourceUnavailable(_) => 503,
            Error::Timeout(_) => 408,
            Error::PluginError(_) => 500,
            Error::MachineLearningError(_) => 500,
            Error::NetworkError(_) => 503,
            Error::FileIOError(_) => 500,
            Error::ParseError(_) => 400,
            Error::EncryptionError(_) => 500,
            Error::DecryptionError(_) => 500,
            Error::ApiError(_) => 500,
            Error::BusinessLogicError(_) => 400,
            Error::DatabaseError(_) => 500,
            Error::RedisError(_) => 500,
            Error::SerializationError(_) => 500,
            Error::ConfigurationError(_) => 500,
            Error::ExternalServiceError(_) => 502,
            Error::InternalServerError(_) => 500,
        }
    }

    /// エラーメッセージを取得
    pub fn error_message(&self) -> String {
        match self {
            Error::ValidationError(msg) => format!("Validation failed: {}", msg),
            Error::AuthenticationError(msg) => format!("Authentication failed: {}", msg),
            Error::AuthorizationError(msg) => format!("Authorization failed: {}", msg),
            Error::NotFound(msg) => format!("Resource not found: {}", msg),
            Error::Conflict(msg) => format!("Conflict occurred: {}", msg),
            Error::RateLimitExceeded(msg) => format!("Rate limit exceeded: {}", msg),
            Error::InvalidInput(msg) => format!("Invalid input: {}", msg),
            Error::ResourceUnavailable(msg) => format!("Resource unavailable: {}", msg),
            Error::Timeout(msg) => format!("Operation timed out: {}", msg),
            Error::PluginError(msg) => format!("Plugin error: {}", msg),
            Error::MachineLearningError(msg) => format!("Machine learning error: {}", msg),
            Error::NetworkError(msg) => format!("Network error: {}", msg),
            Error::FileIOError(err) => format!("File I/O error: {}", err),
            Error::ParseError(msg) => format!("Parse error: {}", msg),
            Error::EncryptionError(msg) => format!("Encryption error: {}", msg),
            Error::DecryptionError(msg) => format!("Decryption error: {}", msg),
            Error::ApiError(msg) => format!("API error: {}", msg),
            Error::BusinessLogicError(msg) => format!("Business logic error: {}", msg),
            Error::DatabaseError(err) => format!("Database error: {}", err),
            Error::RedisError(err) => format!("Redis error: {}", err),
            Error::SerializationError(err) => format!("Serialization error: {}", err),
            Error::ConfigurationError(msg) => format!("Configuration error: {}", msg),
            Error::ExternalServiceError(msg) => format!("External service error: {}", msg),
            Error::InternalServerError(msg) => format!("Internal server error: {}", msg),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_message())
    }
}

/// 結果型のエイリアス
pub type Result<T> = std::result::Result<T, Error>;

/// エラーをログに記録する
pub fn log_error(error: &Error, context: &str) {
    use tracing::error;
    
    if error.is_server_error() {
        error!(error = %error, context = context, "Server error occurred");
    } else if error.is_client_error() {
        error!(error = %error, context = context, "Client error occurred");
    } else {
        error!(error = %error, context = context, "Unknown error occurred");
    }
}

/// エラーをユーザーフレンドリーなメッセージに変換する
pub fn user_friendly_message(error: &Error) -> String {
    match error {
        Error::ValidationError(_) => "入力データが正しくありません。".to_string(),
        Error::AuthenticationError(_) => "認証に失敗しました。".to_string(),
        Error::AuthorizationError(_) => "アクセス権限がありません。".to_string(),
        Error::NotFound(_) => "リソースが見つかりません。".to_string(),
        Error::Conflict(_) => "リソースが競合しています。".to_string(),
        Error::RateLimitExceeded(_) => "リクエスト制限を超えました。しばらく待ってから再試行してください。".to_string(),
        Error::InvalidInput(_) => "入力データが正しくありません。".to_string(),
        Error::ResourceUnavailable(_) => "リソースが利用できません。".to_string(),
        Error::Timeout(_) => "操作がタイムアウトしました。".to_string(),
        Error::PluginError(_) => "プラグインでエラーが発生しました。".to_string(),
        Error::MachineLearningError(_) => "機械学習処理でエラーが発生しました。".to_string(),
        Error::NetworkError(_) => "ネットワークエラーが発生しました。".to_string(),
        Error::FileIOError(_) => "ファイル操作でエラーが発生しました。".to_string(),
        Error::ParseError(_) => "データの解析でエラーが発生しました。".to_string(),
        Error::EncryptionError(_) => "暗号化処理でエラーが発生しました。".to_string(),
        Error::DecryptionError(_) => "復号化処理でエラーが発生しました。".to_string(),
        Error::ApiError(_) => "APIでエラーが発生しました。".to_string(),
        Error::BusinessLogicError(_) => "ビジネスロジックでエラーが発生しました。".to_string(),
        Error::DatabaseError(_) => "データベースでエラーが発生しました。".to_string(),
        Error::RedisError(_) => "キャッシュでエラーが発生しました。".to_string(),
        Error::SerializationError(_) => "データの変換でエラーが発生しました。".to_string(),
        Error::ConfigurationError(_) => "設定でエラーが発生しました。".to_string(),
        Error::ExternalServiceError(_) => "外部サービスでエラーが発生しました。".to_string(),
        Error::InternalServerError(_) => "サーバーでエラーが発生しました。".to_string(),
    }
}
