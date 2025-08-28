use async_trait::async_trait;
use sqlx::{PgPool, Row, Error as SqlxError};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;
use crate::domain::*;
use crate::shared::error::{Result, Error};

/// SQLxを使用したエージェントリポジトリの実装
pub struct SqlxAgentRepository {
    pool: PgPool,
}

impl SqlxAgentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AgentRepository for SqlxAgentRepository {
    async fn create(&self, agent: &Agent) -> Result<Agent> {
        let mut tx = self.pool.begin().await?;

        // エージェントテーブルに挿入
        let agent_row = sqlx::query!(
            r#"
            INSERT INTO agents (id, name, description, agent_type, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, name, description, agent_type, status, created_at, updated_at
            "#,
            agent.id.0,
            agent.name,
            agent.description,
            serde_json::to_value(&agent.agent_type)?,
            serde_json::to_value(&agent.status)?,
            agent.created_at,
            agent.updated_at
        )
        .fetch_one(&mut *tx)
        .await?;

        // 能力テーブルに挿入
        for capability in &agent.capabilities {
            sqlx::query!(
                r#"
                INSERT INTO agent_capabilities (agent_id, name, description, version, parameters)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                agent.id.0,
                capability.name,
                capability.description,
                capability.version,
                serde_json::to_value(&capability.parameters)?
            )
            .execute(&mut *tx)
            .await?;
        }

        // 設定テーブルに挿入
        sqlx::query!(
            r#"
            INSERT INTO agent_configurations (agent_id, model_config, execution_config, security_config)
            VALUES ($1, $2, $3, $4)
            "#,
            agent.id.0,
            serde_json::to_value(&agent.configuration.model_config)?,
            serde_json::to_value(&agent.configuration.execution_config)?,
            serde_json::to_value(&agent.configuration.security_config)?
        )
        .execute(&mut *tx)
        .await?;

        // メタデータテーブルに挿入
        for (key, value) in &agent.metadata {
            sqlx::query!(
                r#"
                INSERT INTO agent_metadata (agent_id, key, value)
                VALUES ($1, $2, $3)
                "#,
                agent.id.0,
                key,
                value
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(agent.clone())
    }

    async fn find_by_id(&self, id: &AgentId) -> Result<Option<Agent>> {
        let agent_row = sqlx::query!(
            r#"
            SELECT id, name, description, agent_type, status, created_at, updated_at
            FROM agents
            WHERE id = $1
            "#,
            id.0
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = agent_row {
            let agent_type: serde_json::Value = row.agent_type;
            let status: serde_json::Value = row.status;

            // 能力を取得
            let capabilities = sqlx::query!(
                r#"
                SELECT name, description, version, parameters
                FROM agent_capabilities
                WHERE agent_id = $1
                "#,
                id.0
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|cap_row| {
                let parameters: serde_json::Value = cap_row.parameters;
                Capability {
                    name: cap_row.name,
                    description: cap_row.description,
                    version: cap_row.version,
                    parameters: serde_json::from_value(parameters).unwrap_or_default(),
                }
            })
            .collect();

            // 設定を取得
            let config_row = sqlx::query!(
                r#"
                SELECT model_config, execution_config, security_config
                FROM agent_configurations
                WHERE agent_id = $1
                "#,
                id.0
            )
            .fetch_one(&self.pool)
            .await?;

            let model_config: serde_json::Value = config_row.model_config;
            let execution_config: serde_json::Value = config_row.execution_config;
            let security_config: serde_json::Value = config_row.security_config;

            // メタデータを取得
            let metadata = sqlx::query!(
                r#"
                SELECT key, value
                FROM agent_metadata
                WHERE agent_id = $1
                "#,
                id.0
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|meta_row| (meta_row.key, meta_row.value))
            .collect();

            let agent = Agent {
                id: AgentId(row.id),
                name: row.name,
                description: row.description,
                agent_type: serde_json::from_value(agent_type)?,
                status: serde_json::from_value(status)?,
                capabilities,
                configuration: AgentConfiguration {
                    model_config: serde_json::from_value(model_config)?,
                    execution_config: serde_json::from_value(execution_config)?,
                    security_config: serde_json::from_value(security_config)?,
                },
                metadata,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };

            Ok(Some(agent))
        } else {
            Ok(None)
        }
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Agent>> {
        let agent_row = sqlx::query!(
            r#"
            SELECT id, name, description, agent_type, status, created_at, updated_at
            FROM agents
            WHERE name = $1
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = agent_row {
            self.find_by_id(&AgentId(row.id)).await
        } else {
            Ok(None)
        }
    }

    async fn find_all(&self) -> Result<Vec<Agent>> {
        let agent_rows = sqlx::query!(
            r#"
            SELECT id, name, description, agent_type, status, created_at, updated_at
            FROM agents
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut agents = Vec::new();
        for row in agent_rows {
            if let Some(agent) = self.find_by_id(&AgentId(row.id)).await? {
                agents.push(agent);
            }
        }

        Ok(agents)
    }

    async fn find_by_type(&self, agent_type: &AgentType) -> Result<Vec<Agent>> {
        let agent_rows = sqlx::query!(
            r#"
            SELECT id, name, description, agent_type, status, created_at, updated_at
            FROM agents
            WHERE agent_type = $1
            ORDER BY created_at DESC
            "#,
            serde_json::to_value(agent_type)?
        )
        .fetch_all(&self.pool)
        .await?;

        let mut agents = Vec::new();
        for row in agent_rows {
            if let Some(agent) = self.find_by_id(&AgentId(row.id)).await? {
                agents.push(agent);
            }
        }

        Ok(agents)
    }

    async fn find_by_status(&self, status: &AgentStatus) -> Result<Vec<Agent>> {
        let agent_rows = sqlx::query!(
            r#"
            SELECT id, name, description, agent_type, status, created_at, updated_at
            FROM agents
            WHERE status = $1
            ORDER BY created_at DESC
            "#,
            serde_json::to_value(status)?
        )
        .fetch_all(&self.pool)
        .await?;

        let mut agents = Vec::new();
        for row in agent_rows {
            if let Some(agent) = self.find_by_id(&AgentId(row.id)).await? {
                agents.push(agent);
            }
        }

        Ok(agents)
    }

    async fn update(&self, agent: &Agent) -> Result<Agent> {
        let mut tx = self.pool.begin().await?;

        // エージェントテーブルを更新
        sqlx::query!(
            r#"
            UPDATE agents
            SET name = $2, description = $3, agent_type = $4, status = $5, updated_at = $6
            WHERE id = $1
            "#,
            agent.id.0,
            agent.name,
            agent.description,
            serde_json::to_value(&agent.agent_type)?,
            serde_json::to_value(&agent.status)?,
            Utc::now()
        )
        .execute(&mut *tx)
        .await?;

        // 能力テーブルを更新（既存を削除して再挿入）
        sqlx::query!(
            r#"
            DELETE FROM agent_capabilities
            WHERE agent_id = $1
            "#,
            agent.id.0
        )
        .execute(&mut *tx)
        .await?;

        for capability in &agent.capabilities {
            sqlx::query!(
                r#"
                INSERT INTO agent_capabilities (agent_id, name, description, version, parameters)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                agent.id.0,
                capability.name,
                capability.description,
                capability.version,
                serde_json::to_value(&capability.parameters)?
            )
            .execute(&mut *tx)
            .await?;
        }

        // 設定テーブルを更新
        sqlx::query!(
            r#"
            UPDATE agent_configurations
            SET model_config = $2, execution_config = $3, security_config = $4
            WHERE agent_id = $1
            "#,
            agent.id.0,
            serde_json::to_value(&agent.configuration.model_config)?,
            serde_json::to_value(&agent.configuration.execution_config)?,
            serde_json::to_value(&agent.configuration.security_config)?
        )
        .execute(&mut *tx)
        .await?;

        // メタデータテーブルを更新（既存を削除して再挿入）
        sqlx::query!(
            r#"
            DELETE FROM agent_metadata
            WHERE agent_id = $1
            "#,
            agent.id.0
        )
        .execute(&mut *tx)
        .await?;

        for (key, value) in &agent.metadata {
            sqlx::query!(
                r#"
                INSERT INTO agent_metadata (agent_id, key, value)
                VALUES ($1, $2, $3)
                "#,
                agent.id.0,
                key,
                value
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(agent.clone())
    }

    async fn delete(&self, id: &AgentId) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        // 関連テーブルから削除
        sqlx::query!("DELETE FROM agent_metadata WHERE agent_id = $1", id.0)
            .execute(&mut *tx)
            .await?;

        sqlx::query!("DELETE FROM agent_capabilities WHERE agent_id = $1", id.0)
            .execute(&mut *tx)
            .await?;

        sqlx::query!("DELETE FROM agent_configurations WHERE agent_id = $1", id.0)
            .execute(&mut *tx)
            .await?;

        // エージェントテーブルから削除
        sqlx::query!("DELETE FROM agents WHERE id = $1", id.0)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn count(&self) -> Result<usize> {
        let row = sqlx::query!("SELECT COUNT(*) as count FROM agents")
            .fetch_one(&self.pool)
            .await?;

        Ok(row.count.unwrap_or(0) as usize)
    }
}

impl From<SqlxError> for Error {
    fn from(err: SqlxError) -> Self {
        match err {
            SqlxError::RowNotFound => Error::NotFound("Row not found".to_string()),
            SqlxError::Database(db_err) => {
                if let Some(code) = db_err.code() {
                    match code.as_ref() {
                        "23505" => Error::Conflict("Unique constraint violation".to_string()),
                        "23503" => Error::Conflict("Foreign key constraint violation".to_string()),
                        _ => Error::DatabaseError(err),
                    }
                } else {
                    Error::DatabaseError(err)
                }
            }
            _ => Error::DatabaseError(err),
        }
    }
}
