use async_trait::async_trait;
use std::collections::HashMap;
use crate::domain::entities::*;
use crate::shared::error::Result;

/// エージェント管理ドメインサービス
#[async_trait]
pub trait AgentManagementService: Send + Sync {
    async fn create_agent(&self, request: CreateAgentRequest) -> Result<Agent>;
    async fn update_agent_status(&self, agent_id: &AgentId, status: AgentStatus) -> Result<Agent>;
    async fn add_capability(&self, agent_id: &AgentId, capability: Capability) -> Result<Agent>;
    async fn remove_capability(&self, agent_id: &AgentId, capability_name: &str) -> Result<Agent>;
    async fn update_configuration(&self, agent_id: &AgentId, config: AgentConfiguration) -> Result<Agent>;
    async fn validate_agent_configuration(&self, config: &AgentConfiguration) -> Result<bool>;
}

/// エージェント作成リクエスト
#[derive(Debug, Clone)]
pub struct CreateAgentRequest {
    pub name: String,
    pub description: String,
    pub agent_type: AgentType,
    pub capabilities: Vec<Capability>,
    pub configuration: AgentConfiguration,
    pub metadata: HashMap<String, String>,
}

/// タスク管理ドメインサービス
#[async_trait]
pub trait TaskManagementService: Send + Sync {
    async fn create_task(&self, request: CreateTaskRequest) -> Result<Task>;
    async fn assign_task(&self, task_id: &TaskId, agent_id: &AgentId) -> Result<Task>;
    async fn start_task(&self, task_id: &TaskId) -> Result<Task>;
    async fn complete_task(&self, task_id: &TaskId, output: serde_json::Value) -> Result<Task>;
    async fn fail_task(&self, task_id: &TaskId, error_message: String) -> Result<Task>;
    async fn cancel_task(&self, task_id: &TaskId) -> Result<Task>;
    async fn prioritize_task(&self, task_id: &TaskId, priority: TaskPriority) -> Result<Task>;
    async fn validate_task_assignment(&self, task: &Task, agent: &Agent) -> Result<bool>;
}

/// タスク作成リクエスト
#[derive(Debug, Clone)]
pub struct CreateTaskRequest {
    pub agent_id: AgentId,
    pub name: String,
    pub description: String,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    pub input_data: serde_json::Value,
}

/// メッセージングドメインサービス
#[async_trait]
pub trait MessagingService: Send + Sync {
    async fn send_message(&self, request: SendMessageRequest) -> Result<Message>;
    async fn broadcast_message(&self, request: BroadcastMessageRequest) -> Result<Vec<Message>>;
    async fn get_conversation_history(&self, agent1_id: &AgentId, agent2_id: &AgentId, limit: usize) -> Result<Vec<Message>>;
    async fn validate_message(&self, message: &Message) -> Result<bool>;
    async fn encrypt_message_content(&self, content: &MessageContent) -> Result<MessageContent>;
    async fn decrypt_message_content(&self, content: &MessageContent) -> Result<MessageContent>;
}

/// メッセージ送信リクエスト
#[derive(Debug, Clone)]
pub struct SendMessageRequest {
    pub sender_id: AgentId,
    pub receiver_id: Option<AgentId>,
    pub content: MessageContent,
    pub message_type: MessageType,
    pub metadata: HashMap<String, String>,
}

/// ブロードキャストメッセージリクエスト
#[derive(Debug, Clone)]
pub struct BroadcastMessageRequest {
    pub sender_id: AgentId,
    pub content: MessageContent,
    pub message_type: MessageType,
    pub target_agent_types: Vec<AgentType>,
    pub metadata: HashMap<String, String>,
}

/// 学習管理ドメインサービス
#[async_trait]
pub trait LearningManagementService: Send + Sync {
    async fn start_learning_session(&self, request: StartLearningSessionRequest) -> Result<LearningSession>;
    async fn update_learning_progress(&self, session_id: &LearningSessionId, metrics: LearningMetrics) -> Result<LearningSession>;
    async fn complete_learning_session(&self, session_id: &LearningSessionId, final_metrics: LearningMetrics) -> Result<LearningSession>;
    async fn save_model_snapshot(&self, session_id: &LearningSessionId, snapshot: ModelSnapshot) -> Result<LearningSession>;
    async fn validate_training_data(&self, training_data: &[TrainingData]) -> Result<bool>;
    async fn calculate_learning_metrics(&self, predictions: &[f64], actuals: &[f64]) -> Result<LearningMetrics>;
}

/// 学習セッション開始リクエスト
#[derive(Debug, Clone)]
pub struct StartLearningSessionRequest {
    pub agent_id: AgentId,
    pub session_type: LearningSessionType,
    pub training_data: Vec<TrainingData>,
}

/// エージェントオーケストレーションサービス
#[async_trait]
pub trait AgentOrchestrationService: Send + Sync {
    async fn coordinate_agents(&self, task_id: &TaskId, agent_ids: Vec<AgentId>) -> Result<()>;
    async fn balance_workload(&self) -> Result<HashMap<AgentId, usize>>;
    async fn detect_agent_failures(&self) -> Result<Vec<AgentId>>;
    async fn redistribute_tasks(&self, failed_agent_id: &AgentId) -> Result<()>;
    async fn optimize_agent_allocation(&self) -> Result<HashMap<TaskType, Vec<AgentId>>>;
}

/// セキュリティドメインサービス
#[async_trait]
pub trait SecurityService: Send + Sync {
    async fn authenticate_agent(&self, credentials: &AgentCredentials) -> Result<AuthenticationResult>;
    async fn authorize_action(&self, agent_id: &AgentId, action: &str, resource: &str) -> Result<bool>;
    async fn validate_api_key(&self, api_key: &str) -> Result<Option<AgentId>>;
    async fn encrypt_sensitive_data(&self, data: &[u8]) -> Result<Vec<u8>>;
    async fn decrypt_sensitive_data(&self, encrypted_data: &[u8]) -> Result<Vec<u8>>;
}

/// エージェント認証情報
#[derive(Debug, Clone)]
pub struct AgentCredentials {
    pub agent_id: AgentId,
    pub api_key: String,
    pub timestamp: chrono::DateTime<Utc>,
}

/// 認証結果
#[derive(Debug, Clone)]
pub struct AuthenticationResult {
    pub authenticated: bool,
    pub agent_id: Option<AgentId>,
    pub permissions: Vec<String>,
    pub expires_at: chrono::DateTime<Utc>,
}

/// プラグイン管理ドメインサービス
#[async_trait]
pub trait PluginManagementService: Send + Sync {
    async fn install_plugin(&self, plugin_data: &[u8]) -> Result<Plugin>;
    async fn uninstall_plugin(&self, plugin_id: &str) -> Result<()>;
    async fn enable_plugin(&self, plugin_id: &str) -> Result<Plugin>;
    async fn disable_plugin(&self, plugin_id: &str) -> Result<Plugin>;
    async fn update_plugin(&self, plugin_id: &str, new_version: &str) -> Result<Plugin>;
    async fn validate_plugin_dependencies(&self, plugin: &Plugin) -> Result<bool>;
    async fn check_plugin_compatibility(&self, plugin: &Plugin) -> Result<bool>;
}
