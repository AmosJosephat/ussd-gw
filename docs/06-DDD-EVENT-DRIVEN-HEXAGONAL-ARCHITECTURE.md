# Domain-Driven Design, Event-Driven Architecture & Hexagonal Architecture

## Overview

This document redesigns the USSD Gateway using advanced architectural patterns:
- **DDD (Domain-Driven Design)**: Bounded contexts, aggregates, domain events
- **Event-Driven Architecture**: Event sourcing, CQRS, sagas
- **Hexagonal Architecture**: Ports & Adapters pattern for clean architecture

## Why These Patterns?

### Problems They Solve

1. **Tight Coupling**: Traditional layered architecture couples business logic to infrastructure
2. **Poor Testability**: Hard to test without real database/external services
3. **Complex Business Logic**: USSD workflows have complex state transitions
4. **Distributed Transactions**: Multi-service operations need coordination
5. **Auditability**: Need complete history of state changes
6. **Scalability**: Event-driven allows horizontal scaling
7. **Flexibility**: Easy to add new protocols/channels

## Architecture Overview

```
┌────────────────────────────────────────────────────────────────────────┐
│                    USSD Gateway - Hexagonal Architecture                │
├────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌─────────────────────────────────────────────────────────────────┐  │
│  │                    INPUT ADAPTERS (Driving)                      │  │
│  ├─────────────────────────────────────────────────────────────────┤  │
│  │  SS7/MAP    │  HTTP/REST  │  SMPP  │  gRPC  │  WebSocket       │  │
│  │  Adapter    │  Adapter    │ Adapter│ Adapter│  Adapter         │  │
│  └──────────────────────────┬──────────────────────────────────────┘  │
│                              ▼                                          │
│  ┌─────────────────────────────────────────────────────────────────┐  │
│  │                    INPUT PORTS (Use Cases)                       │  │
│  ├─────────────────────────────────────────────────────────────────┤  │
│  │  • InitiateUssdSessionPort                                       │  │
│  │  • ContinueUssdSessionPort                                       │  │
│  │  • TerminateUssdSessionPort                                      │  │
│  │  • ProcessMenuInputPort                                          │  │
│  │  • ExecuteTransactionPort                                        │  │
│  └──────────────────────────┬──────────────────────────────────────┘  │
│                              ▼                                          │
│  ┌─────────────────────────────────────────────────────────────────┐  │
│  │                    DOMAIN LAYER (Core)                           │  │
│  ├─────────────────────────────────────────────────────────────────┤  │
│  │                                                                   │  │
│  │  ┌────────────────────────────────────────────────────────────┐ │  │
│  │  │         BOUNDED CONTEXT: Session Management                 │ │  │
│  │  ├────────────────────────────────────────────────────────────┤ │  │
│  │  │  Aggregates:                                                │ │  │
│  │  │    • UssdSession (Aggregate Root)                          │ │  │
│  │  │    • SessionState (Value Object)                           │ │  │
│  │  │    • SessionLifecycle                                       │ │  │
│  │  │                                                              │ │  │
│  │  │  Domain Events:                                             │ │  │
│  │  │    • SessionInitiated                                       │ │  │
│  │  │    • SessionContinued                                       │ │  │
│  │  │    • SessionTimedOut                                        │ │  │
│  │  │    • SessionTerminated                                      │ │  │
│  │  │                                                              │ │  │
│  │  │  Domain Services:                                           │ │  │
│  │  │    • SessionExpirationService                               │ │  │
│  │  │    • SessionRecoveryService                                 │ │  │
│  │  └────────────────────────────────────────────────────────────┘ │  │
│  │                                                                   │  │
│  │  ┌────────────────────────────────────────────────────────────┐ │  │
│  │  │         BOUNDED CONTEXT: Menu Navigation                    │ │  │
│  │  ├────────────────────────────────────────────────────────────┤ │  │
│  │  │  Aggregates:                                                │ │  │
│  │  │    • MenuFlow (Aggregate Root)                             │ │  │
│  │  │    • MenuItem (Entity)                                      │ │  │
│  │  │    • NavigationPath (Value Object)                         │ │  │
│  │  │                                                              │ │  │
│  │  │  Domain Events:                                             │ │  │
│  │  │    • MenuRendered                                           │ │  │
│  │  │    • MenuOptionSelected                                     │ │  │
│  │  │    • MenuNavigationCompleted                                │ │  │
│  │  │    • InputValidationFailed                                  │ │  │
│  │  │                                                              │ │  │
│  │  │  Domain Services:                                           │ │  │
│  │  │    • MenuRenderingService                                   │ │  │
│  │  │    • InputValidationService                                 │ │  │
│  │  └────────────────────────────────────────────────────────────┘ │  │
│  │                                                                   │  │
│  │  ┌────────────────────────────────────────────────────────────┐ │  │
│  │  │         BOUNDED CONTEXT: Transaction Processing             │ │  │
│  │  ├────────────────────────────────────────────────────────────┤ │  │
│  │  │  Aggregates:                                                │ │  │
│  │  │    • Transaction (Aggregate Root)                          │ │  │
│  │  │    • TransactionStatus (Value Object)                      │ │  │
│  │  │    • Amount (Value Object)                                  │ │  │
│  │  │                                                              │ │  │
│  │  │  Domain Events:                                             │ │  │
│  │  │    • TransactionInitiated                                   │ │  │
│  │  │    • TransactionAuthorized                                  │ │  │
│  │  │    • TransactionCompleted                                   │ │  │
│  │  │    • TransactionFailed                                      │ │  │
│  │  │    • TransactionReversed                                    │ │  │
│  │  │                                                              │ │  │
│  │  │  Domain Services:                                           │ │  │
│  │  │    • TransactionAuthorizationService                        │ │  │
│  │  │    • FraudDetectionService                                  │ │  │
│  │  └────────────────────────────────────────────────────────────┘ │  │
│  │                                                                   │  │
│  │  ┌────────────────────────────────────────────────────────────┐ │  │
│  │  │         BOUNDED CONTEXT: Subscriber Management              │ │  │
│  │  ├────────────────────────────────────────────────────────────┤ │  │
│  │  │  Aggregates:                                                │ │  │
│  │  │    • Subscriber (Aggregate Root)                           │ │  │
│  │  │    • Msisdn (Value Object)                                  │ │  │
│  │  │    • SubscriberTier (Value Object)                         │ │  │
│  │  │                                                              │ │  │
│  │  │  Domain Events:                                             │ │  │
│  │  │    • SubscriberRegistered                                   │ │  │
│  │  │    • SubscriberTierUpgraded                                 │ │  │
│  │  │    • SubscriberBlacklisted                                  │ │  │
│  │  │    • SubscriberPreferencesUpdated                           │ │  │
│  │  └────────────────────────────────────────────────────────────┘ │  │
│  │                                                                   │  │
│  └──────────────────────────┬──────────────────────────────────────┘  │
│                              ▼                                          │
│  ┌─────────────────────────────────────────────────────────────────┐  │
│  │                    OUTPUT PORTS (Interfaces)                     │  │
│  ├─────────────────────────────────────────────────────────────────┤  │
│  │  • SessionRepositoryPort                                         │  │
│  │  • MenuRepositoryPort                                            │  │
│  │  • SubscriberRepositoryPort                                      │  │
│  │  • EventStorePort                                                │  │
│  │  • ExternalPaymentServicePort                                    │  │
│  │  • NotificationServicePort                                       │  │
│  │  • CacheServicePort                                              │  │
│  └──────────────────────────┬──────────────────────────────────────┘  │
│                              ▼                                          │
│  ┌─────────────────────────────────────────────────────────────────┐  │
│  │                OUTPUT ADAPTERS (Driven)                          │  │
│  ├─────────────────────────────────────────────────────────────────┤  │
│  │  PostgreSQL │  Redis   │  EventStore │  M-Pesa  │  SMTP         │  │
│  │  Adapter    │  Adapter │  Adapter    │  Adapter │  Adapter      │  │
│  └─────────────────────────────────────────────────────────────────┘  │
│                                                                          │
│  ┌─────────────────────────────────────────────────────────────────┐  │
│  │                    EVENT BUS (Infrastructure)                    │  │
│  ├─────────────────────────────────────────────────────────────────┤  │
│  │  • Domain Event Publisher                                        │  │
│  │  • Event Handlers (Subscribers)                                  │  │
│  │  • Event Store (Event Sourcing)                                  │  │
│  │  • Saga Orchestrator                                             │  │
│  └─────────────────────────────────────────────────────────────────┘  │
│                                                                          │
└────────────────────────────────────────────────────────────────────────┘
```

## 1. Domain-Driven Design (DDD)

### 1.1 Bounded Contexts

We identify **4 core bounded contexts** in the USSD Gateway:

#### Context Map

```
┌─────────────────────┐
│  Session Management │ ──────────┐
└─────────────────────┘           │
                                  │ ACL
┌─────────────────────┐           ▼
│  Menu Navigation    │ ←──── Customer/Supplier
└─────────────────────┘
         │
         │ Conformist
         ▼
┌─────────────────────┐
│  Transaction        │
│  Processing         │
└─────────────────────┘
         │
         │ ACL
         ▼
┌─────────────────────┐
│  Subscriber         │
│  Management         │
└─────────────────────┘
```

### 1.2 Ubiquitous Language

| Term | Definition | Context |
|------|------------|---------|
| **Session** | A stateful USSD interaction between subscriber and gateway | Session Management |
| **MSISDN** | Mobile Station International Subscriber Directory Number | All Contexts |
| **Menu Flow** | A directed graph of menu items and navigation paths | Menu Navigation |
| **Transaction** | A financial or data operation initiated via USSD | Transaction Processing |
| **Subscriber** | A registered mobile user with preferences and tier | Subscriber Management |
| **Aggregate** | A cluster of domain objects treated as a single unit | All Contexts |
| **Domain Event** | A significant occurrence in the domain | All Contexts |

### 1.3 Aggregates and Entities

#### Session Management Context

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// AGGREGATE ROOT: UssdSession
// ============================================================================

/// UssdSession is the aggregate root for session management
///
/// Invariants:
/// - A session must have a valid MSISDN
/// - Session state transitions must follow the lifecycle
/// - Sessions must expire after configured TTL
pub struct UssdSession {
    // Aggregate ID
    id: SessionId,

    // Value Objects
    msisdn: Msisdn,
    operator_code: OperatorCode,

    // Entity
    lifecycle: SessionLifecycle,

    // State
    state: SessionState,
    context: SessionContext,

    // Audit
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    // Domain Events (uncommitted)
    uncommitted_events: Vec<DomainEvent>,
}

impl UssdSession {
    /// Factory method: Create a new USSD session
    pub fn initiate(
        msisdn: Msisdn,
        operator_code: OperatorCode,
        service_code: ServiceCode,
    ) -> Result<Self, DomainError> {
        // Business rule: Validate MSISDN format
        msisdn.validate()?;

        let now = Utc::now();
        let session_id = SessionId::generate();

        let mut session = Self {
            id: session_id.clone(),
            msisdn: msisdn.clone(),
            operator_code: operator_code.clone(),
            lifecycle: SessionLifecycle::new(),
            state: SessionState::Active,
            context: SessionContext::new(service_code),
            created_at: now,
            updated_at: now,
            uncommitted_events: Vec::new(),
        };

        // Raise domain event
        session.raise_event(DomainEvent::SessionInitiated(SessionInitiated {
            session_id,
            msisdn,
            operator_code,
            timestamp: now,
        }));

        Ok(session)
    }

    /// Command: Continue session with user input
    pub fn continue_with_input(
        &mut self,
        user_input: String,
    ) -> Result<(), DomainError> {
        // Business rule: Can only continue active sessions
        if !self.state.is_active() {
            return Err(DomainError::InvalidStateTransition(
                "Cannot continue inactive session".to_string()
            ));
        }

        // Business rule: Enforce maximum interactions
        if self.lifecycle.interaction_count >= self.lifecycle.max_interactions {
            self.timeout()?;
            return Err(DomainError::SessionExpired);
        }

        // Update state
        self.context.add_input(user_input.clone());
        self.lifecycle.record_interaction();
        self.updated_at = Utc::now();

        // Raise domain event
        self.raise_event(DomainEvent::SessionContinued(SessionContinued {
            session_id: self.id.clone(),
            user_input,
            interaction_count: self.lifecycle.interaction_count,
            timestamp: self.updated_at,
        }));

        Ok(())
    }

    /// Command: Terminate session
    pub fn terminate(&mut self) -> Result<(), DomainError> {
        // Business rule: Cannot terminate already terminated sessions
        if self.state.is_terminated() {
            return Err(DomainError::InvalidStateTransition(
                "Session already terminated".to_string()
            ));
        }

        self.state = SessionState::Terminated;
        self.updated_at = Utc::now();

        // Raise domain event
        self.raise_event(DomainEvent::SessionTerminated(SessionTerminated {
            session_id: self.id.clone(),
            reason: TerminationReason::UserInitiated,
            timestamp: self.updated_at,
        }));

        Ok(())
    }

    /// Command: Timeout session
    pub fn timeout(&mut self) -> Result<(), DomainError> {
        self.state = SessionState::TimedOut;
        self.updated_at = Utc::now();

        // Raise domain event
        self.raise_event(DomainEvent::SessionTimedOut(SessionTimedOut {
            session_id: self.id.clone(),
            last_activity: self.lifecycle.last_activity_at,
            timestamp: self.updated_at,
        }));

        Ok(())
    }

    /// Query: Check if session has expired
    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        let ttl = chrono::Duration::minutes(5);

        now - self.lifecycle.last_activity_at > ttl
    }

    /// Get uncommitted events and clear
    pub fn take_uncommitted_events(&mut self) -> Vec<DomainEvent> {
        std::mem::take(&mut self.uncommitted_events)
    }

    fn raise_event(&mut self, event: DomainEvent) {
        self.uncommitted_events.push(event);
    }
}

// ============================================================================
// VALUE OBJECTS
// ============================================================================

/// MSISDN (Mobile Station ISDN Number) - E.164 format
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Msisdn(String);

impl Msisdn {
    pub fn new(value: String) -> Result<Self, DomainError> {
        let msisdn = Self(value);
        msisdn.validate()?;
        Ok(msisdn)
    }

    fn validate(&self) -> Result<(), DomainError> {
        // Business rule: MSISDN must be E.164 format
        let re = regex::Regex::new(r"^\+[1-9]\d{1,14}$").unwrap();
        if !re.is_match(&self.0) {
            return Err(DomainError::InvalidMsisdn(self.0.clone()));
        }
        Ok(())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Session State (Value Object)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    Active,
    WaitingForInput,
    Processing,
    Completed,
    TimedOut,
    Cancelled,
    Error,
}

impl SessionState {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active | Self::WaitingForInput | Self::Processing)
    }

    pub fn is_terminated(&self) -> bool {
        matches!(self, Self::Completed | Self::TimedOut | Self::Cancelled | Self::Error)
    }
}

/// Session Context (Value Object)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    service_code: ServiceCode,
    menu_stack: Vec<MenuId>,
    variables: HashMap<String, serde_json::Value>,
    input_history: Vec<String>,
}

impl SessionContext {
    fn new(service_code: ServiceCode) -> Self {
        Self {
            service_code,
            menu_stack: Vec::new(),
            variables: HashMap::new(),
            input_history: Vec::new(),
        }
    }

    fn add_input(&mut self, input: String) {
        self.input_history.push(input);
    }
}

// ============================================================================
// ENTITY (within aggregate)
// ============================================================================

/// Session Lifecycle tracks timing and interaction metrics
#[derive(Debug, Clone)]
pub struct SessionLifecycle {
    pub interaction_count: u32,
    pub max_interactions: u32,
    pub created_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,
}

impl SessionLifecycle {
    fn new() -> Self {
        let now = Utc::now();
        Self {
            interaction_count: 0,
            max_interactions: 20,
            created_at: now,
            last_activity_at: now,
        }
    }

    fn record_interaction(&mut self) {
        self.interaction_count += 1;
        self.last_activity_at = Utc::now();
    }
}

// ============================================================================
// DOMAIN EVENTS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DomainEvent {
    SessionInitiated(SessionInitiated),
    SessionContinued(SessionContinued),
    SessionTimedOut(SessionTimedOut),
    SessionTerminated(SessionTerminated),
    // ... other events
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInitiated {
    pub session_id: SessionId,
    pub msisdn: Msisdn,
    pub operator_code: OperatorCode,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContinued {
    pub session_id: SessionId,
    pub user_input: String,
    pub interaction_count: u32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTimedOut {
    pub session_id: SessionId,
    pub last_activity: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTerminated {
    pub session_id: SessionId,
    pub reason: TerminationReason,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerminationReason {
    UserInitiated,
    SystemTimeout,
    Error,
    Completed,
}

// ============================================================================
// DOMAIN ERRORS
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Invalid MSISDN format: {0}")]
    InvalidMsisdn(String),

    #[error("Invalid state transition: {0}")]
    InvalidStateTransition(String),

    #[error("Session expired")]
    SessionExpired,

    #[error("Session not found")]
    SessionNotFound,
}

// ============================================================================
// TYPE ALIASES (for clarity)
// ============================================================================

pub type SessionId = String;
pub type MenuId = String;
pub type ServiceCode = String;
pub type OperatorCode = String;
```

### 1.4 Domain Services

```rust
/// Domain Service: Session Expiration Service
///
/// This is a domain service because:
/// - It operates on multiple aggregates
/// - Contains business logic that doesn't belong to a single entity
pub struct SessionExpirationService {
    session_repository: Arc<dyn SessionRepositoryPort>,
    event_publisher: Arc<dyn EventPublisherPort>,
}

impl SessionExpirationService {
    pub async fn expire_stale_sessions(&self) -> Result<Vec<SessionId>> {
        // Business rule: Sessions inactive for > 5 minutes should be expired
        let expired_sessions = self.session_repository
            .find_sessions_inactive_for(Duration::minutes(5))
            .await?;

        let mut expired_ids = Vec::new();

        for mut session in expired_sessions {
            // Apply domain logic
            session.timeout()?;

            // Save aggregate
            self.session_repository.save(&session).await?;

            // Publish domain events
            let events = session.take_uncommitted_events();
            for event in events {
                self.event_publisher.publish(event).await?;
            }

            expired_ids.push(session.id.clone());
        }

        Ok(expired_ids)
    }
}
```

## 2. Event-Driven Architecture

### 2.1 Event Sourcing

Instead of storing current state, we store **all events** that led to the current state.

```rust
/// Event Store Port (Output Port)
#[async_trait]
pub trait EventStorePort: Send + Sync {
    /// Append events to the event stream
    async fn append_events(
        &self,
        stream_id: &str,
        events: Vec<DomainEvent>,
        expected_version: i64,
    ) -> Result<()>;

    /// Read all events for an aggregate
    async fn read_stream(
        &self,
        stream_id: &str,
    ) -> Result<Vec<StoredEvent>>;

    /// Read events from a specific version
    async fn read_stream_from_version(
        &self,
        stream_id: &str,
        from_version: i64,
    ) -> Result<Vec<StoredEvent>>;
}

/// Stored Event (with metadata)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredEvent {
    pub stream_id: String,
    pub version: i64,
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub metadata: EventMetadata,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub correlation_id: String,
    pub causation_id: String,
    pub user_id: Option<String>,
}

/// Reconstruct aggregate from events (Event Sourcing)
pub async fn reconstruct_session(
    event_store: &dyn EventStorePort,
    session_id: &SessionId,
) -> Result<UssdSession> {
    // Read all events for this session
    let stored_events = event_store.read_stream(session_id).await?;

    if stored_events.is_empty() {
        return Err(DomainError::SessionNotFound);
    }

    // Replay events to rebuild state
    let mut session: Option<UssdSession> = None;

    for stored_event in stored_events {
        let domain_event: DomainEvent = serde_json::from_value(stored_event.event_data)?;

        session = match (session, domain_event) {
            // First event must be SessionInitiated
            (None, DomainEvent::SessionInitiated(event)) => {
                Some(UssdSession::initiate(
                    event.msisdn,
                    event.operator_code,
                    "default".to_string(), // From event
                )?)
            }

            // Apply subsequent events
            (Some(mut sess), DomainEvent::SessionContinued(event)) => {
                sess.continue_with_input(event.user_input)?;
                Some(sess)
            }

            (Some(mut sess), DomainEvent::SessionTerminated(_)) => {
                sess.terminate()?;
                Some(sess)
            }

            (Some(mut sess), DomainEvent::SessionTimedOut(_)) => {
                sess.timeout()?;
                Some(sess)
            }

            _ => return Err(anyhow::anyhow!("Invalid event stream")),
        };
    }

    session.ok_or_else(|| DomainError::SessionNotFound.into())
}
```

### 2.2 CQRS (Command Query Responsibility Segregation)

Separate **write model** (commands) from **read model** (queries).

```rust
// ============================================================================
// COMMANDS (Write Model)
// ============================================================================

#[derive(Debug, Clone)]
pub enum SessionCommand {
    InitiateSession {
        msisdn: Msisdn,
        operator_code: OperatorCode,
        service_code: ServiceCode,
    },
    ContinueSession {
        session_id: SessionId,
        user_input: String,
    },
    TerminateSession {
        session_id: SessionId,
    },
}

/// Command Handler
pub struct SessionCommandHandler {
    event_store: Arc<dyn EventStorePort>,
    event_publisher: Arc<dyn EventPublisherPort>,
}

impl SessionCommandHandler {
    pub async fn handle(&self, command: SessionCommand) -> Result<SessionId> {
        match command {
            SessionCommand::InitiateSession { msisdn, operator_code, service_code } => {
                // Create aggregate
                let mut session = UssdSession::initiate(msisdn, operator_code, service_code)?;
                let session_id = session.id.clone();

                // Get uncommitted events
                let events = session.take_uncommitted_events();

                // Store events
                self.event_store.append_events(
                    &session_id,
                    events.clone(),
                    0, // Expected version
                ).await?;

                // Publish events
                for event in events {
                    self.event_publisher.publish(event).await?;
                }

                Ok(session_id)
            }

            SessionCommand::ContinueSession { session_id, user_input } => {
                // Reconstruct aggregate from events
                let mut session = reconstruct_session(&*self.event_store, &session_id).await?;

                // Execute command
                session.continue_with_input(user_input)?;

                // Get uncommitted events
                let events = session.take_uncommitted_events();
                let version = events.len() as i64;

                // Store events
                self.event_store.append_events(
                    &session_id,
                    events.clone(),
                    version,
                ).await?;

                // Publish events
                for event in events {
                    self.event_publisher.publish(event).await?;
                }

                Ok(session_id)
            }

            SessionCommand::TerminateSession { session_id } => {
                let mut session = reconstruct_session(&*self.event_store, &session_id).await?;
                session.terminate()?;

                let events = session.take_uncommitted_events();
                let version = events.len() as i64;

                self.event_store.append_events(&session_id, events.clone(), version).await?;

                for event in events {
                    self.event_publisher.publish(event).await?;
                }

                Ok(session_id)
            }
        }
    }
}

// ============================================================================
// QUERIES (Read Model)
// ============================================================================

/// Read Model: Optimized for queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionReadModel {
    pub session_id: String,
    pub msisdn: String,
    pub operator_code: String,
    pub state: String,
    pub interaction_count: u32,
    pub created_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,
}

#[async_trait]
pub trait SessionQueryService: Send + Sync {
    /// Get session by ID (from read model)
    async fn get_session(&self, session_id: &str) -> Result<Option<SessionReadModel>>;

    /// Get active sessions for MSISDN
    async fn get_active_sessions_for_msisdn(&self, msisdn: &str) -> Result<Vec<SessionReadModel>>;

    /// Get sessions by state
    async fn get_sessions_by_state(&self, state: &str) -> Result<Vec<SessionReadModel>>;
}

/// Query Handler (reads from materialized view)
pub struct PostgresSessionQueryService {
    db_pool: Arc<sqlx::PgPool>,
}

#[async_trait]
impl SessionQueryService for PostgresSessionQueryService {
    async fn get_session(&self, session_id: &str) -> Result<Option<SessionReadModel>> {
        let record = sqlx::query_as!(
            SessionReadModel,
            r#"
            SELECT session_id, msisdn, operator_code, state,
                   interaction_count, created_at, last_activity_at
            FROM session_read_model
            WHERE session_id = $1
            "#,
            session_id
        )
        .fetch_optional(&*self.db_pool)
        .await?;

        Ok(record)
    }

    async fn get_active_sessions_for_msisdn(&self, msisdn: &str) -> Result<Vec<SessionReadModel>> {
        let records = sqlx::query_as!(
            SessionReadModel,
            r#"
            SELECT session_id, msisdn, operator_code, state,
                   interaction_count, created_at, last_activity_at
            FROM session_read_model
            WHERE msisdn = $1 AND state IN ('Active', 'WaitingForInput', 'Processing')
            ORDER BY created_at DESC
            "#,
            msisdn
        )
        .fetch_all(&*self.db_pool)
        .await?;

        Ok(records)
    }

    async fn get_sessions_by_state(&self, state: &str) -> Result<Vec<SessionReadModel>> {
        let records = sqlx::query_as!(
            SessionReadModel,
            r#"
            SELECT session_id, msisdn, operator_code, state,
                   interaction_count, created_at, last_activity_at
            FROM session_read_model
            WHERE state = $1
            ORDER BY last_activity_at DESC
            LIMIT 1000
            "#,
            state
        )
        .fetch_all(&*self.db_pool)
        .await?;

        Ok(records)
    }
}

/// Event Handler: Projects events to read model
pub struct SessionReadModelProjector {
    db_pool: Arc<sqlx::PgPool>,
}

impl SessionReadModelProjector {
    pub async fn handle_event(&self, event: DomainEvent) -> Result<()> {
        match event {
            DomainEvent::SessionInitiated(evt) => {
                // Insert into read model
                sqlx::query!(
                    r#"
                    INSERT INTO session_read_model
                    (session_id, msisdn, operator_code, state, interaction_count,
                     created_at, last_activity_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#,
                    evt.session_id,
                    evt.msisdn.value(),
                    evt.operator_code,
                    "Active",
                    0,
                    evt.timestamp,
                    evt.timestamp
                )
                .execute(&*self.db_pool)
                .await?;
            }

            DomainEvent::SessionContinued(evt) => {
                // Update read model
                sqlx::query!(
                    r#"
                    UPDATE session_read_model
                    SET interaction_count = $2,
                        last_activity_at = $3,
                        state = 'WaitingForInput'
                    WHERE session_id = $1
                    "#,
                    evt.session_id,
                    evt.interaction_count as i32,
                    evt.timestamp
                )
                .execute(&*self.db_pool)
                .await?;
            }

            DomainEvent::SessionTerminated(evt) => {
                sqlx::query!(
                    r#"
                    UPDATE session_read_model
                    SET state = 'Terminated',
                        last_activity_at = $2
                    WHERE session_id = $1
                    "#,
                    evt.session_id,
                    evt.timestamp
                )
                .execute(&*self.db_pool)
                .await?;
            }

            _ => {}
        }

        Ok(())
    }
}
```

### 2.3 Saga Pattern (Distributed Transactions)

For complex multi-step transactions (e.g., money transfer), we use the **Saga pattern**.

```rust
/// Saga: Money Transfer Saga
///
/// Steps:
/// 1. Validate sender balance
/// 2. Reserve funds from sender
/// 3. Transfer to recipient
/// 4. Send notifications
///
/// If any step fails, compensating transactions are executed
pub struct MoneyTransferSaga {
    saga_id: String,
    state: SagaState,
    steps: Vec<SagaStep>,
}

#[derive(Debug, Clone)]
pub enum SagaState {
    Started,
    InProgress { current_step: usize },
    Completed,
    Failed { failed_step: usize },
    Compensating { compensated_steps: Vec<usize> },
    Compensated,
}

pub struct SagaStep {
    pub name: String,
    pub execute: Box<dyn Fn() -> Result<()>>,
    pub compensate: Box<dyn Fn() -> Result<()>>,
}

/// Saga Orchestrator
pub struct SagaOrchestrator {
    event_bus: Arc<dyn EventBusPort>,
}

impl SagaOrchestrator {
    pub async fn execute_saga(&self, saga: &mut MoneyTransferSaga) -> Result<()> {
        saga.state = SagaState::Started;

        for (idx, step) in saga.steps.iter().enumerate() {
            saga.state = SagaState::InProgress { current_step: idx };

            // Execute step
            if let Err(e) = (step.execute)() {
                error!("Saga step {} failed: {}", step.name, e);

                // Mark as failed
                saga.state = SagaState::Failed { failed_step: idx };

                // Compensate previous steps in reverse order
                self.compensate_saga(saga, idx).await?;

                return Err(e);
            }
        }

        saga.state = SagaState::Completed;
        Ok(())
    }

    async fn compensate_saga(
        &self,
        saga: &mut MoneyTransferSaga,
        failed_step: usize,
    ) -> Result<()> {
        saga.state = SagaState::Compensating { compensated_steps: Vec::new() };

        // Compensate in reverse order
        for idx in (0..failed_step).rev() {
            let step = &saga.steps[idx];

            if let Err(e) = (step.compensate)() {
                error!("Compensation failed for step {}: {}", step.name, e);
                // Log but continue compensating other steps
            }

            if let SagaState::Compensating { ref mut compensated_steps } = saga.state {
                compensated_steps.push(idx);
            }
        }

        saga.state = SagaState::Compensated;
        Ok(())
    }
}
```

## 3. Hexagonal Architecture (Ports & Adapters)

### 3.1 Input Ports (Use Cases)

```rust
/// Input Port: Initiate USSD Session (Use Case)
#[async_trait]
pub trait InitiateUssdSessionPort: Send + Sync {
    async fn execute(
        &self,
        request: InitiateSessionRequest,
    ) -> Result<InitiateSessionResponse>;
}

#[derive(Debug)]
pub struct InitiateSessionRequest {
    pub msisdn: String,
    pub operator_code: String,
    pub service_code: String,
    pub protocol: String,
}

#[derive(Debug)]
pub struct InitiateSessionResponse {
    pub session_id: String,
    pub initial_menu: String,
    pub should_continue: bool,
}

/// Implementation (Application Service)
pub struct InitiateUssdSessionUseCase {
    command_handler: Arc<SessionCommandHandler>,
    menu_service: Arc<dyn MenuRenderingService>,
}

#[async_trait]
impl InitiateUssdSessionPort for InitiateUssdSessionUseCase {
    async fn execute(
        &self,
        request: InitiateSessionRequest,
    ) -> Result<InitiateSessionResponse> {
        // Validate input
        let msisdn = Msisdn::new(request.msisdn)?;

        // Execute command
        let command = SessionCommand::InitiateSession {
            msisdn: msisdn.clone(),
            operator_code: request.operator_code,
            service_code: request.service_code.clone(),
        };

        let session_id = self.command_handler.handle(command).await?;

        // Render initial menu
        let menu = self.menu_service
            .render_initial_menu(&session_id, &request.service_code)
            .await?;

        Ok(InitiateSessionResponse {
            session_id,
            initial_menu: menu,
            should_continue: true,
        })
    }
}
```

### 3.2 Output Ports (Repository Interfaces)

```rust
/// Output Port: Session Repository
#[async_trait]
pub trait SessionRepositoryPort: Send + Sync {
    async fn save(&self, session: &UssdSession) -> Result<()>;
    async fn find_by_id(&self, id: &SessionId) -> Result<Option<UssdSession>>;
    async fn find_sessions_inactive_for(&self, duration: Duration) -> Result<Vec<UssdSession>>;
}

/// Output Port: Event Publisher
#[async_trait]
pub trait EventPublisherPort: Send + Sync {
    async fn publish(&self, event: DomainEvent) -> Result<()>;
}

/// Output Port: External Payment Service
#[async_trait]
pub trait ExternalPaymentServicePort: Send + Sync {
    async fn transfer_money(
        &self,
        from: &Msisdn,
        to: &Msisdn,
        amount: Money,
    ) -> Result<TransactionId>;

    async fn get_balance(&self, msisdn: &Msisdn) -> Result<Money>;
}
```

### 3.3 Adapters (Infrastructure)

```rust
/// PostgreSQL Adapter (Output Adapter)
pub struct PostgresSessionRepository {
    pool: Arc<sqlx::PgPool>,
    event_store: Arc<dyn EventStorePort>,
}

#[async_trait]
impl SessionRepositoryPort for PostgresSessionRepository {
    async fn save(&self, session: &UssdSession) -> Result<()> {
        // Save using event sourcing
        let events = session.uncommitted_events.clone();

        self.event_store.append_events(
            &session.id,
            events,
            0,
        ).await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &SessionId) -> Result<Option<UssdSession>> {
        // Reconstruct from events
        match reconstruct_session(&*self.event_store, id).await {
            Ok(session) => Ok(Some(session)),
            Err(DomainError::SessionNotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn find_sessions_inactive_for(&self, duration: Duration) -> Result<Vec<UssdSession>> {
        // Query read model for efficiency
        let threshold = Utc::now() - duration;

        let session_ids = sqlx::query_scalar!(
            "SELECT session_id FROM session_read_model
             WHERE last_activity_at < $1 AND state IN ('Active', 'WaitingForInput')",
            threshold
        )
        .fetch_all(&*self.pool)
        .await?;

        // Reconstruct from event store
        let mut sessions = Vec::new();
        for id in session_ids {
            if let Ok(session) = reconstruct_session(&*self.event_store, &id).await {
                sessions.push(session);
            }
        }

        Ok(sessions)
    }
}

/// NATS Event Publisher Adapter (Output Adapter)
pub struct NatsEventPublisher {
    client: async_nats::Client,
}

#[async_trait]
impl EventPublisherPort for NatsEventPublisher {
    async fn publish(&self, event: DomainEvent) -> Result<()> {
        let subject = format!("ussd.events.{}", event.event_type());
        let payload = serde_json::to_vec(&event)?;

        self.client.publish(subject, payload.into()).await?;

        Ok(())
    }
}

/// M-Pesa Adapter (Output Adapter)
pub struct MpesaPaymentAdapter {
    client: reqwest::Client,
    config: MpesaConfig,
}

#[async_trait]
impl ExternalPaymentServicePort for MpesaPaymentAdapter {
    async fn transfer_money(
        &self,
        from: &Msisdn,
        to: &Msisdn,
        amount: Money,
    ) -> Result<TransactionId> {
        // Call M-Pesa C2B API
        let response = self.client
            .post(&format!("{}/b2c/v1/paymentrequest", self.config.base_url))
            .bearer_auth(&self.get_access_token().await?)
            .json(&json!({
                "InitiatorName": self.config.initiator_name,
                "Amount": amount.value(),
                "PartyA": from.value(),
                "PartyB": to.value(),
                "CommandID": "BusinessPayment",
            }))
            .send()
            .await?;

        let result: MpesaResponse = response.json().await?;

        Ok(result.transaction_id)
    }

    async fn get_balance(&self, msisdn: &Msisdn) -> Result<Money> {
        // Implementation
        todo!()
    }
}
```

---

## Summary

This DDD/Event-Driven/Hexagonal architecture provides:

✅ **Clear Domain Model**: Aggregates, entities, value objects with invariants
✅ **Event Sourcing**: Complete audit trail, temporal queries, event replay
✅ **CQRS**: Optimized writes and reads, independent scaling
✅ **Saga Pattern**: Distributed transaction coordination
✅ **Hexagonal Architecture**: Domain isolated from infrastructure
✅ **Testability**: Mock ports for unit testing
✅ **Flexibility**: Easy to swap adapters (e.g., Postgres → EventStore)

**Next Document**: Detailed implementation of remaining bounded contexts and integration examples.
