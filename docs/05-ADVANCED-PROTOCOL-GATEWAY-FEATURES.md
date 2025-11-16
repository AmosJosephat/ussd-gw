# Advanced Protocol Gateway Features

## Overview

This document extends the Protocol Gateway with enterprise-grade, next-level components that enable **no-code workflow configuration**, **policy-based routing**, **dynamic protocol specifications**, and **advanced I/O mapping**. These features transform the gateway from a simple adapter into a **full-featured API gateway and protocol mediator**.

## Enhanced Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                  Advanced Protocol Gateway                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │              Ingress Layer (Multi-Protocol)                   │  │
│  │  SS7/MAP  │  HTTP/REST  │  SMPP  │  gRPC  │  WebSocket       │  │
│  └────────────────────────────┬─────────────────────────────────┘  │
│                                ▼                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │           1. WORKFLOW ORCHESTRATION ENGINE                    │  │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐             │  │
│  │  │ Pre-Process│  │  Process   │  │Post-Process│             │  │
│  │  │   Stage    │→ │   Stage    │→ │   Stage    │             │  │
│  │  └────────────┘  └────────────┘  └────────────┘             │  │
│  │  - Declarative workflow definition (YAML/JSON)               │  │
│  │  - Visual workflow builder support                           │  │
│  │  - Conditional branching and parallel execution              │  │
│  │  - Stage plugins (custom Rust/WASM)                         │  │
│  └────────────────────────────┬─────────────────────────────────┘  │
│                                ▼                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │              2. POLICY ENGINE                                 │  │
│  │  ┌──────────────────────────────────────────────────────┐   │  │
│  │  │ Policy Types:                                         │   │  │
│  │  │  • Routing Policies (content-based, weighted)        │   │  │
│  │  │  • Transformation Policies (enrich, filter, map)     │   │  │
│  │  │  • Security Policies (auth, encryption, validation)   │   │  │
│  │  │  • Rate Limiting Policies (adaptive, burst, quota)   │   │  │
│  │  │  • Circuit Breaker Policies (fail-fast, fallback)    │   │  │
│  │  │  • Caching Policies (TTL, invalidation rules)        │   │  │
│  │  │  • Compliance Policies (PII masking, audit)          │   │  │
│  │  └──────────────────────────────────────────────────────┘   │  │
│  │  - Policy as Code (OPA/Rego, CEL expressions)               │  │
│  │  - Hot-reload without downtime                              │  │
│  │  - Policy versioning and rollback                           │  │
│  └────────────────────────────┬─────────────────────────────────┘  │
│                                ▼                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │         3. SPECIFICATION REGISTRY                             │  │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐             │  │
│  │  │  OpenAPI   │  │ Protocol   │  │  JSON      │             │  │
│  │  │  Specs     │  │  Buffers   │  │  Schema    │             │  │
│  │  └────────────┘  └────────────┘  └────────────┘             │  │
│  │  - Runtime validation against specs                          │  │
│  │  - Automatic code generation                                 │  │
│  │  - Version management (semver)                               │  │
│  │  - Breaking change detection                                 │  │
│  │  - Mock response generation                                  │  │
│  └────────────────────────────┬─────────────────────────────────┘  │
│                                ▼                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │      4. INPUT/OUTPUT MAPPING ENGINE                           │  │
│  │  ┌────────────────────────────────────────────────────────┐ │  │
│  │  │ Transformation Capabilities:                            │ │  │
│  │  │  • Protocol Translation (SS7 ↔ REST ↔ gRPC)          │ │  │
│  │  │  • Data Format Conversion (XML ↔ JSON ↔ Protobuf)   │ │  │
│  │  │  • Field Mapping (JSONPath, XPath, JQ)                 │ │  │
│  │  │  • Data Enrichment (lookup, calculate, aggregate)     │ │  │
│  │  │  • Content-Based Routing                               │ │  │
│  │  │  • Request/Response Aggregation                        │ │  │
│  │  │  • Character Encoding (GSM7, UCS2, UTF-8)            │ │  │
│  │  └────────────────────────────────────────────────────────┘ │  │
│  │  - Declarative mapping DSL                                   │  │
│  │  - Template engine (Handlebars, Jinja2)                     │  │
│  │  - Streaming transformations (large payloads)               │  │
│  └────────────────────────────┬─────────────────────────────────┘  │
│                                ▼                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │         5. PROTOCOL MEDIATION LAYER                           │  │
│  │  • Protocol versioning and negotiation                        │  │
│  │  • Backward compatibility handling                            │  │
│  │  • Protocol bridging (sync ↔ async)                          │  │
│  │  • Message queuing for async protocols                        │  │
│  └────────────────────────────┬─────────────────────────────────┘  │
│                                ▼                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │         6. REQUEST/RESPONSE ENRICHMENT                        │  │
│  │  • Context injection (correlation ID, timestamp)              │  │
│  │  • Metadata enrichment (geo-location, operator info)          │  │
│  │  • Header manipulation                                        │  │
│  │  • Response wrapping/unwrapping                               │  │
│  └────────────────────────────┬─────────────────────────────────┘  │
│                                ▼                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │         7. ADAPTIVE ROUTING ENGINE                            │  │
│  │  • Content-based routing (inspect payload)                    │  │
│  │  • Weighted routing (A/B testing, canary)                     │  │
│  │  • Geo-routing (based on MSISDN prefix)                       │  │
│  │  • Time-based routing (business hours)                        │  │
│  │  • Load-based routing (dynamic backend selection)             │  │
│  │  • Fallback routing (primary → secondary → tertiary)          │  │
│  └────────────────────────────┬─────────────────────────────────┘  │
│                                ▼                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │         8. ANALYTICS & TELEMETRY PIPELINE                     │  │
│  │  • Real-time metrics aggregation                              │  │
│  │  • Request/response logging (structured)                      │  │
│  │  • Distributed tracing (OpenTelemetry)                        │  │
│  │  • Anomaly detection                                          │  │
│  │  • Usage analytics and billing data                           │  │
│  └────────────────────────────┬─────────────────────────────────┘  │
│                                ▼                                      │
│                   To Session Manager (gRPC)                          │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

## 1. Workflow Orchestration Engine

### Overview
A declarative, stage-based workflow engine that allows **no-code configuration** of request processing pipelines.

### Workflow Definition (YAML DSL)

```yaml
# Example: USSD Request Processing Workflow
workflow:
  name: "ussd_request_processor"
  version: "1.0"

  # Workflow variables
  variables:
    max_retry: 3
    timeout_ms: 5000

  # Workflow stages
  stages:
    # Stage 1: Pre-processing
    - name: "validate_input"
      type: "validator"
      config:
        rules:
          - field: "msisdn"
            pattern: "^\\+?254[17]\\d{8}$"
            error_message: "Invalid MSISDN format"
          - field: "service_code"
            required: true

    # Stage 2: Policy evaluation
    - name: "apply_policies"
      type: "policy_evaluator"
      config:
        policies:
          - "rate_limiting_policy"
          - "security_validation_policy"
          - "operator_routing_policy"
      on_policy_violation:
        action: "reject"
        response_template: "policy_violation_response"

    # Stage 3: Protocol transformation
    - name: "transform_to_canonical"
      type: "transformer"
      config:
        mapping_file: "mappings/ss7_to_canonical.json"

    # Stage 4: Enrichment
    - name: "enrich_request"
      type: "enricher"
      config:
        enrichments:
          - type: "lookup"
            source: "redis"
            key_template: "subscriber:{{msisdn}}"
            target_field: "subscriber_info"
          - type: "compute"
            expression: "now()"
            target_field: "gateway_timestamp"
          - type: "geoip"
            source_field: "msisdn"
            target_field: "location"

    # Stage 5: Conditional routing
    - name: "route_request"
      type: "router"
      config:
        routing_rules:
          # VIP customers to premium path
          - condition: "subscriber_info.tier == 'VIP'"
            target: "session_manager_premium"
            weight: 100
          # A/B testing: 10% to new session manager
          - condition: "true"
            targets:
              - name: "session_manager_v2"
                weight: 10
              - name: "session_manager_v1"
                weight: 90

    # Stage 6: Error handling
    - name: "error_handler"
      type: "error_handler"
      config:
        retry:
          max_attempts: 3
          backoff: "exponential"
          initial_delay_ms: 100
        fallback:
          type: "static_response"
          template: "service_unavailable_template"

    # Stage 7: Response transformation
    - name: "transform_response"
      type: "transformer"
      config:
        mapping_file: "mappings/canonical_to_ss7.json"

    # Stage 8: Analytics
    - name: "track_metrics"
      type: "analytics"
      config:
        events:
          - type: "metric"
            name: "request_processed"
            labels:
              operator: "{{operator_code}}"
              protocol: "{{protocol}}"
          - type: "log"
            level: "info"
            message: "Request processed: {{request_id}}"
```

### Workflow Engine Implementation

```rust
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowDefinition {
    pub name: String,
    pub version: String,
    pub variables: HashMap<String, serde_json::Value>,
    pub stages: Vec<StageDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StageDefinition {
    pub name: String,
    pub stage_type: String,
    pub config: serde_json::Value,

    #[serde(default)]
    pub enabled: bool,

    #[serde(default)]
    pub continue_on_error: bool,

    pub on_error: Option<ErrorHandlerConfig>,
}

/// Workflow execution context
pub struct WorkflowContext {
    pub request: UssdRequest,
    pub variables: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,
    pub stage_results: HashMap<String, StageResult>,
}

#[derive(Debug, Clone)]
pub struct StageResult {
    pub stage_name: String,
    pub success: bool,
    pub duration_ms: u64,
    pub output: serde_json::Value,
    pub error: Option<String>,
}

/// Workflow orchestrator
pub struct WorkflowOrchestrator {
    workflows: Arc<RwLock<HashMap<String, WorkflowDefinition>>>,
    stage_registry: Arc<StageRegistry>,
    metrics: Arc<WorkflowMetrics>,
}

impl WorkflowOrchestrator {
    #[instrument(skip(self))]
    pub async fn execute_workflow(
        &self,
        workflow_name: &str,
        request: UssdRequest,
    ) -> Result<WorkflowContext> {
        let workflow = self.get_workflow(workflow_name)?;

        let mut context = WorkflowContext {
            request,
            variables: workflow.variables.clone(),
            metadata: HashMap::new(),
            stage_results: HashMap::new(),
        };

        // Execute stages sequentially (or parallel if configured)
        for stage in &workflow.stages {
            if !stage.enabled {
                continue;
            }

            let start = Instant::now();

            let result = match self.execute_stage(stage, &mut context).await {
                Ok(output) => StageResult {
                    stage_name: stage.name.clone(),
                    success: true,
                    duration_ms: start.elapsed().as_millis() as u64,
                    output,
                    error: None,
                },
                Err(e) => {
                    error!("Stage {} failed: {}", stage.name, e);

                    if stage.continue_on_error {
                        StageResult {
                            stage_name: stage.name.clone(),
                            success: false,
                            duration_ms: start.elapsed().as_millis() as u64,
                            output: json!({}),
                            error: Some(e.to_string()),
                        }
                    } else {
                        // Handle error according to stage config
                        if let Some(error_handler) = &stage.on_error {
                            self.handle_stage_error(error_handler, &context, e).await?;
                        }
                        return Err(e);
                    }
                }
            };

            context.stage_results.insert(stage.name.clone(), result);

            // Record metrics
            self.metrics.record_stage_execution(&stage.name, start.elapsed());
        }

        Ok(context)
    }

    async fn execute_stage(
        &self,
        stage: &StageDefinition,
        context: &mut WorkflowContext,
    ) -> Result<serde_json::Value> {
        let stage_executor = self.stage_registry.get(&stage.stage_type)?;

        stage_executor.execute(&stage.config, context).await
    }
}

/// Stage executor trait
#[async_trait]
pub trait StageExecutor: Send + Sync {
    async fn execute(
        &self,
        config: &serde_json::Value,
        context: &mut WorkflowContext,
    ) -> Result<serde_json::Value>;
}

/// Stage registry for pluggable stages
pub struct StageRegistry {
    executors: HashMap<String, Arc<dyn StageExecutor>>,
}

impl StageRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            executors: HashMap::new(),
        };

        // Register built-in stages
        registry.register("validator", Arc::new(ValidatorStage));
        registry.register("transformer", Arc::new(TransformerStage));
        registry.register("enricher", Arc::new(EnricherStage));
        registry.register("router", Arc::new(RouterStage));
        registry.register("policy_evaluator", Arc::new(PolicyEvaluatorStage));
        registry.register("analytics", Arc::new(AnalyticsStage));

        registry
    }

    pub fn register(&mut self, stage_type: &str, executor: Arc<dyn StageExecutor>) {
        self.executors.insert(stage_type.to_string(), executor);
    }

    pub fn get(&self, stage_type: &str) -> Result<Arc<dyn StageExecutor>> {
        self.executors.get(stage_type)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Unknown stage type: {}", stage_type))
    }
}
```

## 2. Policy Engine

### Policy Types and Examples

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Policy {
    /// Rate limiting policy
    RateLimit {
        id: String,
        name: String,
        rules: Vec<RateLimitRule>,
    },

    /// Routing policy
    Routing {
        id: String,
        name: String,
        rules: Vec<RoutingRule>,
    },

    /// Transformation policy
    Transformation {
        id: String,
        name: String,
        transformations: Vec<TransformationRule>,
    },

    /// Security policy
    Security {
        id: String,
        name: String,
        validations: Vec<SecurityValidation>,
    },

    /// Circuit breaker policy
    CircuitBreaker {
        id: String,
        name: String,
        config: CircuitBreakerConfig,
    },

    /// Caching policy
    Caching {
        id: String,
        name: String,
        rules: Vec<CachingRule>,
    },

    /// Compliance policy
    Compliance {
        id: String,
        name: String,
        rules: Vec<ComplianceRule>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RateLimitRule {
    /// Condition (CEL expression)
    pub condition: String,

    /// Limit type
    pub limit_type: RateLimitType,

    /// Key for rate limiting (e.g., "msisdn", "operator")
    pub key_template: String,

    /// Limit value
    pub limit: u32,

    /// Time window
    pub window: Duration,

    /// Action when exceeded
    pub on_exceeded: RateLimitAction,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RateLimitType {
    FixedWindow,
    SlidingWindow,
    TokenBucket,
    Leaky Bucket,
    Adaptive, // Adjusts based on system load
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoutingRule {
    /// Condition (CEL expression)
    pub condition: String,

    /// Target backends with weights
    pub targets: Vec<RoutingTarget>,

    /// Priority (higher = evaluated first)
    pub priority: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoutingTarget {
    pub backend: String,
    pub weight: u32,

    #[serde(default)]
    pub metadata: HashMap<String, String>,
}
```

### Policy Evaluation Engine

```rust
/// Policy engine with CEL (Common Expression Language) support
pub struct PolicyEngine {
    policies: Arc<RwLock<HashMap<String, Policy>>>,
    cel_engine: Arc<CelEngine>,
    cache: Arc<moka::future::Cache<String, PolicyEvaluationResult>>,
}

impl PolicyEngine {
    pub async fn evaluate_policies(
        &self,
        policy_ids: &[String],
        context: &WorkflowContext,
    ) -> Result<Vec<PolicyEvaluationResult>> {
        let mut results = Vec::new();

        for policy_id in policy_ids {
            let result = self.evaluate_policy(policy_id, context).await?;
            results.push(result);

            // Stop on first violation if configured
            if !result.allowed {
                break;
            }
        }

        Ok(results)
    }

    async fn evaluate_policy(
        &self,
        policy_id: &str,
        context: &WorkflowContext,
    ) -> Result<PolicyEvaluationResult> {
        // Check cache
        let cache_key = format!("{}:{}", policy_id, context.request.request_id);
        if let Some(cached) = self.cache.get(&cache_key).await {
            return Ok(cached);
        }

        let policy = self.get_policy(policy_id)?;

        let result = match policy {
            Policy::RateLimit { rules, .. } => {
                self.evaluate_rate_limit(rules, context).await?
            }
            Policy::Routing { rules, .. } => {
                self.evaluate_routing(rules, context).await?
            }
            Policy::Security { validations, .. } => {
                self.evaluate_security(validations, context).await?
            }
            _ => PolicyEvaluationResult::allowed(),
        };

        // Cache result
        self.cache.insert(cache_key, result.clone()).await;

        Ok(result)
    }

    async fn evaluate_rate_limit(
        &self,
        rules: &[RateLimitRule],
        context: &WorkflowContext,
    ) -> Result<PolicyEvaluationResult> {
        for rule in rules {
            // Evaluate condition using CEL
            if self.cel_engine.evaluate(&rule.condition, context)? {
                // Extract key
                let key = self.render_template(&rule.key_template, context)?;

                // Check rate limit
                if !self.check_rate_limit(&key, &rule).await? {
                    return Ok(PolicyEvaluationResult {
                        allowed: false,
                        reason: Some(format!("Rate limit exceeded for key: {}", key)),
                        metadata: HashMap::new(),
                    });
                }
            }
        }

        Ok(PolicyEvaluationResult::allowed())
    }
}

#[derive(Debug, Clone)]
pub struct PolicyEvaluationResult {
    pub allowed: bool,
    pub reason: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl PolicyEvaluationResult {
    pub fn allowed() -> Self {
        Self {
            allowed: true,
            reason: None,
            metadata: HashMap::new(),
        }
    }
}
```

### Policy Configuration (YAML)

```yaml
# Rate Limiting Policy
policies:
  - type: RateLimit
    id: "global_rate_limit_policy"
    name: "Global Rate Limiting"
    rules:
      # Per-MSISDN limit
      - condition: "true"
        limit_type: SlidingWindow
        key_template: "msisdn:{{request.msisdn}}"
        limit: 10
        window: "1m"
        on_exceeded:
          action: "reject"
          response_code: "RATE_LIMIT_EXCEEDED"

      # Per-operator limit
      - condition: "request.operator_code == 'SAFARICOM'"
        limit_type: TokenBucket
        key_template: "operator:{{request.operator_code}}"
        limit: 100000
        window: "1s"
        on_exceeded:
          action: "throttle"
          delay_ms: 100

  # Routing Policy
  - type: Routing
    id: "operator_routing_policy"
    name: "Operator-Based Routing"
    rules:
      # VIP customers to premium backend
      - condition: "subscriber.tier == 'VIP'"
        targets:
          - backend: "session_manager_premium"
            weight: 100
        priority: 100

      # Canary deployment (5% traffic to v2)
      - condition: "true"
        targets:
          - backend: "session_manager_v2"
            weight: 5
          - backend: "session_manager_v1"
            weight: 95
        priority: 50

  # Security Policy
  - type: Security
    id: "input_validation_policy"
    name: "Input Validation and Security"
    validations:
      # SQL injection prevention
      - type: "sql_injection"
        fields: ["*"]
        action: "reject"

      # XSS prevention
      - type: "xss"
        fields: ["input", "service_code"]
        action: "sanitize"

      # Required fields
      - type: "required_fields"
        fields: ["msisdn", "operator_code"]
        action: "reject"

  # Compliance Policy (PII masking)
  - type: Compliance
    id: "gdpr_compliance_policy"
    name: "GDPR Compliance"
    rules:
      # Mask MSISDN in logs
      - type: "pii_masking"
        fields:
          - name: "msisdn"
            mask_pattern: "***-***-{{last_4}}"
        apply_to: ["logs", "metrics"]

      # Data retention
      - type: "data_retention"
        max_age_days: 90
        fields: ["session_data"]
```

## 3. Specification Registry

### Specification Management

```rust
use openapiv3::OpenAPI;

pub struct SpecificationRegistry {
    /// OpenAPI specifications
    openapi_specs: Arc<RwLock<HashMap<String, OpenAPI>>>,

    /// Protocol Buffer schemas
    protobuf_schemas: Arc<RwLock<HashMap<String, FileDescriptorSet>>>,

    /// JSON schemas
    json_schemas: Arc<RwLock<HashMap<String, serde_json::Value>>>,

    /// Specification versions
    versions: Arc<RwLock<HashMap<String, Vec<SpecVersion>>>>,
}

#[derive(Debug, Clone)]
pub struct SpecVersion {
    pub spec_id: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub deprecated: bool,
    pub breaking_changes: Vec<String>,
}

impl SpecificationRegistry {
    /// Validate request against OpenAPI spec
    pub async fn validate_request(
        &self,
        spec_id: &str,
        version: &str,
        request: &UssdRequest,
    ) -> Result<ValidationResult> {
        let spec = self.get_openapi_spec(spec_id, version)?;

        // Find matching operation
        let operation = self.find_operation(&spec, &request)?;

        // Validate request body
        if let Some(request_body) = &operation.request_body {
            self.validate_request_body(request_body, request)?;
        }

        // Validate parameters
        for parameter in &operation.parameters {
            self.validate_parameter(parameter, request)?;
        }

        Ok(ValidationResult::valid())
    }

    /// Generate mock response from spec
    pub async fn generate_mock_response(
        &self,
        spec_id: &str,
        operation_id: &str,
    ) -> Result<serde_json::Value> {
        let spec = self.get_latest_openapi_spec(spec_id)?;
        let operation = self.find_operation_by_id(&spec, operation_id)?;

        // Generate mock from schema
        if let Some(responses) = &operation.responses {
            if let Some(success_response) = responses.responses.get("200") {
                return self.generate_mock_from_schema(success_response);
            }
        }

        Ok(json!({}))
    }

    /// Detect breaking changes between versions
    pub async fn detect_breaking_changes(
        &self,
        spec_id: &str,
        old_version: &str,
        new_version: &str,
    ) -> Result<Vec<BreakingChange>> {
        let old_spec = self.get_openapi_spec(spec_id, old_version)?;
        let new_spec = self.get_openapi_spec(spec_id, new_version)?;

        let mut breaking_changes = Vec::new();

        // Check for removed endpoints
        for (path, old_item) in &old_spec.paths.paths {
            if !new_spec.paths.paths.contains_key(path) {
                breaking_changes.push(BreakingChange {
                    change_type: "endpoint_removed".to_string(),
                    path: path.clone(),
                    description: format!("Endpoint {} was removed", path),
                });
            }
        }

        // Check for removed/changed fields
        // ... (implementation details)

        Ok(breaking_changes)
    }
}

#[derive(Debug, Clone)]
pub struct BreakingChange {
    pub change_type: String,
    pub path: String,
    pub description: String,
}
```

## 4. Input/Output Mapping Engine

### Mapping DSL (JSON)

```json
{
  "mapping_id": "ss7_to_canonical",
  "version": "1.0",
  "source_format": "ss7_map",
  "target_format": "canonical_ussd",

  "mappings": [
    {
      "source": "$.map.ussdString",
      "target": "$.input",
      "transform": "decode_ussd_string"
    },
    {
      "source": "$.map.msisdn",
      "target": "$.msisdn",
      "transform": "format_e164"
    },
    {
      "source": "$.map.invokeId",
      "target": "$.session_id",
      "transform": "to_string"
    },
    {
      "source": "$.map.serviceCode",
      "target": "$.service_code",
      "transform": "extract_service_code"
    },
    {
      "type": "computed",
      "target": "$.timestamp",
      "expression": "now()"
    },
    {
      "type": "lookup",
      "source": "$.msisdn",
      "lookup_table": "operator_mapping",
      "lookup_key": "prefix",
      "target": "$.operator_code"
    },
    {
      "type": "conditional",
      "condition": "$.map.messageType == 'BEGIN'",
      "then": {
        "target": "$.request_type",
        "value": "Initial"
      },
      "else": {
        "target": "$.request_type",
        "value": "Continue"
      }
    }
  ],

  "transformers": {
    "decode_ussd_string": {
      "type": "function",
      "implementation": "builtin::decode_gsm7"
    },
    "format_e164": {
      "type": "regex",
      "pattern": "^(\\+?254|0)([17]\\d{8})$",
      "replacement": "+254$2"
    },
    "extract_service_code": {
      "type": "regex",
      "pattern": "^\\*([0-9#]+)#$",
      "replacement": "*$1#"
    }
  }
}
```

### Mapping Engine Implementation

```rust
use jsonpath_lib as jsonpath;

pub struct MappingEngine {
    mappings: Arc<RwLock<HashMap<String, MappingDefinition>>>,
    transformer_registry: Arc<TransformerRegistry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MappingDefinition {
    pub mapping_id: String,
    pub version: String,
    pub source_format: String,
    pub target_format: String,
    pub mappings: Vec<FieldMapping>,
    pub transformers: HashMap<String, TransformerConfig>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum FieldMapping {
    Direct {
        source: String,      // JSONPath
        target: String,      // JSONPath
        transform: Option<String>,
    },
    Computed {
        target: String,
        expression: String,  // CEL expression
    },
    Lookup {
        source: String,
        lookup_table: String,
        lookup_key: String,
        target: String,
    },
    Conditional {
        condition: String,
        then_mapping: Box<FieldMapping>,
        else_mapping: Option<Box<FieldMapping>>,
    },
    Aggregate {
        sources: Vec<String>,
        target: String,
        aggregation_fn: String,
    },
}

impl MappingEngine {
    pub async fn transform(
        &self,
        mapping_id: &str,
        source_data: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let mapping = self.get_mapping(mapping_id)?;

        let mut target = json!({});

        for field_mapping in &mapping.mappings {
            match field_mapping {
                FieldMapping::Direct { source, target: target_path, transform } => {
                    // Extract source value using JSONPath
                    let source_value = jsonpath::select(source_data, source)?
                        .first()
                        .cloned()
                        .ok_or_else(|| anyhow::anyhow!("Source path not found: {}", source))?;

                    // Apply transformation if specified
                    let transformed_value = if let Some(transform_name) = transform {
                        self.apply_transform(transform_name, source_value, &mapping)?
                    } else {
                        source_value.clone()
                    };

                    // Set target value using JSONPath
                    self.set_json_path(&mut target, target_path, transformed_value)?;
                }

                FieldMapping::Computed { target: target_path, expression } => {
                    // Evaluate CEL expression
                    let computed_value = self.evaluate_expression(expression, source_data)?;
                    self.set_json_path(&mut target, target_path, computed_value)?;
                }

                FieldMapping::Lookup { source, lookup_table, lookup_key, target: target_path } => {
                    // Extract source value
                    let source_value = jsonpath::select(source_data, source)?
                        .first()
                        .cloned()
                        .ok_or_else(|| anyhow::anyhow!("Source path not found"))?;

                    // Perform lookup
                    let looked_up_value = self.lookup(lookup_table, lookup_key, &source_value).await?;
                    self.set_json_path(&mut target, target_path, looked_up_value)?;
                }

                FieldMapping::Conditional { condition, then_mapping, else_mapping } => {
                    // Evaluate condition
                    if self.evaluate_condition(condition, source_data)? {
                        self.apply_field_mapping(then_mapping, source_data, &mut target).await?;
                    } else if let Some(else_map) = else_mapping {
                        self.apply_field_mapping(else_map, source_data, &mut target).await?;
                    }
                }

                FieldMapping::Aggregate { sources, target: target_path, aggregation_fn } => {
                    // Extract all source values
                    let mut values = Vec::new();
                    for source in sources {
                        if let Ok(vals) = jsonpath::select(source_data, source) {
                            values.extend(vals);
                        }
                    }

                    // Apply aggregation function
                    let aggregated = self.aggregate(&values, aggregation_fn)?;
                    self.set_json_path(&mut target, target_path, aggregated)?;
                }
            }
        }

        Ok(target)
    }

    fn apply_transform(
        &self,
        transform_name: &str,
        value: &serde_json::Value,
        mapping: &MappingDefinition,
    ) -> Result<serde_json::Value> {
        // Get transformer config
        let transformer_config = mapping.transformers.get(transform_name)
            .ok_or_else(|| anyhow::anyhow!("Transformer not found: {}", transform_name))?;

        // Get transformer from registry
        let transformer = self.transformer_registry.get(&transformer_config.transformer_type)?;

        // Apply transformation
        transformer.transform(value, transformer_config)
    }
}

/// Transformer registry for pluggable transformations
pub struct TransformerRegistry {
    transformers: HashMap<String, Arc<dyn Transformer>>,
}

#[async_trait]
pub trait Transformer: Send + Sync {
    fn transform(
        &self,
        value: &serde_json::Value,
        config: &TransformerConfig,
    ) -> Result<serde_json::Value>;
}

/// Example: Regex transformer
pub struct RegexTransformer;

impl Transformer for RegexTransformer {
    fn transform(
        &self,
        value: &serde_json::Value,
        config: &TransformerConfig,
    ) -> Result<serde_json::Value> {
        let input = value.as_str()
            .ok_or_else(|| anyhow::anyhow!("Value must be string for regex transform"))?;

        let pattern = config.get("pattern")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing pattern in config"))?;

        let replacement = config.get("replacement")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing replacement in config"))?;

        let regex = regex::Regex::new(pattern)?;
        let result = regex.replace_all(input, replacement);

        Ok(json!(result.to_string()))
    }
}
```

## 5. Request/Response Enrichment

```rust
pub struct EnrichmentEngine {
    enrichers: Vec<Arc<dyn Enricher>>,
}

#[async_trait]
pub trait Enricher: Send + Sync {
    async fn enrich(&self, context: &mut WorkflowContext) -> Result<()>;
}

/// Geo-location enricher
pub struct GeoLocationEnricher {
    geoip_db: Arc<maxminddb::Reader<Vec<u8>>>,
}

#[async_trait]
impl Enricher for GeoLocationEnricher {
    async fn enrich(&self, context: &mut WorkflowContext) -> Result<()> {
        // Extract country from MSISDN prefix
        let msisdn = &context.request.msisdn;
        let country = self.extract_country_from_msisdn(msisdn)?;

        context.metadata.insert("country".to_string(), country);
        context.metadata.insert("timezone".to_string(), self.get_timezone(&country)?);

        Ok(())
    }
}

/// Subscriber info enricher (from cache/DB)
pub struct SubscriberEnricher {
    cache: Arc<ResponseCache>,
    db_pool: Arc<sqlx::PgPool>,
}

#[async_trait]
impl Enricher for SubscriberEnricher {
    async fn enrich(&self, context: &mut WorkflowContext) -> Result<()> {
        let msisdn = &context.request.msisdn;

        // Try cache first
        let cache_key = format!("subscriber:{}", msisdn);
        if let Some(subscriber_info) = self.cache.get(&cache_key).await? {
            context.variables.insert("subscriber_info".to_string(), subscriber_info);
            return Ok(());
        }

        // Fetch from database
        let subscriber = self.fetch_subscriber_info(msisdn).await?;

        // Cache for future requests
        self.cache.set(&cache_key, subscriber.clone()).await?;

        context.variables.insert("subscriber_info".to_string(), subscriber);

        Ok(())
    }
}
```

---

## Summary

This enhanced Protocol Gateway now includes:

✅ **Workflow Orchestration** - No-code stage-based pipelines
✅ **Policy Engine** - Runtime policy evaluation (rate limiting, routing, security)
✅ **Specification Registry** - OpenAPI/Protobuf validation and versioning
✅ **I/O Mapping Engine** - Declarative field mapping and transformation
✅ **Protocol Mediation** - Version negotiation and compatibility
✅ **Request Enrichment** - Contextual data injection
✅ **Adaptive Routing** - Content-based, weighted, geo-routing
✅ **Analytics Pipeline** - Real-time metrics and telemetry

This transforms the Protocol Gateway into a **full-featured API gateway and integration platform**!
