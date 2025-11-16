# Service 1: Protocol Gateway

## Overview

The Protocol Gateway is the entry point for all USSD traffic. It handles multiple telecom protocols, normalizes them into a canonical internal format, and routes requests to downstream services via gRPC.

## Responsibilities

1. **Multi-Protocol Support**
   - USSD over SS7 (MAP/TCAP)
   - HTTP/REST APIs
   - SMPP protocol
   - Custom telecom operator protocols

2. **Protocol Normalization**
   - Convert all protocols to unified internal format
   - Preserve protocol-specific metadata
   - Handle character encoding (GSM 7-bit, UCS2, UTF-8)

3. **Connection Management**
   - Connection pooling for external operators
   - Keep-alive and heartbeat mechanisms
   - Automatic reconnection with exponential backoff

4. **Traffic Management**
   - Rate limiting per operator/MSISDN
   - Request queuing and throttling
   - Load shedding under high load

## Architecture

```
┌─────────────────────────────────────────────────────┐
│            Protocol Gateway Service                  │
├─────────────────────────────────────────────────────┤
│                                                       │
│  ┌───────────────┐  ┌───────────────┐               │
│  │  SS7 Listener │  │ HTTP Listener │               │
│  │   Port: 8080  │  │  Port: 8081   │               │
│  └───────┬───────┘  └───────┬───────┘               │
│          │                   │                        │
│          └─────────┬─────────┘                        │
│                    ▼                                  │
│          ┌─────────────────────┐                     │
│          │ Protocol Adapters   │                     │
│          │  - SS7 Parser       │                     │
│          │  - HTTP Handler     │                     │
│          │  - SMPP Handler     │                     │
│          └──────────┬──────────┘                     │
│                     ▼                                 │
│          ┌─────────────────────┐                     │
│          │   Normalizer        │                     │
│          │  (Canonical Format) │                     │
│          └──────────┬──────────┘                     │
│                     ▼                                 │
│          ┌─────────────────────┐                     │
│          │   Validator         │                     │
│          │  - Input checks     │                     │
│          │  - Rate limiting    │                     │
│          └──────────┬──────────┘                     │
│                     ▼                                 │
│          ┌─────────────────────┐                     │
│          │  gRPC Client Pool   │                     │
│          │  → Session Manager  │                     │
│          └─────────────────────┘                     │
│                                                       │
└─────────────────────────────────────────────────────┘
```

## Data Models

### Canonical USSD Request
```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UssdRequest {
    /// Unique request identifier
    pub request_id: String,

    /// Session identifier (if exists)
    pub session_id: Option<String>,

    /// User's phone number in E.164 format
    pub msisdn: String,

    /// Operator/network code
    pub operator_code: String,

    /// USSD service code (e.g., *123#)
    pub service_code: String,

    /// User input text
    pub input: String,

    /// Request type
    pub request_type: UssdRequestType,

    /// Source protocol
    pub protocol: Protocol,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UssdRequestType {
    /// Initial USSD request (BEGIN)
    Initial,
    /// Continuation of existing session (CONTINUE)
    Continue,
    /// Session termination (END)
    End,
    /// Session timeout
    Timeout,
    /// User cancellation
    Cancel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Protocol {
    SS7,
    HTTP,
    SMPP,
    Custom(String),
}
```

### Canonical USSD Response
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UssdResponse {
    /// Request identifier (for correlation)
    pub request_id: String,

    /// Session identifier
    pub session_id: String,

    /// Response text to display
    pub message: String,

    /// Response type
    pub response_type: UssdResponseType,

    /// Character encoding
    pub encoding: Encoding,

    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UssdResponseType {
    /// Continue session (expect user input)
    Continue,
    /// End session (no more input expected)
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Encoding {
    Gsm7Bit,
    Ucs2,
    Utf8,
}
```

## Core Components

### 1. SS7/MAP Protocol Handler

```rust
use tokio::net::TcpListener;
use bytes::Bytes;

pub struct Ss7Handler {
    listener: TcpListener,
    connection_pool: Arc<Ss7ConnectionPool>,
    parser: Ss7Parser,
}

impl Ss7Handler {
    pub async fn handle_connection(&self, stream: TcpStream) -> Result<()> {
        loop {
            // Read SS7 frame
            let frame = self.read_frame(&mut stream).await?;

            // Parse MAP/TCAP message
            let map_message = self.parser.parse_map(frame)?;

            // Convert to canonical format
            let ussd_request = self.to_canonical(map_message)?;

            // Forward to session manager
            let response = self.forward_to_session_manager(ussd_request).await?;

            // Convert back to SS7/MAP
            let map_response = self.from_canonical(response)?;

            // Send response
            self.send_frame(&mut stream, map_response).await?;
        }
    }

    async fn read_frame(&self, stream: &mut TcpStream) -> Result<Bytes> {
        // Implement SS7 frame reading with proper length handling
        todo!("SS7 frame decoding")
    }
}
```

### 2. HTTP/REST Protocol Handler

```rust
use axum::{
    Router,
    routing::post,
    Json,
    extract::State,
};

pub struct HttpHandler {
    grpc_client: SessionManagerClient,
}

#[derive(Deserialize)]
struct HttpUssdRequest {
    msisdn: String,
    session_id: Option<String>,
    service_code: String,
    input: String,
    operator: String,
}

async fn handle_ussd_http(
    State(handler): State<Arc<HttpHandler>>,
    Json(payload): Json<HttpUssdRequest>,
) -> Result<Json<UssdResponse>, AppError> {
    // Validate input
    validate_msisdn(&payload.msisdn)?;

    // Check rate limit
    handler.check_rate_limit(&payload.msisdn).await?;

    // Convert to canonical format
    let request = UssdRequest {
        request_id: Uuid::new_v4().to_string(),
        session_id: payload.session_id,
        msisdn: payload.msisdn,
        operator_code: payload.operator,
        service_code: payload.service_code,
        input: payload.input,
        request_type: UssdRequestType::Continue,
        protocol: Protocol::HTTP,
        timestamp: Utc::now(),
        metadata: HashMap::new(),
    };

    // Forward to session manager via gRPC
    let response = handler.grpc_client
        .process_request(request)
        .await?;

    Ok(Json(response))
}

pub fn create_router(handler: Arc<HttpHandler>) -> Router {
    Router::new()
        .route("/ussd", post(handle_ussd_http))
        .with_state(handler)
        .layer(
            ServiceBuilder::new()
                .timeout(Duration::from_secs(30))
                .layer(TraceLayer::new_for_http())
                .into_inner()
        )
}
```

### 3. Rate Limiter

```rust
use governor::{Quota, RateLimiter, state::keyed::DefaultKeyedStateStore};
use std::num::NonZeroU32;

pub struct RateLimitService {
    // Per-MSISDN rate limiter (e.g., 10 req/min)
    msisdn_limiter: RateLimiter<
        String,
        DefaultKeyedStateStore<String>,
        DefaultClock,
    >,

    // Global rate limiter (e.g., 100K req/sec)
    global_limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,

    // Redis-backed distributed rate limiter
    redis_pool: Arc<RedisPool>,
}

impl RateLimitService {
    pub fn new(redis_pool: Arc<RedisPool>) -> Self {
        let msisdn_quota = Quota::per_minute(NonZeroU32::new(10).unwrap());
        let global_quota = Quota::per_second(NonZeroU32::new(100_000).unwrap());

        Self {
            msisdn_limiter: RateLimiter::keyed(msisdn_quota),
            global_limiter: RateLimiter::direct(global_quota),
            redis_pool,
        }
    }

    pub async fn check_rate_limit(&self, msisdn: &str) -> Result<(), RateLimitError> {
        // Check global limit first (fast path)
        self.global_limiter.check()?;

        // Check per-MSISDN limit (in-memory)
        self.msisdn_limiter.check_key(&msisdn.to_string())?;

        // Check distributed limit (Redis)
        self.check_distributed_limit(msisdn).await?;

        Ok(())
    }

    async fn check_distributed_limit(&self, msisdn: &str) -> Result<()> {
        let mut conn = self.redis_pool.get().await?;

        let key = format!("rate_limit:{}", msisdn);
        let count: i32 = conn.incr(&key, 1).await?;

        if count == 1 {
            conn.expire(&key, 60).await?; // 60 seconds TTL
        }

        if count > 10 {
            return Err(RateLimitError::Exceeded);
        }

        Ok(())
    }
}
```

### 4. Protocol Normalizer

```rust
pub trait ProtocolAdapter: Send + Sync {
    /// Convert protocol-specific request to canonical format
    fn to_canonical(&self, raw: Bytes) -> Result<UssdRequest>;

    /// Convert canonical response to protocol-specific format
    fn from_canonical(&self, response: UssdResponse) -> Result<Bytes>;
}

pub struct Ss7Adapter;

impl ProtocolAdapter for Ss7Adapter {
    fn to_canonical(&self, raw: Bytes) -> Result<UssdRequest> {
        // Parse SS7/MAP message
        let map_msg = parse_map_message(&raw)?;

        // Extract USSD-specific data
        let ussd_data = extract_ussd_string(&map_msg)?;
        let msisdn = extract_msisdn(&map_msg)?;
        let session_id = extract_invoke_id(&map_msg)?;

        Ok(UssdRequest {
            request_id: Uuid::new_v4().to_string(),
            session_id: Some(session_id.to_string()),
            msisdn: format_e164(msisdn),
            operator_code: "OPERATOR_SS7".to_string(),
            service_code: extract_service_code(&map_msg)?,
            input: decode_ussd_string(ussd_data)?,
            request_type: determine_request_type(&map_msg),
            protocol: Protocol::SS7,
            timestamp: Utc::now(),
            metadata: extract_metadata(&map_msg),
        })
    }

    fn from_canonical(&self, response: UssdResponse) -> Result<Bytes> {
        // Build MAP response message
        let mut builder = MapMessageBuilder::new();

        builder
            .set_ussd_string(&response.message)
            .set_encoding(response.encoding)
            .set_message_type(match response.response_type {
                UssdResponseType::Continue => MapMessageType::Continue,
                UssdResponseType::End => MapMessageType::End,
            });

        builder.build()
    }
}
```

## gRPC Client Integration

```rust
use tonic::transport::Channel;
use session_manager::session_manager_client::SessionManagerClient;

pub struct GrpcClientPool {
    clients: Vec<SessionManagerClient<Channel>>,
    index: AtomicUsize,
}

impl GrpcClientPool {
    pub async fn new(endpoints: Vec<String>, pool_size: usize) -> Result<Self> {
        let mut clients = Vec::new();

        for endpoint in endpoints {
            for _ in 0..pool_size {
                let channel = Channel::from_shared(endpoint.clone())?
                    .connect_timeout(Duration::from_secs(5))
                    .tcp_keepalive(Some(Duration::from_secs(60)))
                    .http2_keep_alive_interval(Duration::from_secs(30))
                    .connect()
                    .await?;

                clients.push(SessionManagerClient::new(channel));
            }
        }

        Ok(Self {
            clients,
            index: AtomicUsize::new(0),
        })
    }

    pub fn get_client(&self) -> SessionManagerClient<Channel> {
        let idx = self.index.fetch_add(1, Ordering::Relaxed) % self.clients.len();
        self.clients[idx].clone()
    }
}
```

## Performance Optimizations

### 1. Zero-Copy Buffer Management
```rust
use bytes::{Bytes, BytesMut};

// Reuse buffers to avoid allocations
pub struct BufferPool {
    pool: Arc<Mutex<Vec<BytesMut>>>,
}

impl BufferPool {
    pub fn acquire(&self, capacity: usize) -> BytesMut {
        let mut pool = self.pool.lock().unwrap();
        pool.pop()
            .unwrap_or_else(|| BytesMut::with_capacity(capacity))
    }

    pub fn release(&self, mut buffer: BytesMut) {
        buffer.clear();
        let mut pool = self.pool.lock().unwrap();
        if pool.len() < 1000 { // Max pool size
            pool.push(buffer);
        }
    }
}
```

### 2. Connection Pooling
```rust
use deadpool::managed::{Pool, Manager};

pub struct Ss7ConnectionManager {
    endpoint: String,
}

#[async_trait]
impl Manager for Ss7ConnectionManager {
    type Type = TcpStream;
    type Error = io::Error;

    async fn create(&self) -> Result<TcpStream, io::Error> {
        TcpStream::connect(&self.endpoint).await
    }

    async fn recycle(&self, conn: &mut TcpStream) -> Result<(), io::Error> {
        // Send heartbeat to verify connection
        conn.write_all(b"HEARTBEAT").await?;
        Ok(())
    }
}

pub type Ss7Pool = Pool<Ss7ConnectionManager>;
```

### 3. Async Task Spawning
```rust
pub async fn spawn_protocol_listeners(config: Config) -> Result<()> {
    // Spawn SS7 listener
    let ss7_handle = tokio::spawn(async move {
        let listener = TcpListener::bind(&config.ss7_bind_addr).await?;
        let handler = Ss7Handler::new();

        loop {
            let (socket, _) = listener.accept().await?;
            let handler = handler.clone();

            tokio::spawn(async move {
                if let Err(e) = handler.handle_connection(socket).await {
                    error!("SS7 connection error: {}", e);
                }
            });
        }
    });

    // Spawn HTTP listener
    let http_handle = tokio::spawn(async move {
        let app = create_router(HttpHandler::new());
        axum::Server::bind(&config.http_bind_addr)
            .serve(app.into_make_service())
            .await
    });

    // Wait for all listeners
    tokio::try_join!(ss7_handle, http_handle)?;

    Ok(())
}
```

## Configuration

```toml
# config.toml
[gateway]
service_name = "protocol-gateway"
environment = "production"

[listeners.ss7]
enabled = true
bind_address = "0.0.0.0:8080"
max_connections = 1000
read_timeout_ms = 5000
write_timeout_ms = 5000

[listeners.http]
enabled = true
bind_address = "0.0.0.0:8081"
max_body_size = "1MB"
request_timeout_ms = 30000

[grpc]
session_manager_endpoints = [
    "http://session-manager-1:50051",
    "http://session-manager-2:50051",
    "http://session-manager-3:50051"
]
pool_size = 10
connect_timeout_ms = 5000
request_timeout_ms = 10000

[rate_limiting]
global_rps = 100000
per_msisdn_rpm = 10
redis_url = "redis://redis-cluster:6379"

[observability]
metrics_port = 9090
tracing_endpoint = "http://jaeger:14268/api/traces"
log_level = "info"
```

## Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("Protocol parsing error: {0}")]
    ProtocolError(String),

    #[error("Rate limit exceeded for MSISDN: {0}")]
    RateLimitExceeded(String),

    #[error("Invalid MSISDN format: {0}")]
    InvalidMsisdn(String),

    #[error("Session manager unavailable")]
    SessionManagerUnavailable,

    #[error("Connection timeout")]
    Timeout,

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

// Implement conversion to gRPC status codes
impl From<GatewayError> for tonic::Status {
    fn from(err: GatewayError) -> Self {
        match err {
            GatewayError::RateLimitExceeded(_) => {
                tonic::Status::resource_exhausted(err.to_string())
            }
            GatewayError::InvalidMsisdn(_) => {
                tonic::Status::invalid_argument(err.to_string())
            }
            GatewayError::SessionManagerUnavailable => {
                tonic::Status::unavailable(err.to_string())
            }
            _ => tonic::Status::internal(err.to_string()),
        }
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ss7_to_canonical_conversion() {
        let adapter = Ss7Adapter;
        let raw_ss7 = create_mock_ss7_message();

        let canonical = adapter.to_canonical(raw_ss7).unwrap();

        assert_eq!(canonical.msisdn, "+254712345678");
        assert_eq!(canonical.protocol, Protocol::SS7);
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let limiter = RateLimitService::new(mock_redis_pool());

        // First request should succeed
        assert!(limiter.check_rate_limit("254712345678").await.is_ok());

        // Exhaust rate limit
        for _ in 0..10 {
            let _ = limiter.check_rate_limit("254712345678").await;
        }

        // Next request should fail
        assert!(limiter.check_rate_limit("254712345678").await.is_err());
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_end_to_end_http_flow() {
    // Start test server
    let app = create_test_app();
    let server = TestServer::new(app).await;

    // Send USSD request
    let response = server.post("/ussd")
        .json(&json!({
            "msisdn": "+254712345678",
            "service_code": "*123#",
            "input": "1",
            "operator": "TEST_OP"
        }))
        .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body: UssdResponse = response.json().await;
    assert!(!body.message.is_empty());
}
```

## Deployment

### Docker Image
```dockerfile
FROM rust:1.75-slim as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --bin protocol-gateway

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/protocol-gateway /usr/local/bin/

EXPOSE 8080 8081 9090

CMD ["protocol-gateway"]
```

### Kubernetes Deployment
```yaml
apiVersion: v1
kind: Service
metadata:
  name: protocol-gateway
spec:
  selector:
    app: protocol-gateway
  ports:
    - name: ss7
      port: 8080
      targetPort: 8080
    - name: http
      port: 8081
      targetPort: 8081
    - name: metrics
      port: 9090
      targetPort: 9090
  type: LoadBalancer
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: protocol-gateway
spec:
  replicas: 3
  selector:
    matchLabels:
      app: protocol-gateway
  template:
    metadata:
      labels:
        app: protocol-gateway
    spec:
      containers:
      - name: protocol-gateway
        image: ussd-gw/protocol-gateway:v1.0.0
        ports:
        - containerPort: 8080
        - containerPort: 8081
        - containerPort: 9090
        env:
        - name: RUST_LOG
          value: "info"
        - name: SESSION_MANAGER_ENDPOINTS
          value: "http://session-manager:50051"
        resources:
          requests:
            memory: "256Mi"
            cpu: "500m"
          limits:
            memory: "1Gi"
            cpu: "2000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 9090
          initialDelaySeconds: 10
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /ready
            port: 9090
          initialDelaySeconds: 5
          periodSeconds: 10
```

## Monitoring and Alerting

### Key Metrics
```rust
use prometheus::{IntCounter, Histogram, register_int_counter, register_histogram};

lazy_static! {
    static ref REQUEST_COUNTER: IntCounter = register_int_counter!(
        "gateway_requests_total",
        "Total number of USSD requests"
    ).unwrap();

    static ref REQUEST_DURATION: Histogram = register_histogram!(
        "gateway_request_duration_seconds",
        "Request processing duration"
    ).unwrap();

    static ref RATE_LIMIT_HITS: IntCounter = register_int_counter!(
        "gateway_rate_limit_hits_total",
        "Number of rate limit violations"
    ).unwrap();
}

pub async fn process_with_metrics<F, T>(f: F) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    REQUEST_COUNTER.inc();
    let timer = REQUEST_DURATION.start_timer();

    let result = f.await;
    timer.observe_duration();

    result
}
```

### Alerts (Prometheus)
```yaml
groups:
  - name: protocol-gateway
    interval: 30s
    rules:
      - alert: HighLatency
        expr: histogram_quantile(0.99, gateway_request_duration_seconds) > 0.1
        for: 5m
        annotations:
          summary: "Gateway p99 latency above 100ms"

      - alert: HighErrorRate
        expr: rate(gateway_errors_total[5m]) > 0.05
        for: 2m
        annotations:
          summary: "Error rate above 5%"
```

---

**Next**: [Service 2: Session Manager](./02-SESSION-MANAGER-SERVICE.md)
