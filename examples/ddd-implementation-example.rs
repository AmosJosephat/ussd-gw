// ============================================================================
// Complete DDD + Event Sourcing + Hexagonal Architecture Example
// Domain: USSD Money Transfer
// ============================================================================

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

// ============================================================================
// DOMAIN LAYER (Hexagonal Core)
// ============================================================================

// ----------------------------------------------------------------------------
// Bounded Context: Transaction Processing
// ----------------------------------------------------------------------------

/// AGGREGATE ROOT: MoneyTransferTransaction
///
/// Invariants:
/// - Amount must be positive
/// - Sender and recipient must be different
/// - Transaction can only be completed once
/// - Failed transactions can be reversed
pub struct MoneyTransferTransaction {
    // Aggregate ID
    id: TransactionId,

    // Value Objects
    sender: Msisdn,
    recipient: Msisdn,
    amount: Money,

    // State
    status: TransactionStatus,

    // Metadata
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,

    // Domain Events (uncommitted)
    uncommitted_events: Vec<DomainEvent>,
}

impl MoneyTransferTransaction {
    /// Factory Method: Initiate a new money transfer
    pub fn initiate(
        sender: Msisdn,
        recipient: Msisdn,
        amount: Money,
    ) -> Result<Self, DomainError> {
        // Business Rule: Amount must be positive
        if amount.value() <= 0.0 {
            return Err(DomainError::InvalidAmount("Amount must be positive".into()));
        }

        // Business Rule: Sender and recipient must be different
        if sender == recipient {
            return Err(DomainError::SameSenderAndRecipient);
        }

        let transaction_id = TransactionId::generate();
        let now = Utc::now();

        let mut transaction = Self {
            id: transaction_id.clone(),
            sender: sender.clone(),
            recipient: recipient.clone(),
            amount: amount.clone(),
            status: TransactionStatus::Initiated,
            created_at: now,
            completed_at: None,
            uncommitted_events: Vec::new(),
        };

        // Raise Domain Event
        transaction.raise_event(DomainEvent::TransactionInitiated(TransactionInitiated {
            transaction_id,
            sender,
            recipient,
            amount,
            timestamp: now,
        }));

        Ok(transaction)
    }

    /// Command: Authorize the transaction
    pub fn authorize(&mut self) -> Result<(), DomainError> {
        // Business Rule: Can only authorize initiated transactions
        if self.status != TransactionStatus::Initiated {
            return Err(DomainError::InvalidStateTransition(
                format!("Cannot authorize transaction in state: {:?}", self.status)
            ));
        }

        self.status = TransactionStatus::Authorized;

        // Raise Domain Event
        self.raise_event(DomainEvent::TransactionAuthorized(TransactionAuthorized {
            transaction_id: self.id.clone(),
            timestamp: Utc::now(),
        }));

        Ok(())
    }

    /// Command: Complete the transaction
    pub fn complete(&mut self, external_ref: String) -> Result<(), DomainError> {
        // Business Rule: Can only complete authorized transactions
        if self.status != TransactionStatus::Authorized {
            return Err(DomainError::InvalidStateTransition(
                format!("Cannot complete transaction in state: {:?}", self.status)
            ));
        }

        let now = Utc::now();
        self.status = TransactionStatus::Completed;
        self.completed_at = Some(now);

        // Raise Domain Event
        self.raise_event(DomainEvent::TransactionCompleted(TransactionCompleted {
            transaction_id: self.id.clone(),
            external_reference: external_ref,
            timestamp: now,
        }));

        Ok(())
    }

    /// Command: Fail the transaction
    pub fn fail(&mut self, reason: String) -> Result<(), DomainError> {
        // Business Rule: Cannot fail already completed transactions
        if self.status == TransactionStatus::Completed {
            return Err(DomainError::InvalidStateTransition(
                "Cannot fail completed transaction".into()
            ));
        }

        self.status = TransactionStatus::Failed;

        // Raise Domain Event
        self.raise_event(DomainEvent::TransactionFailed(TransactionFailed {
            transaction_id: self.id.clone(),
            reason,
            timestamp: Utc::now(),
        }));

        Ok(())
    }

    /// Command: Reverse a completed transaction
    pub fn reverse(&mut self, reason: String) -> Result<(), DomainError> {
        // Business Rule: Can only reverse completed transactions
        if self.status != TransactionStatus::Completed {
            return Err(DomainError::InvalidStateTransition(
                "Can only reverse completed transactions".into()
            ));
        }

        self.status = TransactionStatus::Reversed;

        // Raise Domain Event
        self.raise_event(DomainEvent::TransactionReversed(TransactionReversed {
            transaction_id: self.id.clone(),
            reason,
            timestamp: Utc::now(),
        }));

        Ok(())
    }

    /// Query: Check if transaction is final (immutable)
    pub fn is_final(&self) -> bool {
        matches!(
            self.status,
            TransactionStatus::Completed | TransactionStatus::Reversed
        )
    }

    /// Get uncommitted events and clear
    pub fn take_uncommitted_events(&mut self) -> Vec<DomainEvent> {
        std::mem::take(&mut self.uncommitted_events)
    }

    fn raise_event(&mut self, event: DomainEvent) {
        self.uncommitted_events.push(event);
    }
}

// ----------------------------------------------------------------------------
// VALUE OBJECTS
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Msisdn(String);

impl Msisdn {
    pub fn new(value: String) -> Result<Self, DomainError> {
        let msisdn = Self(value);
        msisdn.validate()?;
        Ok(msisdn)
    }

    fn validate(&self) -> Result<(), DomainError> {
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

/// Money Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Money {
    amount: f64,
    currency: Currency,
}

impl Money {
    pub fn new(amount: f64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn value(&self) -> f64 {
        self.amount
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    /// Add two money values (must be same currency)
    pub fn add(&self, other: &Money) -> Result<Money, DomainError> {
        if self.currency != other.currency {
            return Err(DomainError::CurrencyMismatch);
        }
        Ok(Money::new(self.amount + other.amount, self.currency.clone()))
    }

    /// Subtract two money values
    pub fn subtract(&self, other: &Money) -> Result<Money, DomainError> {
        if self.currency != other.currency {
            return Err(DomainError::CurrencyMismatch);
        }
        Ok(Money::new(self.amount - other.amount, self.currency.clone()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    KES, // Kenyan Shilling
    USD, // US Dollar
    EUR, // Euro
}

/// Transaction Status (State Machine)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Initiated,
    Authorized,
    Completed,
    Failed,
    Reversed,
}

// ----------------------------------------------------------------------------
// DOMAIN EVENTS
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DomainEvent {
    TransactionInitiated(TransactionInitiated),
    TransactionAuthorized(TransactionAuthorized),
    TransactionCompleted(TransactionCompleted),
    TransactionFailed(TransactionFailed),
    TransactionReversed(TransactionReversed),
}

impl DomainEvent {
    pub fn event_type(&self) -> &str {
        match self {
            Self::TransactionInitiated(_) => "TransactionInitiated",
            Self::TransactionAuthorized(_) => "TransactionAuthorized",
            Self::TransactionCompleted(_) => "TransactionCompleted",
            Self::TransactionFailed(_) => "TransactionFailed",
            Self::TransactionReversed(_) => "TransactionReversed",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInitiated {
    pub transaction_id: TransactionId,
    pub sender: Msisdn,
    pub recipient: Msisdn,
    pub amount: Money,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionAuthorized {
    pub transaction_id: TransactionId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionCompleted {
    pub transaction_id: TransactionId,
    pub external_reference: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionFailed {
    pub transaction_id: TransactionId,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReversed {
    pub transaction_id: TransactionId,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

// ----------------------------------------------------------------------------
// DOMAIN SERVICES
// ----------------------------------------------------------------------------

/// Domain Service: Fraud Detection
/// (Cross-aggregate business logic)
pub struct FraudDetectionService {
    transaction_history_port: Arc<dyn TransactionHistoryPort>,
}

impl FraudDetectionService {
    pub async fn check_for_fraud(
        &self,
        transaction: &MoneyTransferTransaction,
    ) -> Result<FraudCheckResult, DomainError> {
        // Business Rule: Check if sender has multiple failed transactions
        let recent_failures = self.transaction_history_port
            .count_failed_transactions_in_last_hour(&transaction.sender)
            .await?;

        if recent_failures >= 5 {
            return Ok(FraudCheckResult::Suspicious(
                "Multiple failed transactions in last hour".into()
            ));
        }

        // Business Rule: Check for unusual amounts
        let avg_amount = self.transaction_history_port
            .get_average_transaction_amount(&transaction.sender)
            .await?;

        if transaction.amount.value() > avg_amount * 10.0 {
            return Ok(FraudCheckResult::Suspicious(
                "Transaction amount 10x above average".into()
            ));
        }

        Ok(FraudCheckResult::Clean)
    }
}

#[derive(Debug)]
pub enum FraudCheckResult {
    Clean,
    Suspicious(String),
    Blocked(String),
}

// ----------------------------------------------------------------------------
// DOMAIN ERRORS
// ----------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid MSISDN: {0}")]
    InvalidMsisdn(String),

    #[error("Invalid amount: {0}")]
    InvalidAmount(String),

    #[error("Sender and recipient cannot be the same")]
    SameSenderAndRecipient,

    #[error("Invalid state transition: {0}")]
    InvalidStateTransition(String),

    #[error("Currency mismatch")]
    CurrencyMismatch,

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Transaction not found")]
    TransactionNotFound,
}

// ----------------------------------------------------------------------------
// TYPE ALIASES
// ----------------------------------------------------------------------------

pub type TransactionId = String;

impl TransactionId {
    pub fn generate() -> Self {
        uuid::Uuid::new_v4().to_string()
    }
}

// ============================================================================
// APPLICATION LAYER (Use Cases)
// ============================================================================

// ----------------------------------------------------------------------------
// COMMANDS (Write Operations)
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum TransactionCommand {
    InitiateMoneyTransfer {
        sender: Msisdn,
        recipient: Msisdn,
        amount: Money,
    },
    AuthorizeTransaction {
        transaction_id: TransactionId,
    },
    CompleteTransaction {
        transaction_id: TransactionId,
        external_ref: String,
    },
}

/// Command Handler (Application Service)
pub struct TransactionCommandHandler {
    event_store: Arc<dyn EventStorePort>,
    event_publisher: Arc<dyn EventPublisherPort>,
    fraud_service: Arc<FraudDetectionService>,
}

impl TransactionCommandHandler {
    pub async fn handle(&self, command: TransactionCommand) -> Result<TransactionId, DomainError> {
        match command {
            TransactionCommand::InitiateMoneyTransfer { sender, recipient, amount } => {
                // Create aggregate
                let mut transaction = MoneyTransferTransaction::initiate(
                    sender,
                    recipient,
                    amount,
                )?;

                // Check for fraud (Domain Service)
                let fraud_check = self.fraud_service.check_for_fraud(&transaction).await?;
                if matches!(fraud_check, FraudCheckResult::Blocked(_)) {
                    transaction.fail("Blocked by fraud detection".into())?;
                }

                let transaction_id = transaction.id.clone();

                // Get uncommitted events
                let events = transaction.take_uncommitted_events();

                // Store events (Event Sourcing)
                self.event_store.append_events(
                    &transaction_id,
                    events.clone(),
                    0, // Expected version
                ).await?;

                // Publish events (Event-Driven)
                for event in events {
                    self.event_publisher.publish(event).await?;
                }

                Ok(transaction_id)
            }

            TransactionCommand::AuthorizeTransaction { transaction_id } => {
                // Reconstruct aggregate from events
                let mut transaction = self.reconstruct_transaction(&transaction_id).await?;

                // Execute command
                transaction.authorize()?;

                // Store and publish events
                let events = transaction.take_uncommitted_events();
                self.event_store.append_events(&transaction_id, events.clone(), 1).await?;

                for event in events {
                    self.event_publisher.publish(event).await?;
                }

                Ok(transaction_id)
            }

            TransactionCommand::CompleteTransaction { transaction_id, external_ref } => {
                let mut transaction = self.reconstruct_transaction(&transaction_id).await?;

                transaction.complete(external_ref)?;

                let events = transaction.take_uncommitted_events();
                self.event_store.append_events(&transaction_id, events.clone(), 2).await?;

                for event in events {
                    self.event_publisher.publish(event).await?;
                }

                Ok(transaction_id)
            }
        }
    }

    async fn reconstruct_transaction(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<MoneyTransferTransaction, DomainError> {
        let events = self.event_store.read_stream(transaction_id).await?;

        if events.is_empty() {
            return Err(DomainError::TransactionNotFound);
        }

        // Replay events to rebuild state
        let mut transaction: Option<MoneyTransferTransaction> = None;

        for stored_event in events {
            let domain_event: DomainEvent = serde_json::from_value(stored_event.event_data)?;

            transaction = match (transaction, domain_event) {
                (None, DomainEvent::TransactionInitiated(evt)) => {
                    Some(MoneyTransferTransaction::initiate(
                        evt.sender,
                        evt.recipient,
                        evt.amount,
                    )?)
                }

                (Some(mut txn), DomainEvent::TransactionAuthorized(_)) => {
                    txn.authorize()?;
                    Some(txn)
                }

                (Some(mut txn), DomainEvent::TransactionCompleted(evt)) => {
                    txn.complete(evt.external_reference)?;
                    Some(txn)
                }

                (Some(mut txn), DomainEvent::TransactionFailed(evt)) => {
                    txn.fail(evt.reason)?;
                    Some(txn)
                }

                _ => return Err(DomainError::InvalidStateTransition("Invalid event stream".into())),
            };
        }

        transaction.ok_or(DomainError::TransactionNotFound)
    }
}

// ----------------------------------------------------------------------------
// QUERIES (Read Operations - CQRS)
// ----------------------------------------------------------------------------

/// Read Model for queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReadModel {
    pub transaction_id: String,
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait TransactionQueryService: Send + Sync {
    async fn get_transaction(&self, id: &str) -> Result<Option<TransactionReadModel>, DomainError>;
    async fn get_transactions_for_sender(&self, sender: &str) -> Result<Vec<TransactionReadModel>, DomainError>;
}

// ============================================================================
// INFRASTRUCTURE LAYER (Ports & Adapters)
// ============================================================================

// ----------------------------------------------------------------------------
// OUTPUT PORTS (Interfaces)
// ----------------------------------------------------------------------------

#[async_trait]
pub trait EventStorePort: Send + Sync {
    async fn append_events(
        &self,
        stream_id: &str,
        events: Vec<DomainEvent>,
        expected_version: i64,
    ) -> Result<(), DomainError>;

    async fn read_stream(&self, stream_id: &str) -> Result<Vec<StoredEvent>, DomainError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredEvent {
    pub stream_id: String,
    pub version: i64,
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

#[async_trait]
pub trait EventPublisherPort: Send + Sync {
    async fn publish(&self, event: DomainEvent) -> Result<(), DomainError>;
}

#[async_trait]
pub trait TransactionHistoryPort: Send + Sync {
    async fn count_failed_transactions_in_last_hour(&self, msisdn: &Msisdn) -> Result<u32, DomainError>;
    async fn get_average_transaction_amount(&self, msisdn: &Msisdn) -> Result<f64, DomainError>;
}

// ----------------------------------------------------------------------------
// INPUT PORTS (Use Cases)
// ----------------------------------------------------------------------------

#[async_trait]
pub trait InitiateMoneyTransferPort: Send + Sync {
    async fn execute(
        &self,
        request: MoneyTransferRequest,
    ) -> Result<MoneyTransferResponse, DomainError>;
}

#[derive(Debug)]
pub struct MoneyTransferRequest {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug)]
pub struct MoneyTransferResponse {
    pub transaction_id: String,
    pub status: String,
}

/// Use Case Implementation
pub struct InitiateMoneyTransferUseCase {
    command_handler: Arc<TransactionCommandHandler>,
}

#[async_trait]
impl InitiateMoneyTransferPort for InitiateMoneyTransferUseCase {
    async fn execute(
        &self,
        request: MoneyTransferRequest,
    ) -> Result<MoneyTransferResponse, DomainError> {
        // Validate and convert to domain types
        let sender = Msisdn::new(request.sender)?;
        let recipient = Msisdn::new(request.recipient)?;
        let amount = Money::new(
            request.amount,
            match request.currency.as_str() {
                "KES" => Currency::KES,
                "USD" => Currency::USD,
                "EUR" => Currency::EUR,
                _ => return Err(DomainError::InvalidAmount("Invalid currency".into())),
            },
        );

        // Execute command
        let command = TransactionCommand::InitiateMoneyTransfer {
            sender,
            recipient,
            amount,
        };

        let transaction_id = self.command_handler.handle(command).await?;

        Ok(MoneyTransferResponse {
            transaction_id,
            status: "initiated".to_string(),
        })
    }
}

// ----------------------------------------------------------------------------
// ADAPTERS (Implementations)
// ----------------------------------------------------------------------------

/// PostgreSQL Event Store Adapter
pub struct PostgresEventStore {
    pool: Arc<sqlx::PgPool>,
}

#[async_trait]
impl EventStorePort for PostgresEventStore {
    async fn append_events(
        &self,
        stream_id: &str,
        events: Vec<DomainEvent>,
        expected_version: i64,
    ) -> Result<(), DomainError> {
        let mut tx = self.pool.begin().await?;

        for (idx, event) in events.iter().enumerate() {
            let version = expected_version + idx as i64 + 1;
            let event_type = event.event_type();
            let event_data = serde_json::to_value(event)?;

            sqlx::query!(
                r#"
                INSERT INTO event_store (stream_id, version, event_type, event_data)
                VALUES ($1, $2, $3, $4)
                "#,
                stream_id,
                version,
                event_type,
                event_data
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    async fn read_stream(&self, stream_id: &str) -> Result<Vec<StoredEvent>, DomainError> {
        let records = sqlx::query!(
            r#"
            SELECT stream_id, version, event_type, event_data, created_at
            FROM event_store
            WHERE stream_id = $1
            ORDER BY version ASC
            "#,
            stream_id
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|r| StoredEvent {
                stream_id: r.stream_id,
                version: r.version,
                event_type: r.event_type,
                event_data: r.event_data,
                timestamp: r.created_at,
            })
            .collect())
    }
}

/// NATS Event Publisher Adapter
pub struct NatsEventPublisher {
    client: async_nats::Client,
}

#[async_trait]
impl EventPublisherPort for NatsEventPublisher {
    async fn publish(&self, event: DomainEvent) -> Result<(), DomainError> {
        let subject = format!("transactions.{}", event.event_type());
        let payload = serde_json::to_vec(&event)?;

        self.client.publish(subject, payload.into()).await?;

        Ok(())
    }
}

// ============================================================================
// MAIN FUNCTION (Wiring Everything Together)
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Infrastructure setup
    let db_pool = Arc::new(
        sqlx::postgres::PgPoolOptions::new()
            .connect("postgresql://localhost/ussd_gw")
            .await?
    );

    let nats_client = async_nats::connect("nats://localhost:4222").await?;

    // Adapters (Infrastructure Layer)
    let event_store = Arc::new(PostgresEventStore { pool: db_pool.clone() });
    let event_publisher = Arc::new(NatsEventPublisher { client: nats_client });

    // Domain Services
    let transaction_history_port = todo!("Implement transaction history port");
    let fraud_service = Arc::new(FraudDetectionService {
        transaction_history_port,
    });

    // Application Services
    let command_handler = Arc::new(TransactionCommandHandler {
        event_store: event_store.clone(),
        event_publisher: event_publisher.clone(),
        fraud_service,
    });

    // Use Cases (Input Ports)
    let initiate_transfer_use_case = Arc::new(InitiateMoneyTransferUseCase {
        command_handler,
    });

    // Example: Execute use case
    let request = MoneyTransferRequest {
        sender: "+254712345678".to_string(),
        recipient: "+254798765432".to_string(),
        amount: 1000.0,
        currency: "KES".to_string(),
    };

    let response = initiate_transfer_use_case.execute(request).await?;

    println!("Transaction initiated: {:?}", response);

    Ok(())
}
