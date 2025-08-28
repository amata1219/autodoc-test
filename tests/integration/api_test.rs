use autodoc_test::domain::*;
use autodoc_test::usecase::*;
use autodoc_test::presentation::web::api::create_api_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;

// モックリポジトリとサービスの実装
use async_trait::async_trait;
use std::collections::HashMap;

struct MockAgentRepository;

#[async_trait]
impl AgentRepository for MockAgentRepository {
    async fn create(&self, agent: &Agent) -> Result<Agent, crate::shared::error::Error> {
        Ok(agent.clone())
    }

    async fn find_by_id(&self, _id: &AgentId) -> Result<Option<Agent>, crate::shared::error::Error> {
        Ok(None)
    }

    async fn find_by_name(&self, _name: &str) -> Result<Option<Agent>, crate::shared::error::Error> {
        Ok(None)
    }

    async fn find_all(&self) -> Result<Vec<Agent>, crate::shared::error::Error> {
        Ok(vec![])
    }

    async fn find_by_type(&self, _agent_type: &AgentType) -> Result<Vec<Agent>, crate::shared::error::Error> {
        Ok(vec![])
    }

    async fn find_by_status(&self, _status: &AgentStatus) -> Result<Vec<Agent>, crate::shared::error::Error> {
        Ok(vec![])
    }

    async fn update(&self, agent: &Agent) -> Result<Agent, crate::shared::error::Error> {
        Ok(agent.clone())
    }

    async fn delete(&self, _id: &AgentId) -> Result<(), crate::shared::error::Error> {
        Ok(())
    }

    async fn count(&self) -> Result<usize, crate::shared::error::Error> {
        Ok(0)
    }
}

struct MockTaskRepository;

#[async_trait]
impl TaskRepository for MockTaskRepository {
    async fn create(&self, task: &Task) -> Result<Task, crate::shared::error::Error> {
        Ok(task.clone())
    }

    async fn find_by_id(&self, _id: &TaskId) -> Result<Option<Task>, crate::shared::error::Error> {
        Ok(None)
    }

    async fn find_by_agent_id(&self, _agent_id: &AgentId) -> Result<Vec<Task>, crate::shared::error::Error> {
        Ok(vec![])
    }

    async fn find_by_status(&self, _status: &TaskStatus) -> Result<Vec<Task>, crate::shared::error::Error> {
        Ok(vec![])
    }

    async fn find_by_priority(&self, _priority: &TaskPriority) -> Result<Vec<Task>, crate::shared::error::Error> {
        Ok(vec![])
    }

    async fn find_pending_tasks(&self) -> Result<Vec<Task>, crate::shared::error::Error> {
        Ok(vec![])
    }

    async fn find_running_tasks(&self) -> Result<Vec<Task>, crate::shared::error::Error> {
        Ok(vec![])
    }

    async fn update(&self, task: &Task) -> Result<Task, crate::shared::error::Error> {
        Ok(task.clone())
    }

    async fn delete(&self, _id: &TaskId) -> Result<(), crate::shared::error::Error> {
        Ok(())
    }

    async fn count(&self) -> Result<usize, crate::shared::error::Error> {
        Ok(0)
    }

    async fn count_by_status(&self, _status: &TaskStatus) -> Result<usize, crate::shared::error::Error> {
        Ok(0)
    }
}

struct MockLearningSessionRepository;

#[async_trait]
impl LearningSessionRepository for MockLearningSessionRepository {
    async fn create(&self, session: &LearningSession) -> Result<LearningSession, crate::shared::error::Error> {
        Ok(session.clone())
    }

    async fn find_by_id(&self, _id: &LearningSessionId) -> Result<Option<LearningSession>, crate::shared::error::Error> {
        Ok(None)
    }

    async fn find_by_agent_id(&self, _agent_id: &AgentId) -> Result<Vec<LearningSession>, crate::shared::error::Error> {
        Ok(vec![])
    }

    async fn find_by_status(&self, _status: &LearningSessionStatus) -> Result<Vec<LearningSession>, crate::shared::error::Error> {
        Ok(vec![])
    }

    async fn find_by_type(&self, _session_type: &LearningSessionType) -> Result<Vec<LearningSession>, crate::shared::error::Error> {
        Ok(vec![])
    }

    async fn find_active_sessions(&self) -> Result<Vec<LearningSession>, crate::shared::error::Error> {
        Ok(vec![])
    }

    async fn update(&self, session: &LearningSession) -> Result<LearningSession, crate::shared::error::Error> {
        Ok(session.clone())
    }

    async fn delete(&self, _id: &LearningSessionId) -> Result<(), crate::shared::error::Error> {
        Ok(())
    }

    async fn count(&self) -> Result<usize, crate::shared::error::Error> {
        Ok(0)
    }

    async fn count_by_status(&self, _status: &LearningSessionStatus) -> Result<usize, crate::shared::error::Error> {
        Ok(0)
    }
}

// モックサービスの実装
struct MockAgentManagementService;

#[async_trait]
impl AgentManagementService for MockAgentManagementService {
    async fn create_agent(&self, request: CreateAgentRequest) -> Result<Agent, crate::shared::error::Error> {
        let now = chrono::Utc::now();
        Ok(Agent {
            id: AgentId::new(),
            name: request.name,
            description: request.description,
            agent_type: request.agent_type,
            status: AgentStatus::Active,
            capabilities: request.capabilities,
            configuration: request.configuration,
            metadata: request.metadata,
            created_at: now,
            updated_at: now,
        })
    }

    async fn update_agent_status(&self, agent_id: &AgentId, status: AgentStatus) -> Result<Agent, crate::shared::error::Error> {
        Ok(Agent {
            id: agent_id.clone(),
            name: "Test Agent".to_string(),
            description: "Test Description".to_string(),
            agent_type: AgentType::Conversational,
            status,
            capabilities: vec![],
            configuration: AgentConfiguration {
                model_config: ModelConfiguration {
                    model_name: "test".to_string(),
                    model_version: "1.0".to_string(),
                    parameters: HashMap::new(),
                    context_window: 1000,
                },
                execution_config: ExecutionConfiguration {
                    max_concurrent_tasks: 10,
                    timeout_seconds: 30,
                    retry_attempts: 3,
                    memory_limit_mb: 100,
                },
                security_config: SecurityConfiguration {
                    api_key_required: false,
                    rate_limit: None,
                    allowed_ips: vec![],
                    encryption_enabled: false,
                },
            },
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    async fn add_capability(&self, agent_id: &AgentId, _capability: Capability) -> Result<Agent, crate::shared::error::Error> {
        self.update_agent_status(agent_id, AgentStatus::Active).await
    }

    async fn remove_capability(&self, agent_id: &AgentId, _capability_name: &str) -> Result<Agent, crate::shared::error::Error> {
        self.update_agent_status(agent_id, AgentStatus::Active).await
    }

    async fn update_configuration(&self, agent_id: &AgentId, _config: AgentConfiguration) -> Result<Agent, crate::shared::error::Error> {
        self.update_agent_status(agent_id, AgentStatus::Active).await
    }

    async fn validate_agent_configuration(&self, _config: &AgentConfiguration) -> Result<bool, crate::shared::error::Error> {
        Ok(true)
    }
}

struct MockTaskManagementService;

#[async_trait]
impl TaskManagementService for MockTaskManagementService {
    async fn create_task(&self, request: CreateTaskRequest) -> Result<Task, crate::shared::error::Error> {
        let now = chrono::Utc::now();
        Ok(Task {
            id: TaskId::new(),
            agent_id: request.agent_id,
            name: request.name,
            description: request.description,
            task_type: request.task_type,
            status: TaskStatus::Pending,
            priority: request.priority,
            input_data: request.input_data,
            output_data: None,
            created_at: now,
            started_at: None,
            completed_at: None,
            error_message: None,
        })
    }

    async fn assign_task(&self, _task_id: &TaskId, _agent_id: &AgentId) -> Result<Task, crate::shared::error::Error> {
        Ok(Task {
            id: TaskId::new(),
            agent_id: AgentId::new(),
            name: "Mock Task".to_string(),
            description: "Mock Description".to_string(),
            task_type: TaskType::TextGeneration,
            status: TaskStatus::Pending,
            priority: TaskPriority::Normal,
            input_data: serde_json::json!({}),
            output_data: None,
            created_at: chrono::Utc::now(),
            started_at: None,
            completed_at: None,
            error_message: None,
        })
    }

    async fn start_task(&self, task_id: &TaskId) -> Result<Task, crate::shared::error::Error> {
        Ok(Task {
            id: task_id.clone(),
            agent_id: AgentId::new(),
            name: "Mock Task".to_string(),
            description: "Mock Description".to_string(),
            task_type: TaskType::TextGeneration,
            status: TaskStatus::Running,
            priority: TaskPriority::Normal,
            input_data: serde_json::json!({}),
            output_data: None,
            created_at: chrono::Utc::now(),
            started_at: Some(chrono::Utc::now()),
            completed_at: None,
            error_message: None,
        })
    }

    async fn complete_task(&self, task_id: &TaskId, output: serde_json::Value) -> Result<Task, crate::shared::error::Error> {
        Ok(Task {
            id: task_id.clone(),
            agent_id: AgentId::new(),
            name: "Mock Task".to_string(),
            description: "Mock Description".to_string(),
            task_type: TaskType::TextGeneration,
            status: TaskStatus::Completed,
            priority: TaskPriority::Normal,
            input_data: serde_json::json!({}),
            output_data: Some(output),
            created_at: chrono::Utc::now(),
            started_at: Some(chrono::Utc::now()),
            completed_at: Some(chrono::Utc::now()),
            error_message: None,
        })
    }

    async fn fail_task(&self, task_id: &TaskId, error_message: String) -> Result<Task, crate::shared::error::Error> {
        Ok(Task {
            id: task_id.clone(),
            agent_id: AgentId::new(),
            name: "Mock Task".to_string(),
            description: "Mock Description".to_string(),
            task_type: TaskType::TextGeneration,
            status: TaskStatus::Failed,
            priority: TaskPriority::Normal,
            input_data: serde_json::json!({}),
            output_data: None,
            created_at: chrono::Utc::now(),
            started_at: Some(chrono::Utc::now()),
            completed_at: None,
            error_message: Some(error_message),
        })
    }

    async fn cancel_task(&self, task_id: &TaskId) -> Result<Task, crate::shared::error::Error> {
        Ok(Task {
            id: task_id.clone(),
            agent_id: AgentId::new(),
            name: "Mock Task".to_string(),
            description: "Mock Description".to_string(),
            task_type: TaskType::TextGeneration,
            status: TaskStatus::Cancelled,
            priority: TaskPriority::Normal,
            input_data: serde_json::json!({}),
            output_data: None,
            created_at: chrono::Utc::now(),
            started_at: None,
            completed_at: None,
            error_message: None,
        })
    }

    async fn prioritize_task(&self, task_id: &TaskId, priority: TaskPriority) -> Result<Task, crate::shared::error::Error> {
        Ok(Task {
            id: task_id.clone(),
            agent_id: AgentId::new(),
            name: "Mock Task".to_string(),
            description: "Mock Description".to_string(),
            task_type: TaskType::TextGeneration,
            status: TaskStatus::Pending,
            priority,
            input_data: serde_json::json!({}),
            output_data: None,
            created_at: chrono::Utc::now(),
            started_at: None,
            completed_at: None,
            error_message: None,
        })
    }

    async fn validate_task_assignment(&self, _task: &Task, _agent: &Agent) -> Result<bool, crate::shared::error::Error> {
        Ok(true)
    }
}

struct MockLearningManagementService;

#[async_trait]
impl LearningManagementService for MockLearningManagementService {
    async fn start_learning_session(&self, request: StartLearningSessionRequest) -> Result<LearningSession, crate::shared::error::Error> {
        let now = chrono::Utc::now();
        Ok(LearningSession {
            id: LearningSessionId::new(),
            agent_id: request.agent_id,
            session_type: request.session_type,
            status: LearningSessionStatus::Preparing,
            training_data: request.training_data,
            model_snapshot: None,
            metrics: LearningMetrics {
                accuracy: None,
                loss: None,
                precision: None,
                recall: None,
                f1_score: None,
                custom_metrics: HashMap::new(),
            },
            created_at: now,
            completed_at: None,
        })
    }

    async fn update_learning_progress(&self, session_id: &LearningSessionId, metrics: LearningMetrics) -> Result<LearningSession, crate::shared::error::Error> {
        Ok(LearningSession {
            id: session_id.clone(),
            agent_id: AgentId::new(),
            session_type: LearningSessionType::Supervised,
            status: LearningSessionStatus::Training,
            training_data: vec![],
            model_snapshot: None,
            metrics,
            created_at: chrono::Utc::now(),
            completed_at: None,
        })
    }

    async fn complete_learning_session(&self, session_id: &LearningSessionId, final_metrics: LearningMetrics) -> Result<LearningSession, crate::shared::error::Error> {
        Ok(LearningSession {
            id: session_id.clone(),
            agent_id: AgentId::new(),
            session_type: LearningSessionType::Supervised,
            status: LearningSessionStatus::Completed,
            training_data: vec![],
            model_snapshot: None,
            metrics: final_metrics,
            created_at: chrono::Utc::now(),
            completed_at: Some(chrono::Utc::now()),
        })
    }

    async fn save_model_snapshot(&self, session_id: &LearningSessionId, snapshot: ModelSnapshot) -> Result<LearningSession, crate::shared::error::Error> {
        Ok(LearningSession {
            id: session_id.clone(),
            agent_id: AgentId::new(),
            session_type: LearningSessionType::Supervised,
            status: LearningSessionStatus::Training,
            training_data: vec![],
            model_snapshot: Some(snapshot),
            metrics: LearningMetrics {
                accuracy: None,
                loss: None,
                precision: None,
                recall: None,
                f1_score: None,
                custom_metrics: HashMap::new(),
            },
            created_at: chrono::Utc::now(),
            completed_at: None,
        })
    }

    async fn validate_training_data(&self, _training_data: &[TrainingData]) -> Result<bool, crate::shared::error::Error> {
        Ok(true)
    }

    async fn calculate_learning_metrics(&self, predictions: &[f64], actuals: &[f64]) -> Result<LearningMetrics, crate::shared::error::Error> {
        if predictions.is_empty() || actuals.is_empty() {
            return Ok(LearningMetrics {
                accuracy: None,
                loss: None,
                precision: None,
                recall: None,
                f1_score: None,
                custom_metrics: HashMap::new(),
            });
        }

        let accuracy = predictions.iter().zip(actuals.iter())
            .map(|(p, a)| if (p - a).abs() < 0.1 { 1.0 } else { 0.0 })
            .sum::<f64>() / predictions.len() as f64;

        Ok(LearningMetrics {
            accuracy: Some(accuracy),
            loss: Some(1.0 - accuracy),
            precision: Some(accuracy),
            recall: Some(accuracy),
            f1_score: Some(accuracy),
            custom_metrics: HashMap::new(),
        })
    }
}

struct MockAgentOrchestrationService;

#[async_trait]
impl AgentOrchestrationService for MockAgentOrchestrationService {
    async fn coordinate_agents(&self, _task_id: &TaskId, _agent_ids: Vec<AgentId>) -> Result<(), crate::shared::error::Error> {
        Ok(())
    }

    async fn balance_workload(&self) -> Result<HashMap<AgentId, usize>, crate::shared::error::Error> {
        Ok(HashMap::new())
    }

    async fn detect_agent_failures(&self) -> Result<Vec<AgentId>, crate::shared::error::Error> {
        Ok(vec![])
    }

    async fn redistribute_tasks(&self, _failed_agent_id: &AgentId) -> Result<(), crate::shared::error::Error> {
        Ok(())
    }

    async fn optimize_agent_allocation(&self) -> Result<HashMap<TaskType, Vec<AgentId>>, crate::shared::error::Error> {
        Ok(HashMap::new())
    }
}

struct MockSecurityService;

#[async_trait]
impl SecurityService for MockSecurityService {
    async fn authenticate_agent(&self, _credentials: &AgentCredentials) -> Result<AuthenticationResult, crate::shared::error::Error> {
        Ok(AuthenticationResult {
            authenticated: true,
            agent_id: Some(AgentId::new()),
            permissions: vec!["read".to_string(), "write".to_string()],
            expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
        })
    }

    async fn authorize_action(&self, _agent_id: &AgentId, _action: &str, _resource: &str) -> Result<bool, crate::shared::error::Error> {
        Ok(true)
    }

    async fn validate_api_key(&self, _api_key: &str) -> Result<Option<AgentId>, crate::shared::error::Error> {
        Some(AgentId::new()).into()
    }

    async fn encrypt_sensitive_data(&self, data: &[u8]) -> Result<Vec<u8>, crate::shared::error::Error> {
        Ok(data.to_vec())
    }

    async fn decrypt_sensitive_data(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, crate::shared::error::Error> {
        Ok(encrypted_data.to_vec())
    }
}

// テスト用のアプリケーションを作成
fn create_test_app() -> axum::Router {
    let agent_repo = Arc::new(MockAgentRepository);
    let task_repo = Arc::new(MockTaskRepository);
    let learning_repo = Arc::new(MockLearningSessionRepository);
    
    let agent_service = Arc::new(MockAgentManagementService);
    let task_service = Arc::new(MockTaskManagementService);
    let learning_service = Arc::new(MockLearningManagementService);
    let orchestration_service = Arc::new(MockAgentOrchestrationService);
    let security_service = Arc::new(MockSecurityService);

    let agent_use_case = Arc::new(AgentManagementUseCase::new(
        agent_repo.clone(),
        agent_service.clone(),
        security_service.clone(),
    ));

    let task_use_case = Arc::new(TaskManagementUseCase::new(
        task_repo.clone(),
        agent_repo.clone(),
        task_service.clone(),
        orchestration_service.clone(),
    ));

    let learning_use_case = Arc::new(LearningManagementUseCase::new(
        learning_repo.clone(),
        agent_repo.clone(),
        learning_service.clone(),
    ));

    create_api_router(agent_use_case, task_use_case, learning_use_case)
}

#[tokio::test]
async fn test_health_check() {
    let app = create_test_app();

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let health: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(health["status"], "healthy");
    assert!(health["timestamp"].is_string());
    assert!(health["version"].is_string());
}

#[tokio::test]
async fn test_create_agent() {
    let app = create_test_app();

    let agent_data = json!({
        "name": "Test Agent",
        "description": "A test agent",
        "agent_type": "Conversational",
        "capabilities": [],
        "configuration": {
            "model_config": {
                "model_name": "test",
                "model_version": "1.0",
                "parameters": {},
                "context_window": 1000
            },
            "execution_config": {
                "max_concurrent_tasks": 10,
                "timeout_seconds": 30,
                "retry_attempts": 3,
                "memory_limit_mb": 100
            },
            "security_config": {
                "api_key_required": false,
                "rate_limit": null,
                "allowed_ips": [],
                "encryption_enabled": false
            }
        },
        "metadata": {}
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/agents")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&agent_data).unwrap()))
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let agent: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(agent["name"], "Test Agent");
    assert_eq!(agent["description"], "A test agent");
    assert_eq!(agent["agent_type"], "Conversational");
    assert_eq!(agent["status"], "Active");
}

#[tokio::test]
async fn test_create_task() {
    let app = create_test_app();

    let task_data = json!({
        "agent_id": "550e8400-e29b-41d4-a716-446655440000",
        "name": "Test Task",
        "description": "A test task",
        "task_type": "TextGeneration",
        "priority": "Normal",
        "input_data": {
            "prompt": "Hello, world!"
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/tasks")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&task_data).unwrap()))
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let task: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(task["name"], "Test Task");
    assert_eq!(task["description"], "A test task");
    assert_eq!(task["task_type"], "TextGeneration");
    assert_eq!(task["status"], "Pending");
    assert_eq!(task["priority"], "Normal");
}

#[tokio::test]
async fn test_start_task() {
    let app = create_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/tasks/550e8400-e29b-41d4-a716-446655440000/start")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let task: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(task["status"], "Running");
}

#[tokio::test]
async fn test_create_learning_session() {
    let app = create_test_app();

    let session_data = json!({
        "agent_id": "550e8400-e29b-41d4-a716-446655440000",
        "session_type": "Supervised",
        "training_data": [
            {
                "input": {"text": "Hello"},
                "output": {"response": "Hi there!"},
                "weight": 1.0
            }
        ]
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/learning-sessions")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&session_data).unwrap()))
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let session: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(session["session_type"], "Supervised");
    assert_eq!(session["status"], "Preparing");
}

#[tokio::test]
async fn test_get_agent_statistics() {
    let app = create_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/agents/statistics")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let stats: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(stats["total_agents"], 0);
    assert_eq!(stats["active_agents"], 0);
    assert_eq!(stats["inactive_agents"], 0);
    assert_eq!(stats["training_agents"], 0);
    assert_eq!(stats["error_agents"], 0);
}

#[tokio::test]
async fn test_get_task_statistics() {
    let app = create_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/tasks/statistics")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let stats: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(stats["total_tasks"], 0);
    assert_eq!(stats["pending_tasks"], 0);
    assert_eq!(stats["running_tasks"], 0);
    assert_eq!(stats["completed_tasks"], 0);
    assert_eq!(stats["failed_tasks"], 0);
    assert_eq!(stats["cancelled_tasks"], 0);
}

#[tokio::test]
async fn test_get_learning_session_statistics() {
    let app = create_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/learning-sessions/statistics")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let stats: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(stats["total_sessions"], 0);
    assert_eq!(stats["preparing_sessions"], 0);
    assert_eq!(stats["training_sessions"], 0);
    assert_eq!(stats["evaluating_sessions"], 0);
    assert_eq!(stats["completed_sessions"], 0);
    assert_eq!(stats["failed_sessions"], 0);
}
