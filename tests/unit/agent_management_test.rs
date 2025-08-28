use autodoc_test::domain::*;
use autodoc_test::usecase::agent_management::*;
use autodoc_test::shared::error::Result;
use async_trait::async_trait;
use std::collections::HashMap;

// モックリポジトリ
struct MockAgentRepository {
    agents: HashMap<AgentId, Agent>,
}

impl MockAgentRepository {
    fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }
}

#[async_trait]
impl AgentRepository for MockAgentRepository {
    async fn create(&self, agent: &Agent) -> Result<Agent> {
        Ok(agent.clone())
    }

    async fn find_by_id(&self, id: &AgentId) -> Result<Option<Agent>> {
        Ok(self.agents.get(id).cloned())
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Agent>> {
        Ok(self.agents.values().find(|a| a.name == name).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Agent>> {
        Ok(self.agents.values().cloned().collect())
    }

    async fn find_by_type(&self, agent_type: &AgentType) -> Result<Vec<Agent>> {
        Ok(self.agents.values()
            .filter(|a| &a.agent_type == agent_type)
            .cloned()
            .collect())
    }

    async fn find_by_status(&self, status: &AgentStatus) -> Result<Vec<Agent>> {
        Ok(self.agents.values()
            .filter(|a| &a.status == status)
            .cloned()
            .collect())
    }

    async fn update(&self, agent: &Agent) -> Result<Agent> {
        Ok(agent.clone())
    }

    async fn delete(&self, id: &AgentId) -> Result<()> {
        Ok(())
    }

    async fn count(&self) -> Result<usize> {
        Ok(self.agents.len())
    }
}

// モックサービス
struct MockAgentManagementService;

impl MockAgentManagementService {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AgentManagementService for MockAgentManagementService {
    async fn create_agent(&self, request: CreateAgentRequest) -> Result<Agent> {
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

    async fn update_agent_status(&self, agent_id: &AgentId, status: AgentStatus) -> Result<Agent> {
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

    async fn add_capability(&self, agent_id: &AgentId, capability: Capability) -> Result<Agent> {
        self.update_agent_status(agent_id, AgentStatus::Active).await
    }

    async fn remove_capability(&self, agent_id: &AgentId, _capability_name: &str) -> Result<Agent> {
        self.update_agent_status(agent_id, AgentStatus::Active).await
    }

    async fn update_configuration(&self, agent_id: &AgentId, _config: AgentConfiguration) -> Result<Agent> {
        self.update_agent_status(agent_id, AgentStatus::Active).await
    }

    async fn validate_agent_configuration(&self, _config: &AgentConfiguration) -> Result<bool> {
        Ok(true)
    }
}

// モックセキュリティサービス
struct MockSecurityService;

impl MockSecurityService {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SecurityService for MockSecurityService {
    async fn authenticate_agent(&self, _credentials: &AgentCredentials) -> Result<AuthenticationResult> {
        Ok(AuthenticationResult {
            authenticated: true,
            agent_id: Some(AgentId::new()),
            permissions: vec!["read".to_string(), "write".to_string()],
            expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
        })
    }

    async fn authorize_action(&self, _agent_id: &AgentId, _action: &str, _resource: &str) -> Result<bool> {
        Ok(true)
    }

    async fn validate_api_key(&self, _api_key: &str) -> Result<Option<AgentId>> {
        Some(AgentId::new()).into()
    }

    async fn encrypt_sensitive_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        Ok(data.to_vec())
    }

    async fn decrypt_sensitive_data(&self, encrypted_data: &[u8]) -> Result<Vec<u8>> {
        Ok(encrypted_data.to_vec())
    }
}

#[tokio::test]
async fn test_create_agent() {
    let repo = MockAgentRepository::new();
    let service = MockAgentManagementService::new();
    let security = MockSecurityService::new();
    
    let use_case = AgentManagementUseCase::new(
        Box::new(repo),
        Box::new(service),
        Box::new(security),
    );

    let request = CreateAgentRequest {
        name: "Test Agent".to_string(),
        description: "A test agent".to_string(),
        agent_type: AgentType::Conversational,
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
    };

    let result = use_case.create_agent(request).await;
    assert!(result.is_ok());
    
    let agent = result.unwrap();
    assert_eq!(agent.name, "Test Agent");
    assert_eq!(agent.description, "A test agent");
    assert!(matches!(agent.agent_type, AgentType::Conversational));
    assert!(matches!(agent.status, AgentStatus::Active));
}

#[tokio::test]
async fn test_update_agent_status() {
    let repo = MockAgentRepository::new();
    let service = MockAgentManagementService::new();
    let security = MockSecurityService::new();
    
    let use_case = AgentManagementUseCase::new(
        Box::new(repo),
        Box::new(service),
        Box::new(security),
    );

    let agent_id = AgentId::new();
    let new_status = AgentStatus::Inactive;

    let result = use_case.update_agent_status(&agent_id, new_status).await;
    assert!(result.is_ok());
    
    let agent = result.unwrap();
    assert_eq!(agent.id, agent_id);
    assert!(matches!(agent.status, AgentStatus::Inactive));
}

#[tokio::test]
async fn test_add_capability() {
    let repo = MockAgentRepository::new();
    let service = MockAgentManagementService::new();
    let security = MockSecurityService::new();
    
    let use_case = AgentManagementUseCase::new(
        Box::new(repo),
        Box::new(service),
        Box::new(security),
    );

    let agent_id = AgentId::new();
    let capability = Capability {
        name: "text_generation".to_string(),
        description: "Generate text".to_string(),
        version: "1.0".to_string(),
        parameters: HashMap::new(),
    };

    let result = use_case.add_capability(&agent_id, capability).await;
    assert!(result.is_ok());
    
    let agent = result.unwrap();
    assert_eq!(agent.id, agent_id);
}

#[tokio::test]
async fn test_remove_capability() {
    let repo = MockAgentRepository::new();
    let service = MockAgentManagementService::new();
    let security = MockSecurityService::new();
    
    let use_case = AgentManagementUseCase::new(
        Box::new(repo),
        Box::new(service),
        Box::new(security),
    );

    let agent_id = AgentId::new();
    let capability_name = "text_generation";

    let result = use_case.remove_capability(&agent_id, capability_name).await;
    assert!(result.is_ok());
    
    let agent = result.unwrap();
    assert_eq!(agent.id, agent_id);
}

#[tokio::test]
async fn test_update_agent_configuration() {
    let repo = MockAgentRepository::new();
    let service = MockAgentManagementService::new();
    let security = MockSecurityService::new();
    
    let use_case = AgentManagementUseCase::new(
        Box::new(repo),
        Box::new(service),
        Box::new(security),
    );

    let agent_id = AgentId::new();
    let new_config = AgentConfiguration {
        model_config: ModelConfiguration {
            model_name: "updated".to_string(),
            model_version: "2.0".to_string(),
            parameters: HashMap::new(),
            context_window: 2000,
        },
        execution_config: ExecutionConfiguration {
            max_concurrent_tasks: 20,
            timeout_seconds: 60,
            retry_attempts: 5,
            memory_limit_mb: 200,
        },
        security_config: SecurityConfiguration {
            api_key_required: true,
            rate_limit: Some(100),
            allowed_ips: vec!["127.0.0.1".to_string()],
            encryption_enabled: true,
        },
    };

    let result = use_case.update_agent_configuration(&agent_id, new_config).await;
    assert!(result.is_ok());
    
    let agent = result.unwrap();
    assert_eq!(agent.id, agent_id);
}

#[tokio::test]
async fn test_delete_agent() {
    let repo = MockAgentRepository::new();
    let service = MockAgentManagementService::new();
    let security = MockSecurityService::new();
    
    let use_case = AgentManagementUseCase::new(
        Box::new(repo),
        Box::new(service),
        Box::new(security),
    );

    let agent_id = AgentId::new();

    let result = use_case.delete_agent(&agent_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_agent_statistics() {
    let repo = MockAgentRepository::new();
    let service = MockAgentManagementService::new();
    let security = MockSecurityService::new();
    
    let use_case = AgentManagementUseCase::new(
        Box::new(repo),
        Box::new(service),
        Box::new(security),
    );

    let result = use_case.get_agent_statistics().await;
    assert!(result.is_ok());
    
    let stats = result.unwrap();
    assert_eq!(stats.total_agents, 0);
    assert_eq!(stats.active_agents, 0);
    assert_eq!(stats.inactive_agents, 0);
    assert_eq!(stats.training_agents, 0);
    assert_eq!(stats.error_agents, 0);
}
