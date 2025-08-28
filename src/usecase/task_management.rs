use async_trait::async_trait;
use crate::domain::*;
use crate::shared::error::Result;

/// タスク管理ユースケース
pub struct TaskManagementUseCase {
    task_repo: Box<dyn TaskRepository>,
    agent_repo: Box<dyn AgentRepository>,
    task_service: Box<dyn TaskManagementService>,
    orchestration_service: Box<dyn AgentOrchestrationService>,
}

impl TaskManagementUseCase {
    pub fn new(
        task_repo: Box<dyn TaskRepository>,
        agent_repo: Box<dyn AgentRepository>,
        task_service: Box<dyn TaskManagementService>,
        orchestration_service: Box<dyn AgentOrchestrationService>,
    ) -> Self {
        Self {
            task_repo,
            agent_repo,
            task_service,
            orchestration_service,
        }
    }

    /// タスクを作成する
    pub async fn create_task(&self, request: CreateTaskRequest) -> Result<Task> {
        // エージェントの存在確認
        let _agent = self.agent_repo.find_by_id(&request.agent_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Agent with id {} not found", request.agent_id.0)
            ))?;

        // タスクの作成
        let task = self.task_service.create_task(request).await?;
        
        // リポジトリに保存
        let saved_task = self.task_repo.create(&task).await?;
        
        Ok(saved_task)
    }

    /// タスクを開始する
    pub async fn start_task(&self, task_id: &TaskId) -> Result<Task> {
        // タスクの存在確認
        let task = self.task_repo.find_by_id(task_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Task with id {} not found", task_id.0)
            ))?;

        // タスクの開始
        let started_task = self.task_service.start_task(task_id).await?;
        
        // リポジトリに保存
        let saved_task = self.task_repo.update(&started_task).await?;
        
        Ok(saved_task)
    }

    /// タスクを完了する
    pub async fn complete_task(&self, task_id: &TaskId, output: serde_json::Value) -> Result<Task> {
        // タスクの存在確認
        let task = self.task_repo.find_by_id(task_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Task with id {} not found", task_id.0)
            ))?;

        // タスクの完了
        let completed_task = self.task_service.complete_task(task_id, output).await?;
        
        // リポジトリに保存
        let saved_task = self.task_repo.update(&completed_task).await?;
        
        Ok(saved_task)
    }

    /// タスクを失敗としてマークする
    pub async fn fail_task(&self, task_id: &TaskId, error_message: String) -> Result<Task> {
        // タスクの存在確認
        let task = self.task_repo.find_by_id(task_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Task with id {} not found", task_id.0)
            ))?;

        // タスクの失敗マーク
        let failed_task = self.task_service.fail_task(task_id, error_message).await?;
        
        // リポジトリに保存
        let saved_task = self.task_repo.update(&failed_task).await?;
        
        Ok(saved_task)
    }

    /// タスクをキャンセルする
    pub async fn cancel_task(&self, task_id: &TaskId) -> Result<Task> {
        // タスクの存在確認
        let task = self.task_repo.find_by_id(task_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Task with id {} not found", task_id.0)
            ))?;

        // タスクのキャンセル
        let cancelled_task = self.task_service.cancel_task(task_id).await?;
        
        // リポジトリに保存
        let saved_task = self.task_repo.update(&cancelled_task).await?;
        
        Ok(saved_task)
    }

    /// タスクの優先度を変更する
    pub async fn prioritize_task(&self, task_id: &TaskId, priority: TaskPriority) -> Result<Task> {
        // タスクの存在確認
        let task = self.task_repo.find_by_id(task_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Task with id {} not found", task_id.0)
            ))?;

        // タスクの優先度変更
        let prioritized_task = self.task_service.prioritize_task(task_id, priority).await?;
        
        // リポジトリに保存
        let saved_task = self.task_repo.update(&prioritized_task).await?;
        
        Ok(saved_task)
    }

    /// タスクを削除する
    pub async fn delete_task(&self, task_id: &TaskId) -> Result<()> {
        // タスクの存在確認
        let _task = self.task_repo.find_by_id(task_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Task with id {} not found", task_id.0)
            ))?;

        // リポジトリから削除
        self.task_repo.delete(task_id).await?;
        
        Ok(())
    }

    /// タスクを検索する
    pub async fn find_task(&self, task_id: &TaskId) -> Result<Option<Task>> {
        self.task_repo.find_by_id(task_id).await
    }

    /// エージェントのタスクを取得する
    pub async fn find_tasks_by_agent(&self, agent_id: &AgentId) -> Result<Vec<Task>> {
        self.task_repo.find_by_agent_id(agent_id).await
    }

    /// ステータスでタスクを検索する
    pub async fn find_tasks_by_status(&self, status: &TaskStatus) -> Result<Vec<Task>> {
        self.task_repo.find_by_status(status).await
    }

    /// 優先度でタスクを検索する
    pub async fn find_tasks_by_priority(&self, priority: &TaskPriority) -> Result<Vec<Task>> {
        self.task_repo.find_by_priority(priority).await
    }

    /// 保留中のタスクを取得する
    pub async fn get_pending_tasks(&self) -> Result<Vec<Task>> {
        self.task_repo.find_pending_tasks().await
    }

    /// 実行中のタスクを取得する
    pub async fn get_running_tasks(&self) -> Result<Vec<Task>> {
        self.task_repo.find_running_tasks().await
    }

    /// すべてのタスクを取得する
    pub async fn list_all_tasks(&self) -> Result<Vec<Task>> {
        self.task_repo.find_all().await
    }

    /// タスク数を取得する
    pub async fn get_task_count(&self) -> Result<usize> {
        self.task_repo.count().await
    }

    /// ステータス別のタスク数を取得する
    pub async fn get_task_count_by_status(&self, status: &TaskStatus) -> Result<usize> {
        self.task_repo.count_by_status(status).await
    }

    /// タスクの統計情報を取得する
    pub async fn get_task_statistics(&self) -> Result<TaskStatistics> {
        let total_tasks = self.task_repo.count().await?;
        let pending_tasks = self.task_repo.count_by_status(&TaskStatus::Pending).await?;
        let running_tasks = self.task_repo.count_by_status(&TaskStatus::Running).await?;
        let completed_tasks = self.task_repo.count_by_status(&TaskStatus::Completed).await?;
        let failed_tasks = self.task_repo.count_by_status(&TaskStatus::Failed).await?;
        let cancelled_tasks = self.task_repo.count_by_status(&TaskStatus::Cancelled).await?;

        Ok(TaskStatistics {
            total_tasks,
            pending_tasks,
            running_tasks,
            completed_tasks,
            failed_tasks,
            cancelled_tasks,
        })
    }

    /// ワークロードのバランスを取る
    pub async fn balance_workload(&self) -> Result<HashMap<AgentId, usize>> {
        self.orchestration_service.balance_workload().await
    }

    /// エージェントの失敗を検出する
    pub async fn detect_agent_failures(&self) -> Result<Vec<AgentId>> {
        self.orchestration_service.detect_agent_failures().await
    }

    /// 失敗したエージェントのタスクを再配布する
    pub async fn redistribute_tasks(&self, failed_agent_id: &AgentId) -> Result<()> {
        self.orchestration_service.redistribute_tasks(failed_agent_id).await
    }

    /// エージェントの割り当てを最適化する
    pub async fn optimize_agent_allocation(&self) -> Result<HashMap<TaskType, Vec<AgentId>>> {
        self.orchestration_service.optimize_agent_allocation().await
    }
}

/// タスク統計情報
#[derive(Debug, Clone)]
pub struct TaskStatistics {
    pub total_tasks: usize,
    pub pending_tasks: usize,
    pub running_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub cancelled_tasks: usize,
}
