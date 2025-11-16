# Service 4: Integration Hub

## Overview

The Integration Hub is the gateway to external systems, responsible for orchestrating API calls to third-party services such as payment processors, banking systems, CRM platforms, and other backend services. It provides intelligent routing, caching, circuit breaking, and retry mechanisms to ensure reliable integration with external dependencies.

## Responsibilities

1. **External API Integration**
   - HTTP/REST API calls to external services
   - SOAP/XML integrations (legacy systems)
   - Database queries to external systems
   - Message queue consumers/producers

2. **Resilience Patterns**
   - Circuit breaker (fail-fast)
   - Retry with exponential backoff
   - Timeout management
   - Bulkhead isolation

3. **Performance Optimization**
   - Response caching (multi-level)
   - Request deduplication
   - Connection pooling
   - Rate limiting (outbound)

4. **Service Orchestration**
   - Sequential API calls
   - Parallel API calls with aggregation
   - Saga pattern for distributed transactions
   - Compensation logic for failures

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│           Integration Hub Service                        │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌────────────────────────────────────┐                 │
│  │     gRPC Server (Port 50053)       │                 │
│  │  - ExecuteAction                    │                 │
│  │  - GetBalance                       │                 │
│  │  - ValidatePin                      │                 │
│  │  - TransferMoney                    │                 │
│  └──────────────┬─────────────────────┘                 │
│                 ▼                                         │
│  ┌────────────────────────────────────┐                 │
│  │    Integration Orchestrator        │                 │
│  │  - Route to correct adapter         │                 │
│  │  - Apply resilience patterns        │                 │
│  │  - Handle errors gracefully         │                 │
│  └──────────────┬─────────────────────┘                 │
│                 ▼                                         │
│  ┌────────────────────────────────────┐                 │
│  │      Service Adapters              │                 │
│  │                                     │                 │
│  │  ┌──────────────────────────────┐ │                 │
│  │  │ Payment Adapter              │ │                 │
│  │  │  - M-Pesa, Airtel Money      │ │                 │
│  │  └──────────────────────────────┘ │                 │
│  │                                     │                 │
│  │  ┌──────────────────────────────┐ │                 │
│  │  │ Banking Adapter              │ │                 │
│  │  │  - Core Banking System       │ │                 │
│  │  └──────────────────────────────┘ │                 │
│  │                                     │                 │
│  │  ┌──────────────────────────────┐ │                 │
│  │  │ CRM Adapter                  │ │                 │
│  │  │  - Customer Data             │ │                 │
│  │  └──────────────────────────────┘ │                 │
│  │                                     │                 │
│  │  ┌──────────────────────────────┐ │                 │
│  │  │ Generic HTTP Adapter         │ │                 │
│  │  │  - REST APIs                 │ │                 │
│  │  └──────────────────────────────┘ │                 │
│  └──────────────┬─────────────────────┘                 │
│                 ▼                                         │
│  ┌────────────────────────────────────┐                 │
│  │    Resilience Layer                │                 │
│  │  - Circuit Breaker                  │                 │
│  │  - Retry Logic                      │                 │
│  │  - Timeout Control                  │                 │
│  │  - Bulkhead (Resource Isolation)    │                 │
│  └──────────────┬─────────────────────┘                 │
│                 ▼                                         │
│  ┌────────────────────────────────────┐                 │
│  │     HTTP Client Pool               │                 │
│  │  - Connection pooling               │                 │
│  │  - Keep-alive management            │                 │
│  │  - TLS/mTLS support                 │                 │
│  └──────────────┬─────────────────────┘                 │
│                 ▼                                         │
│  ┌────────────────────────────────────┐                 │
│  │      Response Cache                │                 │
│  │  L1: In-memory (Moka)              │                 │
│  │  L2: Redis                          │                 │
│  └────────────────────────────────────┘                 │
│                 │                                         │
│                 ▼                                         │
│       External Services                                  │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐                │
│  │ Payment  │ │  Banks   │ │   CRM    │                │
│  │  APIs    │ │  APIs    │ │  APIs    │                │
│  └──────────┘ └──────────┘ └──────────┘                │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

## Data Models

### Integration Request/Response
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationRequest {
    /// Unique request identifier
    pub request_id: String,

    /// Action to execute
    pub action: String,

    /// Parameters for the action
    pub parameters: HashMap<String, serde_json::Value>,

    /// Authentication context
    pub auth_context: Option<AuthContext>,

    /// Request metadata
    pub metadata: HashMap<String, String>,

    /// Timeout override (ms)
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationResponse {
    /// Request identifier (for correlation)
    pub request_id: String,

    /// Success flag
    pub success: bool,

    /// Response data
    pub data: serde_json::Value,

    /// Error information (if failed)
    pub error: Option<IntegrationError>,

    /// Response metadata
    pub metadata: HashMap<String, String>,

    /// Processing duration (ms)
    pub duration_ms: u64,

    /// Next menu ID (for menu engine)
    pub next_menu_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationError {
    pub code: String,
    pub message: String,
    pub retryable: bool,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthContext {
    pub msisdn: String,
    pub session_id: String,
    pub token: Option<String>,
}
```

### Service Configuration
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    /// Service identifier
    pub service_id: String,

    /// Service name
    pub name: String,

    /// Base URL
    pub base_url: String,

    /// Authentication method
    pub auth: AuthMethod,

    /// Timeout configuration
    pub timeout: TimeoutConfig,

    /// Retry configuration
    pub retry: RetryConfig,

    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerConfig,

    /// Cache configuration
    pub cache: CacheConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AuthMethod {
    None,
    BasicAuth { username: String, password: String },
    BearerToken { token: String },
    ApiKey { header: String, key: String },
    OAuth2 { client_id: String, client_secret: String, token_url: String },
    Custom { config: HashMap<String, String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    pub connect_timeout_ms: u64,
    pub request_timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_backoff_ms: u64,
    pub max_backoff_ms: u64,
    pub backoff_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub success_threshold: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub ttl_seconds: u64,
    pub cache_key_template: Option<String>,
}
```

## Core Components

### 1. gRPC Service Implementation

```rust
use tonic::{Request, Response, Status};
use integration_hub::integration_hub_server::{IntegrationHub, IntegrationHubServer};
use integration_hub::*;

pub struct IntegrationHubService {
    orchestrator: Arc<IntegrationOrchestrator>,
}

#[tonic::async_trait]
impl IntegrationHub for IntegrationHubService {
    #[instrument(skip(self))]
    async fn execute_action(
        &self,
        request: Request<ExecuteActionRequest>,
    ) -> Result<Response<ExecuteActionResponse>, Status> {
        let req = request.into_inner();

        let integration_req = IntegrationRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            action: req.action,
            parameters: serde_json::from_str(&req.parameters)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            auth_context: req.auth_context.map(|ctx| AuthContext {
                msisdn: ctx.msisdn,
                session_id: ctx.session_id,
                token: ctx.token,
            }),
            metadata: HashMap::new(),
            timeout_ms: req.timeout_ms,
        };

        let result = self.orchestrator
            .execute(integration_req)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ExecuteActionResponse {
            success: result.success,
            data: serde_json::to_string(&result.data).unwrap(),
            error_code: result.error.as_ref().map(|e| e.code.clone()),
            error_message: result.error.as_ref().map(|e| e.message.clone()),
            next_menu_id: result.next_menu_id,
        }))
    }

    #[instrument(skip(self))]
    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let msisdn = &request.into_inner().msisdn;

        let integration_req = IntegrationRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            action: "get_balance".to_string(),
            parameters: [("msisdn".to_string(), json!(msisdn))].into(),
            auth_context: None,
            metadata: HashMap::new(),
            timeout_ms: Some(5000),
        };

        let result = self.orchestrator.execute(integration_req).await
            .map_err(|e| Status::internal(e.to_string()))?;

        if !result.success {
            return Err(Status::internal("Failed to get balance"));
        }

        let balance = result.data.get("balance")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| Status::internal("Invalid balance response"))?;

        Ok(Response::new(GetBalanceResponse { balance }))
    }

    #[instrument(skip(self, pin))]
    async fn validate_pin(
        &self,
        request: Request<ValidatePinRequest>,
    ) -> Result<Response<ValidatePinResponse>, Status> {
        let req = request.into_inner();

        let integration_req = IntegrationRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            action: "validate_pin".to_string(),
            parameters: [
                ("msisdn".to_string(), json!(req.msisdn)),
                ("pin".to_string(), json!(req.pin)),
            ].into(),
            auth_context: None,
            metadata: HashMap::new(),
            timeout_ms: Some(3000),
        };

        let result = self.orchestrator.execute(integration_req).await
            .map_err(|e| Status::internal(e.to_string()))?;

        let valid = result.data.get("valid")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Ok(Response::new(ValidatePinResponse { valid }))
    }

    #[instrument(skip(self))]
    async fn transfer_money(
        &self,
        request: Request<TransferMoneyRequest>,
    ) -> Result<Response<TransferMoneyResponse>, Status> {
        let req = request.into_inner();

        let integration_req = IntegrationRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            action: "transfer_money".to_string(),
            parameters: [
                ("from_msisdn".to_string(), json!(req.from_msisdn)),
                ("to_msisdn".to_string(), json!(req.to_msisdn)),
                ("amount".to_string(), json!(req.amount)),
            ].into(),
            auth_context: None,
            metadata: HashMap::new(),
            timeout_ms: Some(30000), // Longer timeout for transfers
        };

        let result = self.orchestrator.execute(integration_req).await
            .map_err(|e| Status::internal(e.to_string()))?;

        if !result.success {
            let error = result.error.unwrap_or_else(|| IntegrationError {
                code: "UNKNOWN".to_string(),
                message: "Transfer failed".to_string(),
                retryable: false,
                details: None,
            });

            return Ok(Response::new(TransferMoneyResponse {
                success: false,
                transaction_id: None,
                error_message: Some(error.message),
            }));
        }

        let transaction_id = result.data.get("transaction_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(Response::new(TransferMoneyResponse {
            success: true,
            transaction_id,
            error_message: None,
        }))
    }
}
```

### 2. Integration Orchestrator

```rust
use tower::{ServiceBuilder, Service, ServiceExt};
use tower::limit::RateLimitLayer;
use tower::timeout::TimeoutLayer;

pub struct IntegrationOrchestrator {
    adapters: Arc<HashMap<String, Box<dyn ServiceAdapter>>>,
    cache: Arc<ResponseCache>,
    circuit_breaker_registry: Arc<CircuitBreakerRegistry>,
}

impl IntegrationOrchestrator {
    #[instrument(skip(self))]
    pub async fn execute(
        &self,
        request: IntegrationRequest,
    ) -> Result<IntegrationResponse> {
        let start = std::time::Instant::now();

        // Check cache first
        if let Some(cached) = self.check_cache(&request).await? {
            info!("Cache hit for action: {}", request.action);
            return Ok(cached);
        }

        // Get appropriate adapter
        let adapter = self.get_adapter(&request.action)?;

        // Get circuit breaker for this service
        let circuit_breaker = self.circuit_breaker_registry
            .get_or_create(&adapter.service_id());

        // Check circuit breaker state
        if circuit_breaker.is_open() {
            warn!("Circuit breaker open for service: {}", adapter.service_id());
            return Ok(IntegrationResponse {
                request_id: request.request_id.clone(),
                success: false,
                data: json!({}),
                error: Some(IntegrationError {
                    code: "CIRCUIT_BREAKER_OPEN".to_string(),
                    message: "Service temporarily unavailable".to_string(),
                    retryable: true,
                    details: None,
                }),
                metadata: HashMap::new(),
                duration_ms: start.elapsed().as_millis() as u64,
                next_menu_id: None,
            });
        }

        // Execute with retry logic
        let result = self.execute_with_retry(adapter, &request).await;

        // Update circuit breaker
        match &result {
            Ok(response) if response.success => {
                circuit_breaker.record_success();
            }
            _ => {
                circuit_breaker.record_failure();
            }
        }

        // Cache successful responses
        if let Ok(response) = &result {
            if response.success {
                self.cache_response(&request, response).await?;
            }
        }

        let mut response = result?;
        response.duration_ms = start.elapsed().as_millis() as u64;

        Ok(response)
    }

    async fn execute_with_retry(
        &self,
        adapter: &dyn ServiceAdapter,
        request: &IntegrationRequest,
    ) -> Result<IntegrationResponse> {
        let config = adapter.get_config();
        let mut attempt = 0;
        let mut backoff = config.retry.initial_backoff_ms;

        loop {
            attempt += 1;

            match adapter.execute(request).await {
                Ok(response) => {
                    info!("Request succeeded on attempt {}", attempt);
                    return Ok(response);
                }
                Err(e) if attempt >= config.retry.max_retries => {
                    error!("Request failed after {} attempts: {}", attempt, e);
                    return Err(e);
                }
                Err(e) => {
                    warn!("Request failed on attempt {}: {}", attempt, e);

                    // Check if error is retryable
                    if !Self::is_retryable(&e) {
                        return Err(e);
                    }

                    // Exponential backoff
                    tokio::time::sleep(Duration::from_millis(backoff)).await;
                    backoff = (backoff as f64 * config.retry.backoff_multiplier) as u64;
                    backoff = backoff.min(config.retry.max_backoff_ms);
                }
            }
        }
    }

    fn is_retryable(error: &anyhow::Error) -> bool {
        // Network errors, timeouts, 5xx errors are retryable
        // 4xx client errors are not retryable
        error.to_string().contains("timeout") ||
        error.to_string().contains("connection") ||
        error.to_string().contains("503") ||
        error.to_string().contains("502")
    }

    fn get_adapter(&self, action: &str) -> Result<&dyn ServiceAdapter> {
        // Map action to adapter
        let service_id = match action {
            "get_balance" | "transfer_money" => "banking_service",
            "validate_pin" => "auth_service",
            "send_mpesa" => "mpesa_service",
            _ => "default_http",
        };

        self.adapters.get(service_id)
            .map(|a| a.as_ref())
            .ok_or_else(|| anyhow::anyhow!("No adapter found for action: {}", action))
    }

    async fn check_cache(
        &self,
        request: &IntegrationRequest,
    ) -> Result<Option<IntegrationResponse>> {
        let cache_key = self.generate_cache_key(request);
        self.cache.get(&cache_key).await
    }

    async fn cache_response(
        &self,
        request: &IntegrationRequest,
        response: &IntegrationResponse,
    ) -> Result<()> {
        let cache_key = self.generate_cache_key(request);
        self.cache.set(&cache_key, response.clone()).await
    }

    fn generate_cache_key(&self, request: &IntegrationRequest) -> String {
        use sha2::{Sha256, Digest};

        let data = format!(
            "{}:{}",
            request.action,
            serde_json::to_string(&request.parameters).unwrap_or_default()
        );

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("integration:{}", hex::encode(hasher.finalize()))
    }
}
```

### 3. Service Adapters

```rust
#[async_trait]
pub trait ServiceAdapter: Send + Sync {
    /// Get service identifier
    fn service_id(&self) -> &str;

    /// Get service configuration
    fn get_config(&self) -> &ServiceConfig;

    /// Execute integration request
    async fn execute(&self, request: &IntegrationRequest) -> Result<IntegrationResponse>;
}

/// Generic HTTP adapter for REST APIs
pub struct HttpServiceAdapter {
    service_id: String,
    config: ServiceConfig,
    client: reqwest::Client,
}

impl HttpServiceAdapter {
    pub fn new(service_id: String, config: ServiceConfig) -> Result<Self> {
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_millis(config.timeout.connect_timeout_ms))
            .timeout(Duration::from_millis(config.timeout.request_timeout_ms))
            .pool_max_idle_per_host(50)
            .pool_idle_timeout(Duration::from_secs(90))
            .build()?;

        Ok(Self {
            service_id,
            config,
            client,
        })
    }
}

#[async_trait]
impl ServiceAdapter for HttpServiceAdapter {
    fn service_id(&self) -> &str {
        &self.service_id
    }

    fn get_config(&self) -> &ServiceConfig {
        &self.config
    }

    async fn execute(&self, request: &IntegrationRequest) -> Result<IntegrationResponse> {
        // Build URL
        let url = format!("{}/{}", self.config.base_url, request.action);

        // Build request
        let mut req_builder = self.client.post(&url);

        // Add authentication
        req_builder = self.add_auth(req_builder);

        // Add body
        req_builder = req_builder.json(&request.parameters);

        // Send request
        let response = req_builder.send().await?;

        // Check status
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();

            return Ok(IntegrationResponse {
                request_id: request.request_id.clone(),
                success: false,
                data: json!({}),
                error: Some(IntegrationError {
                    code: format!("HTTP_{}", status.as_u16()),
                    message: error_text,
                    retryable: status.is_server_error(),
                    details: None,
                }),
                metadata: HashMap::new(),
                duration_ms: 0,
                next_menu_id: None,
            });
        }

        // Parse response
        let data: serde_json::Value = response.json().await?;

        Ok(IntegrationResponse {
            request_id: request.request_id.clone(),
            success: true,
            data,
            error: None,
            metadata: HashMap::new(),
            duration_ms: 0,
            next_menu_id: None,
        })
    }
}

impl HttpServiceAdapter {
    fn add_auth(&self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match &self.config.auth {
            AuthMethod::None => builder,
            AuthMethod::BasicAuth { username, password } => {
                builder.basic_auth(username, Some(password))
            }
            AuthMethod::BearerToken { token } => {
                builder.bearer_auth(token)
            }
            AuthMethod::ApiKey { header, key } => {
                builder.header(header, key)
            }
            AuthMethod::OAuth2 { .. } => {
                // Implement OAuth2 flow
                builder
            }
            AuthMethod::Custom { config } => {
                for (key, value) in config {
                    builder = builder.header(key, value);
                }
                builder
            }
        }
    }
}
```

### 4. Circuit Breaker

```rust
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

pub struct CircuitBreaker {
    state: Arc<RwLock<CircuitState>>,
    failure_count: AtomicU32,
    success_count: AtomicU32,
    last_failure_time: AtomicU64,
    config: CircuitBreakerConfig,
}

#[derive(Debug, Clone, PartialEq)]
enum CircuitState {
    Closed,  // Normal operation
    Open,    // Failing, reject requests
    HalfOpen, // Testing if service recovered
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            failure_count: AtomicU32::new(0),
            success_count: AtomicU32::new(0),
            last_failure_time: AtomicU64::new(0),
            config,
        }
    }

    pub fn is_open(&self) -> bool {
        let state = self.state.read().unwrap();
        *state == CircuitState::Open
    }

    pub fn record_success(&self) {
        let mut state = self.state.write().unwrap();

        match *state {
            CircuitState::HalfOpen => {
                let successes = self.success_count.fetch_add(1, Ordering::SeqCst) + 1;

                if successes >= self.config.success_threshold {
                    info!("Circuit breaker closing (service recovered)");
                    *state = CircuitState::Closed;
                    self.failure_count.store(0, Ordering::SeqCst);
                    self.success_count.store(0, Ordering::SeqCst);
                }
            }
            CircuitState::Closed => {
                // Reset failure count on success
                self.failure_count.store(0, Ordering::SeqCst);
            }
            _ => {}
        }
    }

    pub fn record_failure(&self) {
        let mut state = self.state.write().unwrap();

        match *state {
            CircuitState::Closed => {
                let failures = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;

                if failures >= self.config.failure_threshold {
                    warn!("Circuit breaker opening (too many failures)");
                    *state = CircuitState::Open;
                    self.last_failure_time.store(
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                        Ordering::SeqCst
                    );
                }
            }
            CircuitState::Open => {
                // Check if timeout has elapsed
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let last_failure = self.last_failure_time.load(Ordering::SeqCst);

                if now - last_failure >= self.config.timeout_seconds {
                    info!("Circuit breaker entering half-open state");
                    *state = CircuitState::HalfOpen;
                    self.success_count.store(0, Ordering::SeqCst);
                }
            }
            CircuitState::HalfOpen => {
                // Failed in half-open state, go back to open
                warn!("Circuit breaker re-opening (half-open test failed)");
                *state = CircuitState::Open;
                self.last_failure_time.store(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    Ordering::SeqCst
                );
            }
        }
    }
}

pub struct CircuitBreakerRegistry {
    breakers: Arc<DashMap<String, Arc<CircuitBreaker>>>,
}

impl CircuitBreakerRegistry {
    pub fn new() -> Self {
        Self {
            breakers: Arc::new(DashMap::new()),
        }
    }

    pub fn get_or_create(&self, service_id: &str) -> Arc<CircuitBreaker> {
        self.breakers.entry(service_id.to_string())
            .or_insert_with(|| {
                Arc::new(CircuitBreaker::new(CircuitBreakerConfig {
                    failure_threshold: 5,
                    success_threshold: 2,
                    timeout_seconds: 60,
                }))
            })
            .clone()
    }
}
```

### 5. Response Cache

```rust
use moka::future::Cache;

pub struct ResponseCache {
    l1_cache: Cache<String, IntegrationResponse>,
    redis_pool: Arc<deadpool_redis::Pool>,
}

impl ResponseCache {
    pub fn new(redis_pool: Arc<deadpool_redis::Pool>) -> Self {
        let l1_cache = Cache::builder()
            .max_capacity(10_000)
            .time_to_live(Duration::from_secs(300))
            .build();

        Self {
            l1_cache,
            redis_pool,
        }
    }

    pub async fn get(&self, key: &str) -> Result<Option<IntegrationResponse>> {
        // Try L1 cache
        if let Some(response) = self.l1_cache.get(key).await {
            return Ok(Some(response));
        }

        // Try Redis
        let mut conn = self.redis_pool.get().await?;
        let result: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut *conn)
            .await?;

        if let Some(json) = result {
            let response: IntegrationResponse = serde_json::from_str(&json)?;
            // Populate L1 cache
            self.l1_cache.insert(key.to_string(), response.clone()).await;
            return Ok(Some(response));
        }

        Ok(None)
    }

    pub async fn set(&self, key: &str, response: IntegrationResponse) -> Result<()> {
        // Set in L1 cache
        self.l1_cache.insert(key.to_string(), response.clone()).await;

        // Set in Redis
        let mut conn = self.redis_pool.get().await?;
        let json = serde_json::to_string(&response)?;

        redis::cmd("SETEX")
            .arg(key)
            .arg(300) // 5 minutes TTL
            .arg(json)
            .query_async(&mut *conn)
            .await?;

        Ok(())
    }
}
```

## Configuration

```toml
[service]
name = "integration-hub"
port = 50053
metrics_port = 9093

[redis]
url = "redis://redis:6379"
pool_size = 50

[cache]
l1_max_size = 10000
l1_ttl_seconds = 300
l2_ttl_seconds = 600

[circuit_breaker]
failure_threshold = 5
success_threshold = 2
timeout_seconds = 60

[services.banking]
name = "Core Banking System"
base_url = "https://api.bank.example.com"
auth = { type = "BasicAuth", username = "ussd_gw", password = "${BANKING_PASSWORD}" }
timeout = { connect_timeout_ms = 5000, request_timeout_ms = 30000 }
retry = { max_retries = 3, initial_backoff_ms = 1000, max_backoff_ms = 10000, backoff_multiplier = 2.0 }

[services.mpesa]
name = "M-Pesa API"
base_url = "https://api.safaricom.co.ke"
auth = { type = "OAuth2", client_id = "${MPESA_CLIENT_ID}", client_secret = "${MPESA_SECRET}", token_url = "https://api.safaricom.co.ke/oauth/v1/generate" }
timeout = { connect_timeout_ms = 3000, request_timeout_ms = 15000 }
retry = { max_retries = 2, initial_backoff_ms = 500, max_backoff_ms = 5000, backoff_multiplier = 2.0 }

[services.crm]
name = "Customer CRM"
base_url = "https://crm.example.com/api"
auth = { type = "ApiKey", header = "X-API-Key", key = "${CRM_API_KEY}" }
timeout = { connect_timeout_ms = 2000, request_timeout_ms = 10000 }
cache = { enabled = true, ttl_seconds = 600 }
```

## Monitoring

```rust
use prometheus::{IntCounter, Histogram};

lazy_static! {
    static ref EXTERNAL_REQUESTS: IntCounter = register_int_counter!(
        "integration_hub_requests_total",
        "Total external API requests"
    ).unwrap();

    static ref EXTERNAL_FAILURES: IntCounter = register_int_counter!(
        "integration_hub_failures_total",
        "Total external API failures"
    ).unwrap();

    static ref CIRCUIT_BREAKER_OPENS: IntCounter = register_int_counter!(
        "integration_hub_circuit_breaker_opens_total",
        "Total circuit breaker opens"
    ).unwrap();

    static ref EXTERNAL_LATENCY: Histogram = register_histogram!(
        "integration_hub_request_duration_seconds",
        "External API request duration"
    ).unwrap();
}
```

## Database Schema

```sql
CREATE TABLE integration_logs (
    id BIGSERIAL PRIMARY KEY,
    request_id VARCHAR(64) NOT NULL,
    action VARCHAR(100) NOT NULL,
    service_id VARCHAR(100) NOT NULL,
    success BOOLEAN NOT NULL,
    duration_ms INTEGER NOT NULL,
    error_code VARCHAR(50),
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    INDEX idx_request_id (request_id),
    INDEX idx_action (action),
    INDEX idx_created_at (created_at)
);

CREATE TABLE integration_metrics (
    id BIGSERIAL PRIMARY KEY,
    service_id VARCHAR(100) NOT NULL,
    metric_type VARCHAR(50) NOT NULL,
    value DOUBLE PRECISION NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

---

## Summary

The Integration Hub provides a robust, resilient gateway to external services with:

- **Circuit breakers** to prevent cascade failures
- **Intelligent caching** for performance
- **Retry logic** with exponential backoff
- **Connection pooling** for efficiency
- **Comprehensive monitoring** and logging

This completes the 4-microservice architecture for the next-level Rust-based USSD Gateway!

---

**See Also**:
- [Architecture Overview](./00-ARCHITECTURE-OVERVIEW.md)
- [Protocol Gateway](./01-PROTOCOL-GATEWAY-SERVICE.md)
- [Session Manager](./02-SESSION-MANAGER-SERVICE.md)
- [Menu Engine](./03-MENU-ENGINE-SERVICE.md)
