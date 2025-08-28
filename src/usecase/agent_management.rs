use async_trait::async_trait;
use std::collections::HashMap;
use crate::domain::*;
use crate::shared::error::Result;

/// エージェント管理ユースケース
pub struct AgentManagementUseCase {
    agent_repo: Box<dyn AgentRepository>,
    agent_service: Box<dyn AgentManagementService>,
    security_service: Box<dyn SecurityService>,
}

impl AgentManagementUseCase {
    pub fn new(
        agent_repo: Box<dyn AgentRepository>,
        agent_service: Box<dyn AgentManagementService>,
        security_service: Box<dyn SecurityService>,
    ) -> Self {
        Self {
            agent_repo,
            agent_service,
            security_service,
        }
    }

    /// エージェントを作成する
    pub async fn create_agent(&self, request: CreateAgentRequest) -> Result<Agent> {
        // 設定の検証
        if !self.agent_service.validate_agent_configuration(&request.configuration).await? {
            return Err(crate::shared::error::Error::ValidationError(
                "Invalid agent configuration".to_string(),
            ));
        }

        // エージェントの作成
        let agent = self.agent_service.create_agent(request).await?;
        
        // リポジトリに保存
        let saved_agent = self.agent_repo.create(&agent).await?;
        
        Ok(saved_agent)
    }

    /// エージェントのステータスを更新する
    pub async fn update_agent_status(
        &self,
        agent_id: &AgentId,
        new_status: AgentStatus,
    ) -> Result<Agent> {
        // エージェントの存在確認
        let agent = self.agent_repo.find_by_id(agent_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Agent with id {} not found", agent_id.0)
            ))?;

        // ステータス更新
        let updated_agent = self.agent_service.update_agent_status(agent_id, new_status).await?;
        
        // リポジトリに保存
        let saved_agent = self.agent_repo.update(&updated_agent).await?;
        
        Ok(saved_agent)
    }

    /// エージェントに能力を追加する
    pub async fn add_capability(
        &self,
        agent_id: &AgentId,
        capability: Capability,
    ) -> Result<Agent> {
        // エージェントの存在確認
        let agent = self.agent_repo.find_by_id(agent_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Agent with id {} not found", agent_id.0)
            ))?;

        // 能力の追加
        let updated_agent = self.agent_service.add_capability(agent_id, capability).await?;
        
        // リポジトリに保存
        let saved_agent = self.agent_repo.update(&updated_agent).await?;
        
        Ok(saved_agent)
    }

    /// エージェントから能力を削除する
    pub async fn remove_capability(
        &self,
        agent_id: &AgentId,
        capability_name: &str,
    ) -> Result<Agent> {
        // エージェントの存在確認
        let agent = self.agent_repo.find_by_id(agent_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Agent with id {} not found", agent_id.0)
            ))?;

        // 能力の削除
        let updated_agent = self.agent_service.remove_capability(agent_id, capability_name).await?;
        
        // リポジトリに保存
        let saved_agent = self.agent_repo.update(&updated_agent).await?;
        
        Ok(saved_agent)
    }

    /// エージェントの設定を更新する
    pub async fn update_agent_configuration(
        &self,
        agent_id: &AgentId,
        new_config: AgentConfiguration,
    ) -> Result<Agent> {
        // 設定の検証
        if !self.agent_service.validate_agent_configuration(&new_config).await? {
            return Err(crate::shared::error::Error::ValidationError(
                "Invalid agent configuration".to_string(),
            ));
        }

        // エージェントの存在確認
        let agent = self.agent_repo.find_by_id(agent_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Agent with id {} not found", agent_id.0)
            ))?;

        // 設定の更新
        let updated_agent = self.agent_service.update_configuration(agent_id, new_config).await?;
        
        // リポジトリに保存
        let saved_agent = self.agent_repo.update(&updated_agent).await?;
        
        Ok(saved_agent)
    }

    /// エージェントを削除する
    pub async fn delete_agent(&self, agent_id: &AgentId) -> Result<()> {
        // エージェントの存在確認
        let _agent = self.agent_repo.find_by_id(agent_id).await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(
                format!("Agent with id {} not found", agent_id.0)
            ))?;

        // リポジトリから削除
        self.agent_repo.delete(agent_id).await?;
        
        Ok(())
    }

    /// エージェントを検索する
    pub async fn find_agent(&self, agent_id: &AgentId) -> Result<Option<Agent>> {
        self.agent_repo.find_by_id(agent_id).await
    }

    /// エージェント名で検索する
    pub async fn find_agent_by_name(&self, name: &str) -> Result<Option<Agent>> {
        self.agent_repo.find_by_name(name).await
    }

    /// すべてのエージェントを取得する
    pub async fn list_all_agents(&self) -> Result<Vec<Agent>> {
        self.agent_repo.find_all().await
    }

    /// エージェントタイプで検索する
    pub async fn find_agents_by_type(&self, agent_type: &AgentType) -> Result<Vec<Agent>> {
        self.agent_repo.find_by_type(agent_type).await
    }

    /// エージェントステータスで検索する
    pub async fn find_agents_by_status(&self, status: &AgentStatus) -> Result<Vec<Agent>> {
        self.agent_repo.find_by_status(status).await
    }

    /// エージェント数を取得する
    pub async fn get_agent_count(&self) -> Result<usize> {
        self.agent_repo.count().await
    }

    /// エージェントの統計情報を取得する
    pub async fn get_agent_statistics(&self) -> Result<AgentStatistics> {
        let total_agents = self.agent_repo.count().await?;
        let active_agents = self.agent_repo.find_by_status(&AgentStatus::Active).await?.len();
        let inactive_agents = self.agent_repo.find_by_status(&AgentStatus::Inactive).await?.len();
        let training_agents = self.agent_repo.find_by_status(&AgentStatus::Training).await?.len();
        let error_agents = self.agent_repo.find_by_status(&AgentStatus::Error).await?.len();

        Ok(AgentStatistics {
            total_agents,
            active_agents,
            inactive_agents,
            training_agents,
            error_agents,
        })
    }
}

/// エージェント統計情報
#[derive(Debug, Clone)]
pub struct AgentStatistics {
    pub total_agents: usize,
    pub active_agents: usize,
    pub inactive_agents: usize,
    pub training_agents: usize,
    pub error_agents: usize,
}
