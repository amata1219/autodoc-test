use axum::{
    routing::{get, post, put, delete},
    Router, Json, extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use crate::domain::*;
use crate::usecase::*;
use crate::shared::error::{Result, Error};

/// APIルーターを作成
pub fn create_api_router(
    agent_use_case: Arc<AgentManagementUseCase>,
    task_use_case: Arc<TaskManagementUseCase>,
    learning_use_case: Arc<LearningManagementUseCase>,
) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/agents", get(list_agents))
        .route("/agents", post(create_agent))
        .route("/agents/:id", get(get_agent))
        .route("/agents/:id", put(update_agent))
        .route("/agents/:id", delete(delete_agent))
        .route("/agents/:id/status", put(update_agent_status))
        .route("/agents/:id/capabilities", post(add_agent_capability))
        .route("/agents/:id/capabilities/:capability_name", delete(remove_agent_capability))
        .route("/agents/statistics", get(get_agent_statistics))
        .route("/tasks", get(list_tasks))
        .route("/tasks", post(create_task))
        .route("/tasks/:id", get(get_task))
        .route("/tasks/:id", put(update_task))
        .route("/tasks/:id", delete(delete_task))
        .route("/tasks/:id/start", post(start_task))
        .route("/tasks/:id/complete", post(complete_task))
        .route("/tasks/:id/fail", post(fail_task))
        .route("/tasks/:id/cancel", post(cancel_task))
        .route("/tasks/statistics", get(get_task_statistics))
        .route("/learning-sessions", get(list_learning_sessions))
        .route("/learning-sessions", post(create_learning_session))
        .route("/learning-sessions/:id", get(get_learning_session))
        .route("/learning-sessions/:id", delete(delete_learning_session))
        .route("/learning-sessions/:id/progress", put(update_learning_progress))
        .route("/learning-sessions/:id/complete", post(complete_learning_session))
        .route("/learning-sessions/statistics", get(get_learning_session_statistics))
        .with_state(AppState {
            agent_use_case,
            task_use_case,
            learning_use_case,
        })
}

/// アプリケーション状態
#[derive(Clone)]
pub struct AppState {
    agent_use_case: Arc<AgentManagementUseCase>,
    task_use_case: Arc<TaskManagementUseCase>,
    learning_use_case: Arc<LearningManagementUseCase>,
}

/// ヘルスチェック
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// エージェント一覧取得
async fn list_agents(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Agent>>, ApiError> {
    let agents = if let Some(agent_type) = params.get("type") {
        let agent_type = serde_json::from_str(agent_type)
            .map_err(|_| ApiError::BadRequest("Invalid agent type".to_string()))?;
        state.agent_use_case.find_agents_by_type(&agent_type).await?
    } else if let Some(status) = params.get("status") {
        let status = serde_json::from_str(status)
            .map_err(|_| ApiError::BadRequest("Invalid status".to_string()))?;
        state.agent_use_case.find_agents_by_status(&status).await?
    } else {
        state.agent_use_case.list_all_agents().await?
    };

    Ok(Json(agents))
}

/// エージェント作成
async fn create_agent(
    State(state): State<AppState>,
    Json(request): Json<CreateAgentRequest>,
) -> Result<Json<Agent>, ApiError> {
    let agent = state.agent_use_case.create_agent(request).await?;
    Ok(Json(agent))
}

/// エージェント取得
async fn get_agent(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Agent>, ApiError> {
    let agent_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid agent ID".to_string()))?;
    
    let agent = state.agent_use_case.find_agent(&AgentId(agent_id)).await?
        .ok_or_else(|| ApiError::NotFound("Agent not found".to_string()))?;
    
    Ok(Json(agent))
}

/// エージェント更新
async fn update_agent(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(agent): Json<Agent>,
) -> Result<Json<Agent>, ApiError> {
    let agent_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid agent ID".to_string()))?;
    
    if agent.id.0 != agent_id {
        return Err(ApiError::BadRequest("Agent ID mismatch".to_string()));
    }
    
    let updated_agent = state.agent_use_case.update_agent_configuration(&agent.id, agent.configuration).await?;
    Ok(Json(updated_agent))
}

/// エージェント削除
async fn delete_agent(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    let agent_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid agent ID".to_string()))?;
    
    state.agent_use_case.delete_agent(&AgentId(agent_id)).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// エージェントステータス更新
async fn update_agent_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<UpdateAgentStatusRequest>,
) -> Result<Json<Agent>, ApiError> {
    let agent_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid agent ID".to_string()))?;
    
    let agent = state.agent_use_case.update_agent_status(&AgentId(agent_id), request.status).await?;
    Ok(Json(agent))
}

/// エージェント能力追加
async fn add_agent_capability(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(capability): Json<Capability>,
) -> Result<Json<Agent>, ApiError> {
    let agent_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid agent ID".to_string()))?;
    
    let agent = state.agent_use_case.add_capability(&AgentId(agent_id), capability).await?;
    Ok(Json(agent))
}

/// エージェント能力削除
async fn remove_agent_capability(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Path(capability_name): Path<String>,
) -> Result<Json<Agent>, ApiError> {
    let agent_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid agent ID".to_string()))?;
    
    let agent = state.agent_use_case.remove_capability(&AgentId(agent_id), &capability_name).await?;
    Ok(Json(agent))
}

/// エージェント統計取得
async fn get_agent_statistics(
    State(state): State<AppState>,
) -> Result<Json<AgentStatistics>, ApiError> {
    let stats = state.agent_use_case.get_agent_statistics().await?;
    Ok(Json(stats))
}

/// タスク一覧取得
async fn list_tasks(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Task>>, ApiError> {
    let tasks = if let Some(agent_id) = params.get("agent_id") {
        let agent_id = Uuid::parse_str(agent_id)
            .map_err(|_| ApiError::BadRequest("Invalid agent ID".to_string()))?;
        state.task_use_case.find_tasks_by_agent(&AgentId(agent_id)).await?
    } else if let Some(status) = params.get("status") {
        let status = serde_json::from_str(status)
            .map_err(|_| ApiError::BadRequest("Invalid status".to_string()))?;
        state.task_use_case.find_tasks_by_status(&status).await?
    } else {
        state.task_use_case.list_all_tasks().await?
    };

    Ok(Json(tasks))
}

/// タスク作成
async fn create_task(
    State(state): State<AppState>,
    Json(request): Json<CreateTaskRequest>,
) -> Result<Json<Task>, ApiError> {
    let task = state.task_use_case.create_task(request).await?;
    Ok(Json(task))
}

/// タスク取得
async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Task>, ApiError> {
    let task_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid task ID".to_string()))?;
    
    let task = state.task_use_case.find_task(&TaskId(task_id)).await?
        .ok_or_else(|| ApiError::NotFound("Task not found".to_string()))?;
    
    Ok(Json(task))
}

/// タスク更新
async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(task): Json<Task>,
) -> Result<Json<Task>, ApiError> {
    let task_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid task ID".to_string()))?;
    
    if task.id.0 != task_id {
        return Err(ApiError::BadRequest("Task ID mismatch".to_string()));
    }
    
    // タスクの更新処理（実際の実装では適切な更新メソッドを呼び出す）
    Ok(Json(task))
}

/// タスク削除
async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    let task_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid task ID".to_string()))?;
    
    state.task_use_case.delete_task(&TaskId(task_id)).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// タスク開始
async fn start_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Task>, ApiError> {
    let task_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid task ID".to_string()))?;
    
    let task = state.task_use_case.start_task(&TaskId(task_id)).await?;
    Ok(Json(task))
}

/// タスク完了
async fn complete_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<CompleteTaskRequest>,
) -> Result<Json<Task>, ApiError> {
    let task_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid task ID".to_string()))?;
    
    let task = state.task_use_case.complete_task(&TaskId(task_id), request.output).await?;
    Ok(Json(task))
}

/// タスク失敗
async fn fail_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<FailTaskRequest>,
) -> Result<Json<Task>, ApiError> {
    let task_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid task ID".to_string()))?;
    
    let task = state.task_use_case.fail_task(&TaskId(task_id), request.error_message).await?;
    Ok(Json(task))
}

/// タスクキャンセル
async fn cancel_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Task>, ApiError> {
    let task_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid task ID".to_string()))?;
    
    let task = state.task_use_case.cancel_task(&TaskId(task_id)).await?;
    Ok(Json(task))
}

/// タスク統計取得
async fn get_task_statistics(
    State(state): State<AppState>,
) -> Result<Json<TaskStatistics>, ApiError> {
    let stats = state.task_use_case.get_task_statistics().await?;
    Ok(Json(stats))
}

/// 学習セッション一覧取得
async fn list_learning_sessions(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<LearningSession>>, ApiError> {
    let sessions = if let Some(agent_id) = params.get("agent_id") {
        let agent_id = Uuid::parse_str(agent_id)
            .map_err(|_| ApiError::BadRequest("Invalid agent ID".to_string()))?;
        state.learning_use_case.find_learning_sessions_by_agent(&AgentId(agent_id)).await?
    } else if let Some(status) = params.get("status") {
        let status = serde_json::from_str(status)
            .map_err(|_| ApiError::BadRequest("Invalid status".to_string()))?;
        state.learning_use_case.find_learning_sessions_by_status(&status).await?
    } else {
        state.learning_use_case.list_all_learning_sessions().await?
    };

    Ok(Json(sessions))
}

/// 学習セッション作成
async fn create_learning_session(
    State(state): State<AppState>,
    Json(request): Json<StartLearningSessionRequest>,
) -> Result<Json<LearningSession>, ApiError> {
    let session = state.learning_use_case.start_learning_session(request).await?;
    Ok(Json(session))
}

/// 学習セッション取得
async fn get_learning_session(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<LearningSession>, ApiError> {
    let session_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid session ID".to_string()))?;
    
    let session = state.learning_use_case.find_learning_session(&LearningSessionId(session_id)).await?
        .ok_or_else(|| ApiError::NotFound("Learning session not found".to_string()))?;
    
    Ok(Json(session))
}

/// 学習セッション削除
async fn delete_learning_session(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    let session_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid session ID".to_string()))?;
    
    state.learning_use_case.delete_learning_session(&LearningSessionId(session_id)).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// 学習進捗更新
async fn update_learning_progress(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(metrics): Json<LearningMetrics>,
) -> Result<Json<LearningSession>, ApiError> {
    let session_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid session ID".to_string()))?;
    
    let session = state.learning_use_case.update_learning_progress(&LearningSessionId(session_id), metrics).await?;
    Ok(Json(session))
}

/// 学習セッション完了
async fn complete_learning_session(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<CompleteLearningSessionRequest>,
) -> Result<Json<LearningSession>, ApiError> {
    let session_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid session ID".to_string()))?;
    
    let session = state.learning_use_case.complete_learning_session(&LearningSessionId(session_id), request.final_metrics).await?;
    Ok(Json(session))
}

/// 学習セッション統計取得
async fn get_learning_session_statistics(
    State(state): State<AppState>,
) -> Result<Json<LearningSessionStatistics>, ApiError> {
    let stats = state.learning_use_case.get_learning_session_statistics().await?;
    Ok(Json(stats))
}

// リクエスト/レスポンス構造体

#[derive(Deserialize)]
pub struct UpdateAgentStatusRequest {
    pub status: AgentStatus,
}

#[derive(Deserialize)]
pub struct CompleteTaskRequest {
    pub output: serde_json::Value,
}

#[derive(Deserialize)]
pub struct FailTaskRequest {
    pub error_message: String,
}

#[derive(Deserialize)]
pub struct CompleteLearningSessionRequest {
    pub final_metrics: LearningMetrics,
}

/// APIエラー
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(serde_json::json!({
            "error": message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        match err {
            Error::ValidationError(msg) => ApiError::BadRequest(msg),
            Error::NotFound(msg) => ApiError::NotFound(msg),
            Error::AuthenticationError(msg) => ApiError::BadRequest(msg),
            Error::AuthorizationError(msg) => ApiError::BadRequest(msg),
            Error::Conflict(msg) => ApiError::BadRequest(msg),
            _ => ApiError::InternalServerError(err.to_string()),
        }
    }
}
