# Service 2: Session Manager

## Overview

The Session Manager is responsible for maintaining USSD session state across the distributed system. It provides high-performance session storage, lifecycle management, and ensures session consistency across multiple gateway instances.

## Responsibilities

1. **Session Lifecycle Management**
   - Create new sessions
   - Update session state
   - Handle session expiration/timeout
   - Clean up terminated sessions

2. **Distributed State Storage**
   - Redis-backed session persistence
   - In-memory caching for hot sessions
   - Multi-level caching strategy (L1/L2)

3. **Session Routing**
   - Maintain session affinity
   - Route requests to appropriate Menu Engine
   - Handle session migration

4. **Fault Tolerance**
   - Session recovery on service restart
   - Redis cluster support for HA
   - Graceful degradation

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│            Session Manager Service                       │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌────────────────────────────────────┐                 │
│  │       gRPC Server (Port 50051)     │                 │
│  │  - CreateSession                    │                 │
│  │  - UpdateSession                    │                 │
│  │  - GetSession                       │                 │
│  │  - DeleteSession                    │                 │
│  └──────────────┬─────────────────────┘                 │
│                 ▼                                         │
│  ┌────────────────────────────────────┐                 │
│  │      Session Service Layer         │                 │
│  │  - Validation                       │                 │
│  │  - Business logic                   │                 │
│  │  - Event publishing                 │                 │
│  └──────────────┬─────────────────────┘                 │
│                 ▼                                         │
│  ┌────────────────────────────────────┐                 │
│  │    Multi-Level Cache Manager       │                 │
│  │                                     │                 │
│  │  L1: DashMap (In-Memory)           │                 │
│  │      - Hot sessions                 │                 │
│  │      - <1ms access                  │                 │
│  │                                     │                 │
│  │  L2: Redis (Distributed)           │                 │
│  │      - All active sessions          │                 │
│  │      - 1-5ms access                 │                 │
│  │                                     │                 │
│  │  L3: PostgreSQL (Persistent)       │                 │
│  │      - Historical data              │                 │
│  │      - Analytics                    │                 │
│  └──────────────┬─────────────────────┘                 │
│                 ▼                                         │
│  ┌────────────────────────────────────┐                 │
│  │     Background Tasks               │                 │
│  │  - Session cleanup (TTL)           │                 │
│  │  - Health checks                    │                 │
│  │  - Metrics collection               │                 │
│  └────────────────────────────────────┘                 │
│                 │                                         │
│                 ▼                                         │
│  ┌────────────────────────────────────┐                 │
│  │      gRPC Client Pool              │                 │
│  │      → Menu Engine                 │                 │
│  └────────────────────────────────────┘                 │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

## Data Models

### Session State
```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique session identifier
    pub session_id: String,

    /// User's MSISDN in E.164 format
    pub msisdn: String,

    /// Operator/network code
    pub operator_code: String,

    /// USSD service code
    pub service_code: String,

    /// Current session state
    pub state: SessionState,

    /// Current menu context
    pub menu_context: MenuContext,

    /// User input history
    pub input_history: Vec<String>,

    /// Session metadata
    pub metadata: HashMap<String, String>,

    /// Session creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,

    /// Session expiration time
    pub expires_at: DateTime<Utc>,

    /// Number of interactions
    pub interaction_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionState {
    /// Session just created
    Active,
    /// Waiting for user input
    WaitingForInput,
    /// Processing request
    Processing,
    /// Session ended normally
    Completed,
    /// Session timed out
    TimedOut,
    /// User cancelled
    Cancelled,
    /// Error state
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuContext {
    /// Current menu ID
    pub current_menu_id: String,

    /// Previous menu ID (for back navigation)
    pub previous_menu_id: Option<String>,

    /// Menu navigation stack
    pub menu_stack: Vec<String>,

    /// Current page (for paginated menus)
    pub current_page: u32,

    /// User variables/context
    pub variables: HashMap<String, serde_json::Value>,
}

impl Session {
    pub fn new(msisdn: String, operator_code: String, service_code: String) -> Self {
        let now = Utc::now();
        let session_id = Self::generate_session_id(&msisdn);

        Self {
            session_id,
            msisdn,
            operator_code,
            service_code,
            state: SessionState::Active,
            menu_context: MenuContext::default(),
            input_history: Vec::new(),
            metadata: HashMap::new(),
            created_at: now,
            last_activity: now,
            expires_at: now + chrono::Duration::minutes(5),
            interaction_count: 0,
        }
    }

    pub fn generate_session_id(msisdn: &str) -> String {
        use sha2::{Sha256, Digest};

        let timestamp = Utc::now().timestamp_millis();
        let data = format!("{}{}{}", msisdn, timestamp, uuid::Uuid::new_v4());

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        format!("sess_{}", hex::encode(&result[..16]))
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn update_activity(&mut self) {
        self.last_activity = Utc::now();
        self.expires_at = Utc::now() + chrono::Duration::minutes(5);
        self.interaction_count += 1;
    }
}
```

## Core Components

### 1. gRPC Service Implementation

```rust
use tonic::{Request, Response, Status};
use session_manager::session_manager_server::{SessionManager, SessionManagerServer};
use session_manager::{
    CreateSessionRequest, CreateSessionResponse,
    UpdateSessionRequest, UpdateSessionResponse,
    GetSessionRequest, GetSessionResponse,
    DeleteSessionRequest, DeleteSessionResponse,
};

pub struct SessionManagerService {
    cache_manager: Arc<CacheManager>,
    menu_client_pool: Arc<MenuEngineClientPool>,
    event_publisher: Arc<EventPublisher>,
}

#[tonic::async_trait]
impl SessionManager for SessionManagerService {
    #[instrument(skip(self), fields(msisdn = %request.get_ref().msisdn))]
    async fn create_session(
        &self,
        request: Request<CreateSessionRequest>,
    ) -> Result<Response<CreateSessionResponse>, Status> {
        let req = request.into_inner();

        // Create new session
        let session = Session::new(
            req.msisdn,
            req.operator_code,
            req.service_code,
        );

        // Store in cache
        self.cache_manager.set(&session.session_id, &session).await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Publish session created event
        self.event_publisher.publish_session_created(&session).await;

        info!("Session created: {}", session.session_id);

        Ok(Response::new(CreateSessionResponse {
            session_id: session.session_id,
            expires_at: session.expires_at.timestamp(),
        }))
    }

    #[instrument(skip(self), fields(session_id = %request.get_ref().session_id))]
    async fn update_session(
        &self,
        request: Request<UpdateSessionRequest>,
    ) -> Result<Response<UpdateSessionResponse>, Status> {
        let req = request.into_inner();

        // Get existing session
        let mut session = self.cache_manager
            .get(&req.session_id)
            .await
            .map_err(|e| Status::not_found(e.to_string()))?
            .ok_or_else(|| Status::not_found("Session not found"))?;

        // Check if expired
        if session.is_expired() {
            session.state = SessionState::TimedOut;
            self.cache_manager.delete(&session.session_id).await?;
            return Err(Status::deadline_exceeded("Session expired"));
        }

        // Update session
        session.update_activity();
        session.input_history.push(req.user_input.clone());

        // Forward to Menu Engine for processing
        let menu_response = self.menu_client_pool
            .get_client()
            .process_menu(MenuRequest {
                session: session.clone(),
                user_input: req.user_input,
            })
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .into_inner();

        // Update menu context
        session.menu_context = menu_response.menu_context;
        session.state = SessionState::WaitingForInput;

        // Save updated session
        self.cache_manager.set(&session.session_id, &session).await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Publish session updated event
        self.event_publisher.publish_session_updated(&session).await;

        Ok(Response::new(UpdateSessionResponse {
            session_id: session.session_id,
            menu_text: menu_response.menu_text,
            should_continue: menu_response.should_continue,
        }))
    }

    #[instrument(skip(self))]
    async fn get_session(
        &self,
        request: Request<GetSessionRequest>,
    ) -> Result<Response<GetSessionResponse>, Status> {
        let session_id = &request.into_inner().session_id;

        let session = self.cache_manager
            .get(session_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Session not found"))?;

        Ok(Response::new(GetSessionResponse {
            session: Some(session.into()),
        }))
    }

    #[instrument(skip(self))]
    async fn delete_session(
        &self,
        request: Request<DeleteSessionRequest>,
    ) -> Result<Response<DeleteSessionResponse>, Status> {
        let req = request.into_inner();

        self.cache_manager.delete(&req.session_id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Publish session deleted event
        self.event_publisher.publish_session_deleted(&req.session_id).await;

        info!("Session deleted: {}", req.session_id);

        Ok(Response::new(DeleteSessionResponse {
            success: true,
        }))
    }
}
```

### 2. Multi-Level Cache Manager

```rust
use dashmap::DashMap;
use redis::aio::ConnectionManager;
use moka::future::Cache;
use std::sync::Arc;

pub struct CacheManager {
    // L1: In-memory cache (hot sessions)
    l1_cache: Arc<DashMap<String, Session>>,

    // L2: Redis (distributed cache)
    redis_pool: Arc<deadpool_redis::Pool>,

    // L3: PostgreSQL (persistent storage)
    db_pool: Arc<sqlx::PgPool>,

    // Moka cache for additional performance
    moka_cache: Cache<String, Session>,
}

impl CacheManager {
    pub fn new(
        redis_pool: Arc<deadpool_redis::Pool>,
        db_pool: Arc<sqlx::PgPool>,
    ) -> Self {
        Self {
            l1_cache: Arc::new(DashMap::new()),
            redis_pool,
            db_pool,
            moka_cache: Cache::builder()
                .max_capacity(100_000) // 100K sessions in memory
                .time_to_live(Duration::from_secs(300)) // 5 min TTL
                .build(),
        }
    }

    #[instrument(skip(self, session))]
    pub async fn set(&self, session_id: &str, session: &Session) -> Result<()> {
        // Write to all cache levels in parallel
        let l1_fut = async {
            self.l1_cache.insert(session_id.to_string(), session.clone());
            self.moka_cache.insert(session_id.to_string(), session.clone()).await;
            Ok::<_, anyhow::Error>(())
        };

        let l2_fut = self.set_redis(session_id, session);
        let l3_fut = self.set_postgres(session_id, session);

        tokio::try_join!(l1_fut, l2_fut, l3_fut)?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get(&self, session_id: &str) -> Result<Option<Session>> {
        // Try L1 cache first (DashMap)
        if let Some(session) = self.l1_cache.get(session_id) {
            debug!("Cache hit: L1 (DashMap)");
            return Ok(Some(session.clone()));
        }

        // Try Moka cache
        if let Some(session) = self.moka_cache.get(session_id).await {
            debug!("Cache hit: L1 (Moka)");
            // Populate DashMap
            self.l1_cache.insert(session_id.to_string(), session.clone());
            return Ok(Some(session));
        }

        // Try L2 cache (Redis)
        if let Some(session) = self.get_redis(session_id).await? {
            debug!("Cache hit: L2 (Redis)");
            // Populate L1 caches
            self.l1_cache.insert(session_id.to_string(), session.clone());
            self.moka_cache.insert(session_id.to_string(), session.clone()).await;
            return Ok(Some(session));
        }

        // Try L3 (PostgreSQL)
        if let Some(session) = self.get_postgres(session_id).await? {
            debug!("Cache hit: L3 (PostgreSQL)");
            // Populate upper cache levels
            self.set(session_id, &session).await?;
            return Ok(Some(session));
        }

        debug!("Cache miss: session not found");
        Ok(None)
    }

    async fn set_redis(&self, session_id: &str, session: &Session) -> Result<()> {
        let mut conn = self.redis_pool.get().await?;

        let json = serde_json::to_string(session)?;
        let ttl = (session.expires_at - Utc::now()).num_seconds().max(0) as usize;

        redis::cmd("SETEX")
            .arg(format!("session:{}", session_id))
            .arg(ttl)
            .arg(json)
            .query_async(&mut *conn)
            .await?;

        Ok(())
    }

    async fn get_redis(&self, session_id: &str) -> Result<Option<Session>> {
        let mut conn = self.redis_pool.get().await?;

        let result: Option<String> = redis::cmd("GET")
            .arg(format!("session:{}", session_id))
            .query_async(&mut *conn)
            .await?;

        match result {
            Some(json) => Ok(Some(serde_json::from_str(&json)?)),
            None => Ok(None),
        }
    }

    async fn set_postgres(&self, session_id: &str, session: &Session) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO sessions (
                session_id, msisdn, operator_code, service_code,
                state, menu_context, created_at, last_activity, expires_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (session_id) DO UPDATE SET
                state = EXCLUDED.state,
                menu_context = EXCLUDED.menu_context,
                last_activity = EXCLUDED.last_activity,
                expires_at = EXCLUDED.expires_at
            "#,
            session.session_id,
            session.msisdn,
            session.operator_code,
            session.service_code,
            serde_json::to_value(&session.state)?,
            serde_json::to_value(&session.menu_context)?,
            session.created_at,
            session.last_activity,
            session.expires_at,
        )
        .execute(&*self.db_pool)
        .await?;

        Ok(())
    }

    async fn get_postgres(&self, session_id: &str) -> Result<Option<Session>> {
        let record = sqlx::query!(
            r#"
            SELECT session_id, msisdn, operator_code, service_code,
                   state, menu_context, created_at, last_activity, expires_at
            FROM sessions
            WHERE session_id = $1
            "#,
            session_id
        )
        .fetch_optional(&*self.db_pool)
        .await?;

        match record {
            Some(row) => Ok(Some(Session {
                session_id: row.session_id,
                msisdn: row.msisdn,
                operator_code: row.operator_code,
                service_code: row.service_code,
                state: serde_json::from_value(row.state)?,
                menu_context: serde_json::from_value(row.menu_context)?,
                input_history: Vec::new(),
                metadata: HashMap::new(),
                created_at: row.created_at,
                last_activity: row.last_activity,
                expires_at: row.expires_at,
                interaction_count: 0,
            })),
            None => Ok(None),
        }
    }

    pub async fn delete(&self, session_id: &str) -> Result<()> {
        // Remove from all cache levels
        self.l1_cache.remove(session_id);
        self.moka_cache.invalidate(session_id).await;

        let mut conn = self.redis_pool.get().await?;
        redis::cmd("DEL")
            .arg(format!("session:{}", session_id))
            .query_async(&mut *conn)
            .await?;

        // Keep in PostgreSQL for analytics
        sqlx::query!(
            "UPDATE sessions SET state = $1 WHERE session_id = $2",
            serde_json::to_value(&SessionState::Completed)?,
            session_id
        )
        .execute(&*self.db_pool)
        .await?;

        Ok(())
    }
}
```

### 3. Background Session Cleanup

```rust
use tokio_cron_scheduler::{Job, JobScheduler};

pub struct SessionCleanupService {
    cache_manager: Arc<CacheManager>,
    scheduler: JobScheduler,
}

impl SessionCleanupService {
    pub async fn new(cache_manager: Arc<CacheManager>) -> Result<Self> {
        let scheduler = JobScheduler::new().await?;

        Ok(Self {
            cache_manager,
            scheduler,
        })
    }

    pub async fn start(&self) -> Result<()> {
        let cache_manager = self.cache_manager.clone();

        // Run cleanup every minute
        let job = Job::new_async("0 * * * * *", move |_uuid, _lock| {
            let cache_manager = cache_manager.clone();
            Box::pin(async move {
                if let Err(e) = cleanup_expired_sessions(cache_manager).await {
                    error!("Session cleanup failed: {}", e);
                }
            })
        })?;

        self.scheduler.add(job).await?;
        self.scheduler.start().await?;

        info!("Session cleanup service started");

        Ok(())
    }
}

async fn cleanup_expired_sessions(cache_manager: Arc<CacheManager>) -> Result<()> {
    let start = std::time::Instant::now();
    let mut cleaned = 0;

    // Cleanup L1 cache (DashMap)
    let expired_keys: Vec<String> = cache_manager.l1_cache
        .iter()
        .filter(|entry| entry.value().is_expired())
        .map(|entry| entry.key().clone())
        .collect();

    for key in expired_keys {
        cache_manager.l1_cache.remove(&key);
        cleaned += 1;
    }

    info!(
        "Cleaned {} expired sessions in {:?}",
        cleaned,
        start.elapsed()
    );

    Ok(())
}
```

### 4. Event Publisher (NATS)

```rust
use async_nats::Client as NatsClient;

pub struct EventPublisher {
    nats_client: NatsClient,
}

impl EventPublisher {
    pub async fn new(nats_url: &str) -> Result<Self> {
        let nats_client = async_nats::connect(nats_url).await?;

        Ok(Self { nats_client })
    }

    pub async fn publish_session_created(&self, session: &Session) {
        let event = SessionEvent {
            event_type: "session.created".to_string(),
            session_id: session.session_id.clone(),
            msisdn: session.msisdn.clone(),
            timestamp: Utc::now(),
        };

        if let Ok(payload) = serde_json::to_vec(&event) {
            let _ = self.nats_client.publish("sessions.created", payload.into()).await;
        }
    }

    pub async fn publish_session_updated(&self, session: &Session) {
        let event = SessionEvent {
            event_type: "session.updated".to_string(),
            session_id: session.session_id.clone(),
            msisdn: session.msisdn.clone(),
            timestamp: Utc::now(),
        };

        if let Ok(payload) = serde_json::to_vec(&event) {
            let _ = self.nats_client.publish("sessions.updated", payload.into()).await;
        }
    }

    pub async fn publish_session_deleted(&self, session_id: &str) {
        let event = SessionEvent {
            event_type: "session.deleted".to_string(),
            session_id: session_id.to_string(),
            msisdn: String::new(),
            timestamp: Utc::now(),
        };

        if let Ok(payload) = serde_json::to_vec(&event) {
            let _ = self.nats_client.publish("sessions.deleted", payload.into()).await;
        }
    }
}

#[derive(Serialize)]
struct SessionEvent {
    event_type: String,
    session_id: String,
    msisdn: String,
    timestamp: DateTime<Utc>,
}
```

## Configuration

```toml
[service]
name = "session-manager"
port = 50051
metrics_port = 9091

[redis]
urls = [
    "redis://redis-node-1:6379",
    "redis://redis-node-2:6379",
    "redis://redis-node-3:6379"
]
pool_size = 100
connection_timeout_ms = 3000

[postgres]
url = "postgresql://user:pass@postgres:5432/ussd_gw"
max_connections = 50
min_connections = 10

[cache]
l1_max_size = 100000
l1_ttl_seconds = 300
redis_ttl_seconds = 300

[session]
default_timeout_minutes = 5
max_timeout_minutes = 15
cleanup_interval_seconds = 60

[menu_engine]
endpoints = [
    "http://menu-engine-1:50052",
    "http://menu-engine-2:50052"
]
timeout_ms = 10000

[nats]
url = "nats://nats:4222"
max_reconnects = 10
```

## Performance Optimizations

### 1. Read-Through Cache
```rust
pub async fn get_with_fallback(&self, session_id: &str) -> Result<Session> {
    // Try cache first
    if let Some(session) = self.get(session_id).await? {
        return Ok(session);
    }

    // If not in cache, return error (don't compute)
    Err(anyhow::anyhow!("Session not found"))
}
```

### 2. Write-Behind Cache
```rust
pub async fn set_async(&self, session_id: String, session: Session) {
    let cache_manager = self.clone();

    // Write to L1 immediately
    cache_manager.l1_cache.insert(session_id.clone(), session.clone());

    // Write to L2/L3 asynchronously
    tokio::spawn(async move {
        let _ = cache_manager.set_redis(&session_id, &session).await;
        let _ = cache_manager.set_postgres(&session_id, &session).await;
    });
}
```

### 3. Batch Operations
```rust
pub async fn get_batch(&self, session_ids: &[String]) -> Result<Vec<Session>> {
    // Parallel fetches
    let futures = session_ids.iter()
        .map(|id| self.get(id))
        .collect::<Vec<_>>();

    let results = futures::future::join_all(futures).await;

    results.into_iter()
        .filter_map(|r| r.ok().flatten())
        .collect()
}
```

## Monitoring

```rust
use prometheus::{IntGauge, IntCounter, Histogram};

lazy_static! {
    static ref ACTIVE_SESSIONS: IntGauge = register_int_gauge!(
        "session_manager_active_sessions",
        "Number of active sessions"
    ).unwrap();

    static ref CACHE_HITS: IntCounter = register_int_counter!(
        "session_manager_cache_hits_total",
        "Total cache hits"
    ).unwrap();

    static ref CACHE_MISSES: IntCounter = register_int_counter!(
        "session_manager_cache_misses_total",
        "Total cache misses"
    ).unwrap();

    static ref SESSION_DURATION: Histogram = register_histogram!(
        "session_manager_session_duration_seconds",
        "Session duration"
    ).unwrap();
}
```

## Database Schema

```sql
CREATE TABLE sessions (
    session_id VARCHAR(64) PRIMARY KEY,
    msisdn VARCHAR(20) NOT NULL,
    operator_code VARCHAR(20) NOT NULL,
    service_code VARCHAR(20) NOT NULL,
    state JSONB NOT NULL,
    menu_context JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_activity TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    INDEX idx_msisdn (msisdn),
    INDEX idx_expires_at (expires_at),
    INDEX idx_state ((state->>'state'))
);

CREATE TABLE session_analytics (
    id BIGSERIAL PRIMARY KEY,
    session_id VARCHAR(64) REFERENCES sessions(session_id),
    event_type VARCHAR(50) NOT NULL,
    event_data JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

---

**Next**: [Service 3: Menu Engine](./03-MENU-ENGINE-SERVICE.md)
