use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// AIエージェントのコアエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: AgentId,
    pub name: String,
    pub description: String,
    pub agent_type: AgentType,
    pub status: AgentStatus,
    pub capabilities: Vec<Capability>,
    pub configuration: AgentConfiguration,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// エージェントID（バリューオブジェクト）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub Uuid);

impl AgentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AgentId {
    fn default() -> Self {
        Self::new()
    }
}

/// エージェントタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentType {
    Conversational,
    TaskExecutor,
    Learning,
    Monitoring,
    Orchestrator,
    Custom(String),
}

/// エージェントステータス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Active,
    Inactive,
    Training,
    Error,
    Maintenance,
}

/// エージェントの能力
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub description: String,
    pub version: String,
    pub parameters: HashMap<String, String>,
}

/// エージェント設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfiguration {
    pub model_config: ModelConfiguration,
    pub execution_config: ExecutionConfiguration,
    pub security_config: SecurityConfiguration,
}

/// モデル設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfiguration {
    pub model_name: String,
    pub model_version: String,
    pub parameters: HashMap<String, f64>,
    pub context_window: usize,
}

/// 実行設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConfiguration {
    pub max_concurrent_tasks: usize,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub memory_limit_mb: usize,
}

/// セキュリティ設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfiguration {
    pub api_key_required: bool,
    pub rate_limit: Option<u32>,
    pub allowed_ips: Vec<String>,
    pub encryption_enabled: bool,
}

/// タスクエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub agent_id: AgentId,
    pub name: String,
    pub description: String,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub input_data: serde_json::Value,
    pub output_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

/// タスクID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub Uuid);

impl TaskId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// タスクタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    TextGeneration,
    ImageGeneration,
    DataAnalysis,
    ModelTraining,
    SystemMonitoring,
    Custom(String),
}

/// タスクステータス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// タスク優先度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// メッセージエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: MessageId,
    pub sender_id: AgentId,
    pub receiver_id: Option<AgentId>,
    pub content: MessageContent,
    pub message_type: MessageType,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

/// メッセージID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(pub Uuid);

impl MessageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// メッセージコンテンツ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContent {
    pub text: Option<String>,
    pub data: Option<serde_json::Value>,
    pub attachments: Vec<Attachment>,
}

/// メッセージタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Data,
    Command,
    Response,
    Error,
    System,
}

/// 添付ファイル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub name: String,
    pub content_type: String,
    pub size: usize,
    pub data: Vec<u8>,
}

/// 学習セッションエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSession {
    pub id: LearningSessionId,
    pub agent_id: AgentId,
    pub session_type: LearningSessionType,
    pub status: LearningSessionStatus,
    pub training_data: Vec<TrainingData>,
    pub model_snapshot: Option<ModelSnapshot>,
    pub metrics: LearningMetrics,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// 学習セッションID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LearningSessionId(pub Uuid);

impl LearningSessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// 学習セッションタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningSessionType {
    Supervised,
    Unsupervised,
    Reinforcement,
    Transfer,
    FineTuning,
}

/// 学習セッションステータス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningSessionStatus {
    Preparing,
    Training,
    Evaluating,
    Completed,
    Failed,
}

/// トレーニングデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingData {
    pub input: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub weight: f64,
}

/// モデルスナップショット
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSnapshot {
    pub model_data: Vec<u8>,
    pub version: String,
    pub checksum: String,
}

/// 学習メトリクス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningMetrics {
    pub accuracy: Option<f64>,
    pub loss: Option<f64>,
    pub precision: Option<f64>,
    pub recall: Option<f64>,
    pub f1_score: Option<f64>,
    pub custom_metrics: HashMap<String, f64>,
}
