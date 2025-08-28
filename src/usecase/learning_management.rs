use async_trait::async_trait;
use crate::domain::*;
use crate::shared::error::Result;

/// 学習管理ユースケース
pub struct LearningManagementUseCase {
    learning_repo: Box<dyn LearningSessionRepository>,
    agent_repo: Box<dyn AgentRepository>,
    learning_service: Box<dyn LearningManagementService>,
}

impl LearningManagementUseCase {
    pub fn new(
        learning_repo: Box<dyn LearningSessionRepository>,
        agent_repo: Box<dyn LearningSessionRepository>,
        learning_service: Box<dyn LearningManagementService>,
    ) -> Self {
        Self {
            learning_repo,
            agent_repo,
            learning_service,
        }
    }

    /// 学習セッションを開始する
    pub async fn start_learning_session(&self, request: StartLearningSessionRequest) -> Result<LearningSession> {
        // エージェントの存在確認
        let _agent = self.agent_repo.find_by_id(&request.agent_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Agent with id {} not found", request.agent_id.0)
            ))?;

        // トレーニングデータの検証
        if !self.learning_service.validate_training_data(&request.training_data).await? {
            return Err(crate::shared::error::Error::ValidationError(
                "Invalid training data".to_string(),
            ));
        }

        // 学習セッションの開始
        let session = self.learning_service.start_learning_session(request).await?;
        
        // リポジトリに保存
        let saved_session = self.learning_repo.create(&session).await?;
        
        Ok(saved_session)
    }

    /// 学習の進捗を更新する
    pub async fn update_learning_progress(
        &self,
        session_id: &LearningSessionId,
        metrics: LearningMetrics,
    ) -> Result<LearningSession> {
        // セッションの存在確認
        let session = self.learning_repo.find_by_id(session_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Learning session with id {} not found", session_id.0)
            ))?;

        // 進捗の更新
        let updated_session = self.learning_service.update_learning_progress(session_id, metrics).await?;
        
        // リポジトリに保存
        let saved_session = self.learning_repo.update(&updated_session).await?;
        
        Ok(saved_session)
    }

    /// 学習セッションを完了する
    pub async fn complete_learning_session(
        &self,
        session_id: &LearningSessionId,
        final_metrics: LearningMetrics,
    ) -> Result<LearningSession> {
        // セッションの存在確認
        let session = self.learning_repo.find_by_id(session_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Learning session with id {} not found", session_id.0)
            ))?;

        // セッションの完了
        let completed_session = self.learning_service.complete_learning_session(session_id, final_metrics).await?;
        
        // リポジトリに保存
        let saved_session = self.learning_repo.update(&completed_session).await?;
        
        Ok(saved_session)
    }

    /// モデルスナップショットを保存する
    pub async fn save_model_snapshot(
        &self,
        session_id: &LearningSessionId,
        snapshot: ModelSnapshot,
    ) -> Result<LearningSession> {
        // セッションの存在確認
        let session = self.learning_repo.find_by_id(session_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Learning session with id {} not found", session_id.0)
            ))?;

        // スナップショットの保存
        let updated_session = self.learning_service.save_model_snapshot(session_id, snapshot).await?;
        
        // リポジトリに保存
        let saved_session = self.learning_repo.update(&updated_session).await?;
        
        Ok(saved_session)
    }

    /// 学習セッションを削除する
    pub async fn delete_learning_session(&self, session_id: &LearningSessionId) -> Result<()> {
        // セッションの存在確認
        let _session = self.learning_repo.find_by_id(session_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Learning session with id {} not found", session_id.0)
            ))?;

        // リポジトリから削除
        self.learning_repo.delete(session_id).await?;
        
        Ok(())
    }

    /// 学習セッションを検索する
    pub async fn find_learning_session(&self, session_id: &LearningSessionId) -> Result<Option<LearningSession>> {
        self.learning_repo.find_by_id(session_id).await
    }

    /// エージェントの学習セッションを取得する
    pub async fn find_learning_sessions_by_agent(&self, agent_id: &AgentId) -> Result<Vec<LearningSession>> {
        self.learning_repo.find_by_agent_id(agent_id).await
    }

    /// ステータスで学習セッションを検索する
    pub async fn find_learning_sessions_by_status(&self, status: &LearningSessionStatus) -> Result<Vec<LearningSession>> {
        self.learning_repo.find_by_status(status).await
    }

    /// タイプで学習セッションを検索する
    pub async fn find_learning_sessions_by_type(&self, session_type: &LearningSessionType) -> Result<Vec<LearningSession>> {
        self.learning_repo.find_by_type(session_type).await
    }

    /// アクティブな学習セッションを取得する
    pub async fn get_active_learning_sessions(&self) -> Result<Vec<LearningSession>> {
        self.learning_repo.find_active_sessions().await
    }

    /// すべての学習セッションを取得する
    pub async fn list_all_learning_sessions(&self) -> Result<Vec<LearningSession>> {
        self.learning_repo.find_all().await
    }

    /// 学習セッション数を取得する
    pub async fn get_learning_session_count(&self) -> Result<usize> {
        self.learning_repo.count().await
    }

    /// ステータス別の学習セッション数を取得する
    pub async fn get_learning_session_count_by_status(&self, status: &LearningSessionStatus) -> Result<usize> {
        self.learning_repo.count_by_status(status).await
    }

    /// 学習セッションの統計情報を取得する
    pub async fn get_learning_session_statistics(&self) -> Result<LearningSessionStatistics> {
        let total_sessions = self.learning_repo.count().await?;
        let preparing_sessions = self.learning_repo.count_by_status(&LearningSessionStatus::Preparing).await?;
        let training_sessions = self.learning_repo.count_by_status(&LearningSessionStatus::Training).await?;
        let evaluating_sessions = self.learning_repo.count_by_status(&LearningSessionStatus::Evaluating).await?;
        let completed_sessions = self.learning_repo.count_by_status(&LearningSessionStatus::Completed).await?;
        let failed_sessions = self.learning_repo.count_by_status(&LearningSessionStatus::Failed).await?;

        Ok(LearningSessionStatistics {
            total_sessions,
            preparing_sessions,
            training_sessions,
            evaluating_sessions,
            completed_sessions,
            failed_sessions,
        })
    }

    /// 学習メトリクスを計算する
    pub async fn calculate_learning_metrics(
        &self,
        predictions: &[f64],
        actuals: &[f64],
    ) -> Result<LearningMetrics> {
        self.learning_service.calculate_learning_metrics(predictions, actuals).await
    }

    /// モデルの性能を評価する
    pub async fn evaluate_model_performance(
        &self,
        session_id: &LearningSessionId,
        test_data: &[TrainingData],
    ) -> Result<LearningMetrics> {
        // セッションの存在確認
        let _session = self.learning_repo.find_by_id(session_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Learning session with id {} not found", session_id.0)
            ))?;

        // テストデータの検証
        if !self.learning_service.validate_training_data(test_data).await? {
            return Err(crate::shared::error::Error::ValidationError(
                "Invalid test data".to_string(),
            ));
        }

        // ダミーの予測と実際の値を生成（実際の実装ではモデルから予測を取得）
        let predictions: Vec<f64> = test_data.iter().map(|_| rand::random::<f64>()).collect();
        let actuals: Vec<f64> = test_data.iter().map(|_| rand::random::<f64>()).collect();

        // メトリクスの計算
        let metrics = self.learning_service.calculate_learning_metrics(&predictions, &actuals).await?;
        
        Ok(metrics)
    }

    /// 学習セッションの履歴を取得する
    pub async fn get_learning_history(&self, agent_id: &AgentId, limit: usize) -> Result<Vec<LearningSession>> {
        let mut sessions = self.learning_repo.find_by_agent_id(agent_id).await?;
        sessions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        sessions.truncate(limit);
        Ok(sessions)
    }
}

/// 学習セッション統計情報
#[derive(Debug, Clone)]
pub struct LearningSessionStatistics {
    pub total_sessions: usize,
    pub preparing_sessions: usize,
    pub training_sessions: usize,
    pub evaluating_sessions: usize,
    pub completed_sessions: usize,
    pub failed_sessions: usize,
}
