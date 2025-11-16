# Service 3: Menu Engine

## Overview

The Menu Engine is the brain of the USSD gateway, responsible for orchestrating menu flows, rendering dynamic content, handling user navigation, and executing business logic. It provides a flexible, template-based menu system with support for personalization, multi-language, and complex navigation patterns.

## Responsibilities

1. **Menu Orchestration**
   - Menu tree traversal and navigation
   - Input validation and routing
   - Back/cancel/timeout handling
   - Pagination for long menus

2. **Dynamic Content Rendering**
   - Template-based menu generation
   - Personalization based on user data
   - Multi-language support (i18n)
   - Variable substitution

3. **Business Logic Execution**
   - Pre/post menu hooks
   - Conditional menu flows
   - Form data collection and validation
   - State machine management

4. **Integration Coordination**
   - Call Integration Hub for external data
   - Cache external responses
   - Handle integration failures gracefully

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│              Menu Engine Service                         │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌────────────────────────────────────┐                 │
│  │     gRPC Server (Port 50052)       │                 │
│  │  - ProcessMenu                      │                 │
│  │  - GetMenuDefinition                │                 │
│  │  - ReloadMenus                      │                 │
│  └──────────────┬─────────────────────┘                 │
│                 ▼                                         │
│  ┌────────────────────────────────────┐                 │
│  │      Menu Orchestrator             │                 │
│  │  - Route based on input             │                 │
│  │  - Execute business logic           │                 │
│  │  - Manage state transitions         │                 │
│  └──────────────┬─────────────────────┘                 │
│                 ▼                                         │
│  ┌────────────────────────────────────┐                 │
│  │       Menu Repository              │                 │
│  │  - Load menu definitions            │                 │
│  │  - Cache menu trees                 │                 │
│  │  - Hot-reload support               │                 │
│  └──────────────┬─────────────────────┘                 │
│                 ▼                                         │
│  ┌────────────────────────────────────┐                 │
│  │    Template Renderer               │                 │
│  │  - Handlebars templates             │                 │
│  │  - Variable substitution            │                 │
│  │  - i18n support                     │                 │
│  └──────────────┬─────────────────────┘                 │
│                 ▼                                         │
│  ┌────────────────────────────────────┐                 │
│  │      Validation Engine             │                 │
│  │  - Input type checking              │                 │
│  │  - Regex patterns                   │                 │
│  │  - Custom validators                │                 │
│  └──────────────┬─────────────────────┘                 │
│                 ▼                                         │
│  ┌────────────────────────────────────┐                 │
│  │     gRPC Client Pools              │                 │
│  │  → Integration Hub                 │                 │
│  │  → Session Manager (callback)      │                 │
│  └────────────────────────────────────┘                 │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

## Data Models

### Menu Definition
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuDefinition {
    /// Unique menu identifier
    pub id: String,

    /// Menu type
    pub menu_type: MenuType,

    /// Menu title/header
    pub title: HashMap<String, String>, // language -> text

    /// Menu text/body template
    pub text: HashMap<String, String>, // language -> template

    /// Available options
    pub options: Vec<MenuOption>,

    /// Input validation rules
    pub validation: Option<ValidationRules>,

    /// Business logic hooks
    pub hooks: Option<MenuHooks>,

    /// Navigation rules
    pub navigation: NavigationRules,

    /// Metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MenuType {
    /// Standard menu with numbered options
    Standard,
    /// Free-form input (e.g., enter amount)
    Input { input_type: InputType },
    /// Confirmation menu (Yes/No)
    Confirmation,
    /// Display-only (no input expected)
    Display,
    /// Paginated list
    Paginated { items_per_page: usize },
    /// Dynamic menu (loaded from external source)
    Dynamic { source: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputType {
    Text,
    Numeric,
    Amount,
    PhoneNumber,
    Pin,
    Date,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuOption {
    /// Option key (what user enters)
    pub key: String,

    /// Display text
    pub label: HashMap<String, String>, // language -> text

    /// Target menu ID or action
    pub target: MenuTarget,

    /// Condition for displaying this option
    pub condition: Option<String>, // Expression string
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MenuTarget {
    /// Navigate to another menu
    Menu { menu_id: String },

    /// Execute an action via Integration Hub
    Action { action_id: String },

    /// End session with message
    End { message: HashMap<String, String> },

    /// Go back to previous menu
    Back,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    /// Minimum length
    pub min_length: Option<usize>,

    /// Maximum length
    pub max_length: Option<usize>,

    /// Regex pattern
    pub pattern: Option<String>,

    /// Custom validator function name
    pub custom_validator: Option<String>,

    /// Error message templates
    pub error_messages: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuHooks {
    /// Execute before menu is rendered
    pub pre_render: Option<String>,

    /// Execute after user input is received
    pub post_input: Option<String>,

    /// Execute before navigating away
    pub pre_navigate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationRules {
    /// Allow back navigation
    pub allow_back: bool,

    /// Allow session cancellation
    pub allow_cancel: bool,

    /// Timeout behavior
    pub timeout_action: Option<MenuTarget>,
}
```

### Menu Context (Runtime)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuRenderContext {
    /// Current session
    pub session: Session,

    /// Selected language
    pub language: String,

    /// User variables
    pub variables: HashMap<String, serde_json::Value>,

    /// External data cache
    pub external_data: HashMap<String, serde_json::Value>,
}

impl MenuRenderContext {
    pub fn new(session: Session) -> Self {
        let language = session.metadata
            .get("language")
            .and_then(|v| v.as_str())
            .unwrap_or("en")
            .to_string();

        Self {
            session,
            language,
            variables: HashMap::new(),
            external_data: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, key: String, value: serde_json::Value) {
        self.variables.insert(key, value);
    }

    pub fn get_variable(&self, key: &str) -> Option<&serde_json::Value> {
        self.variables.get(key)
    }
}
```

## Core Components

### 1. gRPC Service Implementation

```rust
use tonic::{Request, Response, Status};
use menu_engine::menu_engine_server::{MenuEngine, MenuEngineServer};
use menu_engine::{ProcessMenuRequest, ProcessMenuResponse};

pub struct MenuEngineService {
    orchestrator: Arc<MenuOrchestrator>,
    repository: Arc<MenuRepository>,
}

#[tonic::async_trait]
impl MenuEngine for MenuEngineService {
    #[instrument(skip(self))]
    async fn process_menu(
        &self,
        request: Request<ProcessMenuRequest>,
    ) -> Result<Response<ProcessMenuResponse>, Status> {
        let req = request.into_inner();

        // Build render context
        let mut context = MenuRenderContext::new(req.session);

        // Get current menu
        let current_menu_id = context.session.menu_context.current_menu_id.clone();
        let menu = self.repository
            .get_menu(&current_menu_id)
            .await
            .map_err(|e| Status::not_found(e.to_string()))?;

        // Execute pre-input hook if exists
        if let Some(hooks) = &menu.hooks {
            if let Some(hook) = &hooks.post_input {
                self.orchestrator.execute_hook(hook, &mut context, &req.user_input).await?;
            }
        }

        // Validate input
        if let Some(validation) = &menu.validation {
            self.orchestrator.validate_input(&req.user_input, validation, &context)
                .map_err(|e| Status::invalid_argument(e.to_string()))?;
        }

        // Process input and determine next menu
        let next_menu = self.orchestrator
            .process_input(&menu, &req.user_input, &mut context)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Update menu context
        context.session.menu_context.previous_menu_id = Some(current_menu_id);
        context.session.menu_context.current_menu_id = next_menu.id.clone();
        context.session.menu_context.menu_stack.push(next_menu.id.clone());

        // Render next menu
        let rendered_text = self.orchestrator
            .render_menu(&next_menu, &context)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Determine if session should continue
        let should_continue = !matches!(next_menu.menu_type, MenuType::Display);

        Ok(Response::new(ProcessMenuResponse {
            menu_text: rendered_text,
            menu_context: context.session.menu_context,
            should_continue,
        }))
    }

    #[instrument(skip(self))]
    async fn reload_menus(
        &self,
        _request: Request<ReloadMenusRequest>,
    ) -> Result<Response<ReloadMenusResponse>, Status> {
        self.repository.reload_all()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ReloadMenusResponse {
            success: true,
            menus_loaded: self.repository.count(),
        }))
    }
}
```

### 2. Menu Orchestrator

```rust
pub struct MenuOrchestrator {
    repository: Arc<MenuRepository>,
    renderer: Arc<TemplateRenderer>,
    validator: Arc<ValidationEngine>,
    integration_client: Arc<IntegrationHubClient>,
}

impl MenuOrchestrator {
    #[instrument(skip(self))]
    pub async fn process_input(
        &self,
        menu: &MenuDefinition,
        user_input: &str,
        context: &mut MenuRenderContext,
    ) -> Result<MenuDefinition> {
        match &menu.menu_type {
            MenuType::Standard => {
                // Find matching option
                let option = menu.options.iter()
                    .find(|opt| opt.key == user_input)
                    .ok_or_else(|| anyhow::anyhow!("Invalid option"))?;

                // Check condition if present
                if let Some(condition) = &option.condition {
                    if !self.evaluate_condition(condition, context)? {
                        return Err(anyhow::anyhow!("Option not available"));
                    }
                }

                // Process target
                self.process_target(&option.target, context).await
            }

            MenuType::Input { input_type } => {
                // Store input in context
                let var_name = format!("{}_input", menu.id);
                context.set_variable(var_name, json!(user_input));

                // Navigate to next menu (configured in metadata or first option)
                let next_menu_id = menu.metadata
                    .get("next_menu")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("No next menu configured"))?;

                self.repository.get_menu(next_menu_id).await
            }

            MenuType::Confirmation => {
                // Handle Yes/No (1/2)
                match user_input {
                    "1" => {
                        // Yes option
                        let yes_target = &menu.options[0].target;
                        self.process_target(yes_target, context).await
                    }
                    "2" => {
                        // No option
                        let no_target = &menu.options[1].target;
                        self.process_target(no_target, context).await
                    }
                    _ => Err(anyhow::anyhow!("Invalid confirmation input")),
                }
            }

            MenuType::Paginated { items_per_page } => {
                self.process_paginated_input(menu, user_input, context, *items_per_page).await
            }

            MenuType::Dynamic { source } => {
                self.process_dynamic_menu(menu, user_input, context, source).await
            }

            MenuType::Display => {
                // Display menus don't process input
                Err(anyhow::anyhow!("Display menus don't accept input"))
            }
        }
    }

    async fn process_target(
        &self,
        target: &MenuTarget,
        context: &mut MenuRenderContext,
    ) -> Result<MenuDefinition> {
        match target {
            MenuTarget::Menu { menu_id } => {
                self.repository.get_menu(menu_id).await
            }

            MenuTarget::Action { action_id } => {
                // Call Integration Hub to execute action
                let result = self.integration_client
                    .execute_action(action_id, context)
                    .await?;

                // Store result in context
                context.external_data.insert(action_id.clone(), result.data);

                // Get next menu from result or metadata
                let next_menu_id = result.next_menu_id
                    .ok_or_else(|| anyhow::anyhow!("Action didn't return next menu"))?;

                self.repository.get_menu(&next_menu_id).await
            }

            MenuTarget::End { message } => {
                // Create a temporary display menu for end message
                let lang = &context.language;
                let end_text = message.get(lang)
                    .or_else(|| message.get("en"))
                    .ok_or_else(|| anyhow::anyhow!("No end message for language"))?;

                Ok(MenuDefinition {
                    id: "END".to_string(),
                    menu_type: MenuType::Display,
                    title: HashMap::new(),
                    text: [(lang.clone(), end_text.clone())].into(),
                    options: Vec::new(),
                    validation: None,
                    hooks: None,
                    navigation: NavigationRules {
                        allow_back: false,
                        allow_cancel: false,
                        timeout_action: None,
                    },
                    metadata: HashMap::new(),
                })
            }

            MenuTarget::Back => {
                // Get previous menu from stack
                let prev_menu_id = context.session.menu_context.previous_menu_id
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("No previous menu"))?;

                self.repository.get_menu(prev_menu_id).await
            }
        }
    }

    fn evaluate_condition(&self, condition: &str, context: &MenuRenderContext) -> Result<bool> {
        // Simple expression evaluator
        // In production, use a proper expression engine like `evalexpr`

        // Example: "balance > 100"
        if condition.contains('>') {
            let parts: Vec<&str> = condition.split('>').collect();
            let var_name = parts[0].trim();
            let threshold: f64 = parts[1].trim().parse()?;

            if let Some(value) = context.get_variable(var_name) {
                if let Some(num) = value.as_f64() {
                    return Ok(num > threshold);
                }
            }
        }

        // Default to true if can't evaluate
        Ok(true)
    }

    pub async fn execute_hook(
        &self,
        hook: &str,
        context: &mut MenuRenderContext,
        input: &str,
    ) -> Result<()> {
        // Execute custom business logic
        // In production, this could load Lua scripts or WASM modules

        match hook {
            "load_user_balance" => {
                let balance = self.integration_client
                    .get_balance(&context.session.msisdn)
                    .await?;
                context.set_variable("balance".to_string(), json!(balance));
            }
            "validate_pin" => {
                let valid = self.integration_client
                    .validate_pin(&context.session.msisdn, input)
                    .await?;
                if !valid {
                    return Err(anyhow::anyhow!("Invalid PIN"));
                }
            }
            _ => {
                warn!("Unknown hook: {}", hook);
            }
        }

        Ok(())
    }
}
```

### 3. Template Renderer

```rust
use handlebars::Handlebars;

pub struct TemplateRenderer {
    handlebars: Handlebars<'static>,
}

impl TemplateRenderer {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();

        // Register helpers
        handlebars.register_helper("currency", Box::new(currency_helper));
        handlebars.register_helper("truncate", Box::new(truncate_helper));

        Self { handlebars }
    }

    pub fn render(
        &self,
        template: &str,
        context: &MenuRenderContext,
    ) -> Result<String> {
        // Build render data
        let mut data = serde_json::Map::new();

        // Add session data
        data.insert("msisdn".to_string(), json!(context.session.msisdn));
        data.insert("session_id".to_string(), json!(context.session.session_id));

        // Add user variables
        for (key, value) in &context.variables {
            data.insert(key.clone(), value.clone());
        }

        // Add external data
        for (key, value) in &context.external_data {
            data.insert(key.clone(), value.clone());
        }

        // Render template
        let rendered = self.handlebars.render_template(template, &data)?;

        // Ensure USSD length limits (182 chars for GSM 7-bit)
        Ok(self.truncate_ussd_text(rendered))
    }

    fn truncate_ussd_text(&self, text: String) -> String {
        const MAX_USSD_LENGTH: usize = 160; // Conservative limit

        if text.chars().count() <= MAX_USSD_LENGTH {
            return text;
        }

        // Truncate and add ellipsis
        let mut truncated: String = text.chars()
            .take(MAX_USSD_LENGTH - 3)
            .collect();
        truncated.push_str("...");

        truncated
    }
}

// Custom Handlebars helpers
fn currency_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0)
        .and_then(|v| v.value().as_f64())
        .ok_or_else(|| handlebars::RenderError::new("Invalid currency value"))?;

    out.write(&format!("KES {:.2}", param))?;
    Ok(())
}

fn truncate_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let text = h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| handlebars::RenderError::new("Invalid text"))?;

    let max_len = h.param(1)
        .and_then(|v| v.value().as_u64())
        .unwrap_or(50) as usize;

    let truncated = if text.len() > max_len {
        format!("{}...", &text[..max_len])
    } else {
        text.to_string()
    };

    out.write(&truncated)?;
    Ok(())
}
```

### 4. Menu Repository

```rust
use std::sync::RwLock;

pub struct MenuRepository {
    menus: Arc<RwLock<HashMap<String, MenuDefinition>>>,
    db_pool: Arc<sqlx::PgPool>,
    cache: Arc<moka::future::Cache<String, MenuDefinition>>,
}

impl MenuRepository {
    pub async fn new(db_pool: Arc<sqlx::PgPool>) -> Result<Self> {
        let cache = Arc::new(
            moka::future::Cache::builder()
                .max_capacity(1000)
                .time_to_live(Duration::from_secs(3600))
                .build()
        );

        let repository = Self {
            menus: Arc::new(RwLock::new(HashMap::new())),
            db_pool,
            cache,
        };

        // Load all menus on startup
        repository.reload_all().await?;

        Ok(repository)
    }

    pub async fn get_menu(&self, menu_id: &str) -> Result<MenuDefinition> {
        // Try cache first
        if let Some(menu) = self.cache.get(menu_id).await {
            return Ok(menu);
        }

        // Try in-memory map
        {
            let menus = self.menus.read().unwrap();
            if let Some(menu) = menus.get(menu_id) {
                self.cache.insert(menu_id.to_string(), menu.clone()).await;
                return Ok(menu.clone());
            }
        }

        // Load from database
        let menu = self.load_from_db(menu_id).await?;
        self.cache.insert(menu_id.to_string(), menu.clone()).await;

        Ok(menu)
    }

    async fn load_from_db(&self, menu_id: &str) -> Result<MenuDefinition> {
        let record = sqlx::query!(
            "SELECT definition FROM menu_definitions WHERE id = $1",
            menu_id
        )
        .fetch_one(&*self.db_pool)
        .await?;

        let menu: MenuDefinition = serde_json::from_value(record.definition)?;
        Ok(menu)
    }

    pub async fn reload_all(&self) -> Result<()> {
        let records = sqlx::query!("SELECT definition FROM menu_definitions")
            .fetch_all(&*self.db_pool)
            .await?;

        let mut menus = HashMap::new();
        for record in records {
            let menu: MenuDefinition = serde_json::from_value(record.definition)?;
            menus.insert(menu.id.clone(), menu);
        }

        {
            let mut map = self.menus.write().unwrap();
            *map = menus;
        }

        // Clear cache to force reload
        self.cache.invalidate_all();

        info!("Reloaded {} menus", self.count());

        Ok(())
    }

    pub fn count(&self) -> usize {
        self.menus.read().unwrap().len()
    }
}
```

### 5. Validation Engine

```rust
use regex::Regex;
use validator::Validate;

pub struct ValidationEngine;

impl ValidationEngine {
    pub fn validate(
        &self,
        input: &str,
        rules: &ValidationRules,
        _context: &MenuRenderContext,
    ) -> Result<()> {
        // Length validation
        if let Some(min_len) = rules.min_length {
            if input.len() < min_len {
                return Err(anyhow::anyhow!("Input too short"));
            }
        }

        if let Some(max_len) = rules.max_length {
            if input.len() > max_len {
                return Err(anyhow::anyhow!("Input too long"));
            }
        }

        // Pattern validation
        if let Some(pattern) = &rules.pattern {
            let regex = Regex::new(pattern)?;
            if !regex.is_match(input) {
                return Err(anyhow::anyhow!("Invalid format"));
            }
        }

        // Custom validators
        if let Some(validator_name) = &rules.custom_validator {
            self.run_custom_validator(validator_name, input)?;
        }

        Ok(())
    }

    fn run_custom_validator(&self, name: &str, input: &str) -> Result<()> {
        match name {
            "phone_number" => {
                let regex = Regex::new(r"^(?:\+254|0)[17]\d{8}$")?;
                if !regex.is_match(input) {
                    return Err(anyhow::anyhow!("Invalid phone number"));
                }
            }
            "amount" => {
                let amount: f64 = input.parse()
                    .map_err(|_| anyhow::anyhow!("Invalid amount"))?;
                if amount <= 0.0 {
                    return Err(anyhow::anyhow!("Amount must be positive"));
                }
            }
            _ => {
                warn!("Unknown validator: {}", name);
            }
        }

        Ok(())
    }
}
```

## Sample Menu Definitions (JSON)

### Main Menu
```json
{
  "id": "main_menu",
  "menu_type": "Standard",
  "title": {
    "en": "Welcome",
    "sw": "Karibu"
  },
  "text": {
    "en": "Welcome to MyService\n1. Check Balance\n2. Send Money\n3. Buy Airtime\n0. Exit",
    "sw": "Karibu MyService\n1. Angalia Salio\n2. Tuma Pesa\n3. Nunua Airtime\n0. Toka"
  },
  "options": [
    {
      "key": "1",
      "label": {"en": "Check Balance", "sw": "Angalia Salio"},
      "target": {"type": "Menu", "menu_id": "check_balance"}
    },
    {
      "key": "2",
      "label": {"en": "Send Money", "sw": "Tuma Pesa"},
      "target": {"type": "Menu", "menu_id": "send_money"}
    },
    {
      "key": "3",
      "label": {"en": "Buy Airtime", "sw": "Nunua Airtime"},
      "target": {"type": "Menu", "menu_id": "buy_airtime"}
    },
    {
      "key": "0",
      "label": {"en": "Exit", "sw": "Toka"},
      "target": {
        "type": "End",
        "message": {"en": "Thank you for using MyService", "sw": "Asante kwa kutumia MyService"}
      }
    }
  ],
  "navigation": {
    "allow_back": false,
    "allow_cancel": true,
    "timeout_action": null
  }
}
```

### Dynamic Balance Menu
```json
{
  "id": "check_balance",
  "menu_type": "Display",
  "title": {
    "en": "Account Balance",
    "sw": "Salio la Akaunti"
  },
  "text": {
    "en": "Your balance is {{currency balance}}\n\nThank you.",
    "sw": "Salio lako ni {{currency balance}}\n\nAsante."
  },
  "hooks": {
    "pre_render": "load_user_balance"
  },
  "navigation": {
    "allow_back": true,
    "allow_cancel": true
  }
}
```

## Configuration

```toml
[service]
name = "menu-engine"
port = 50052
metrics_port = 9092

[database]
url = "postgresql://user:pass@postgres:5432/ussd_gw"
max_connections = 20

[cache]
max_menus = 1000
ttl_seconds = 3600

[integration_hub]
endpoint = "http://integration-hub:50053"
timeout_ms = 15000

[rendering]
max_text_length = 160
default_language = "en"
supported_languages = ["en", "sw"]

[validation]
enable_strict_validation = true
```

## Performance Optimizations

1. **Menu Caching**: All menus cached in memory
2. **Lazy Loading**: Dynamic menus loaded on-demand
3. **Template Pre-compilation**: Handlebars templates compiled once
4. **Async Hooks**: Non-blocking integration calls

---

**Next**: [Service 4: Integration Hub](./04-INTEGRATION-HUB-SERVICE.md)
