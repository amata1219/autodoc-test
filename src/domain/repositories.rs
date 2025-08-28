use async_trait::async_trait;
use std::collections::HashMap;
use crate::domain::entities::*;
use crate::shared::error::Result;

/// エージェントリポジトリトレイト
#[async_trait]
pub trait AgentRepository: Send + Sync {
    async fn create(&self, agent: &Agent) -> Result<Agent>;
    async fn find_by_id(&self, id: &AgentId) -> Result<Option<Agent>>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Agent>>;
    async fn find_all(&self) -> Result<Vec<Agent>>;
    async fn find_by_type(&self, agent_type: &AgentType) -> Result<Vec<Agent>>;
    async fn find_by_status(&self, status: &AgentStatus) -> Result<Vec<Agent>>;
    async fn update(&self, agent: &Agent) -> Result<Agent>;
    async fn delete(&self, id: &AgentId) -> Result<()>;
    async fn count(&self) -> Result<usize>;
}

/// タスクリポジトリトレイト
#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create(&self, task: &Task) -> Result<Task>;
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<Task>>;
    async fn find_by_agent_id(&self, agent_id: &AgentId) -> Result<Vec<Task>>;
    async fn find_by_status(&self, status: &TaskStatus) -> Result<Vec<Task>>;
    async fn find_by_priority(&self, priority: &TaskPriority) -> Result<Vec<Task>>;
    async fn find_pending_tasks(&self) -> Result<Vec<Task>>;
    async fn find_running_tasks(&self) -> Result<Vec<Task>>;
    async fn update(&self, task: &Task) -> Result<Task>;
    async fn delete(&self, id: &TaskId) -> Result<()>;
    async fn count(&self) -> Result<usize>;
    async fn count_by_status(&self, status: &TaskStatus) -> Result<usize>;
}

/// メッセージリポジトリトレイト
#[async_trait]
pub trait MessageRepository: Send + Sync {
    async fn create(&self, message: &Message) -> Result<Message>;
    async fn find_by_id(&self, id: &MessageId) -> Result<Option<Message>>;
    async fn find_by_sender_id(&self, sender_id: &AgentId) -> Result<Vec<Message>>;
    async fn find_by_receiver_id(&self, receiver_id: &AgentId) -> Result<Vec<Message>>;
    async fn find_conversation(&self, agent1_id: &AgentId, agent2_id: &AgentId) -> Result<Vec<Message>>;
    async fn find_by_type(&self, message_type: &MessageType) -> Result<Vec<Message>>;
    async fn find_recent_messages(&self, limit: usize) -> Result<Vec<Message>>;
    async fn update(&self, message: &Message) -> Result<Message>;
    async fn delete(&self, id: &MessageId) -> Result<()>;
    async fn count(&self) -> Result<usize>;
}

/// 学習セッションリポジトリトレイト
#[async_trait]
pub trait LearningSessionRepository: Send + Sync {
    async fn create(&self, session: &LearningSession) -> Result<LearningSession>;
    async fn find_by_id(&self, id: &LearningSessionId) -> Result<Option<LearningSession>>;
    async fn find_by_agent_id(&self, agent_id: &AgentId) -> Result<Vec<LearningSession>>;
    async fn find_by_status(&self, status: &LearningSessionStatus) -> Result<Vec<LearningSession>>;
    async fn find_by_type(&self, session_type: &LearningSessionType) -> Result<Vec<LearningSession>>;
    async fn find_active_sessions(&self) -> Result<Vec<LearningSession>>;
    async fn update(&self, session: &LearningSession) -> Result<LearningSession>;
    async fn delete(&self, id: &LearningSessionId) -> Result<()>;
    async fn count(&self) -> Result<usize>;
    async fn count_by_status(&self, status: &LearningSessionStatus) -> Result<usize>;
}

/// 設定リポジトリトレイト
#[async_trait]
pub trait ConfigurationRepository: Send + Sync {
    async fn get_global_config(&self) -> Result<HashMap<String, serde_json::Value>>;
    async fn get_agent_config(&self, agent_id: &AgentId) -> Result<Option<AgentConfiguration>>;
    async fn update_global_config(&self, config: &HashMap<String, serde_json::Value>) -> Result<()>;
    async fn update_agent_config(&self, agent_id: &AgentId, config: &AgentConfiguration) -> Result<()>;
    async fn delete_agent_config(&self, agent_id: &AgentId) -> Result<()>;
}

/// プラグインリポジトリトレイト
#[async_trait]
pub trait PluginRepository: Send + Sync {
    async fn register_plugin(&self, plugin: &Plugin) -> Result<()>;
    async fn unregister_plugin(&self, plugin_id: &str) -> Result<()>;
    async fn get_plugin(&self, plugin_id: &str) -> Result<Option<Plugin>>;
    async fn list_plugins(&self) -> Result<Vec<Plugin>>;
    async fn enable_plugin(&self, plugin_id: &str) -> Result<()>;
    async fn disable_plugin(&self, plugin_id: &str) -> Result<()>;
}

/// プラグインエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub enabled: bool,
    pub configuration: HashMap<String, serde_json::Value>,
    pub dependencies: Vec<String>,
}
