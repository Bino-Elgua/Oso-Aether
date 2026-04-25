//! WASM bridge for the Ọ̀ṣỌ́ interpreter.
//!
//! Exposes the full interpreter to JavaScript: translate, execute,
//! create agents, and render responses. The frontend calls these
//! functions directly — no Python middleman, no API proxy.
//!
//! Agent state is serialized as JSON between calls. The frontend
//! holds the agent state and passes it back each time.

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use oso_interpreter::{
    Agent, PaymentConfirmation, BIRTH_COST_MIST,
    execute_event, translate, Response,
    DefaultResponder, ResponseGenerator,
    OwnershipRecord, GardenProfile,
};

// ─── Agent Creation ─────────────────────────────────────────────────────────

/// Create a new agent and return its state as JSON.
///
/// Call this after a successful birth payment. The agent starts at
/// reputation 0, Tier 0 — no tools, only think allowed.
/// The Odu key is derived from DNA at birth.
#[wasm_bindgen]
pub fn create_agent(id: &str, name: &str, dna: &str) -> Result<JsValue, JsError> {
    let agent = Agent::new(id.to_string(), name.to_string(), dna.to_string());
    serde_wasm_bindgen::to_value(&agent).map_err(|e| JsError::new(&e.to_string()))
}

// ─── Translation ────────────────────────────────────────────────────────────

/// Translate natural language into a primitive + metadata.
///
/// Returns JSON: { primitive: {...}, original, confidence, slash_command }
/// Returns null if input is empty.
#[wasm_bindgen]
pub fn translate_input(input: &str) -> Result<JsValue, JsError> {
    match translate(input) {
        Some(result) => {
            serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
        }
        None => Ok(JsValue::NULL),
    }
}

// ─── Execution ──────────────────────────────────────────────────────────────

/// JSON wrapper for execution results passed back to JS.
#[derive(Serialize, Deserialize)]
struct ExecuteResult {
    /// The structured event (source of truth).
    event: oso_interpreter::ExecutionEvent,
    /// The user-facing response rendered by DefaultResponder.
    response: Response,
    /// The updated agent state — store this for the next call.
    agent: Agent,
}

/// Execute a primitive against an agent and return the result.
///
/// Takes:
///   - primitive_json: the Primitive from translate_input()
///   - agent_json: the current agent state
///   - payment_json: optional PaymentConfirmation (required for birth)
///
/// Returns JSON: { event, response, agent }
///   - event: the structured ExecutionEvent (source of truth)
///   - response: { message, evolved, reputation_gained }
///   - agent: updated agent state to store for next call
#[wasm_bindgen]
pub fn execute(
    primitive_json: &JsValue,
    agent_json: &JsValue,
    payment_json: &JsValue,
) -> Result<JsValue, JsError> {
    let primitive: oso_parser::Primitive = serde_wasm_bindgen::from_value(primitive_json.clone())
        .map_err(|e| JsError::new(&format!("Invalid primitive: {e}")))?;

    let mut agent: Agent = serde_wasm_bindgen::from_value(agent_json.clone())
        .map_err(|e| JsError::new(&format!("Invalid agent state: {e}")))?;

    let payment: Option<PaymentConfirmation> = if payment_json.is_null() || payment_json.is_undefined() {
        None
    } else {
        Some(serde_wasm_bindgen::from_value(payment_json.clone())
            .map_err(|e| JsError::new(&format!("Invalid payment: {e}")))?)
    };

    let event = execute_event(primitive, &mut agent, payment.as_ref())
        .map_err(|e| JsError::new(&e.to_string()))?;

    let responder = DefaultResponder;
    let response = responder.render(&event);

    let result = ExecuteResult { event, response, agent };
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
}

// ─── Convenience: Translate + Execute in one call ───────────────────────────

/// Process user input end-to-end: translate → execute → respond.
///
/// This is the main entry point for the frontend. Takes natural language,
/// returns the full result with updated agent state.
///
/// Returns null if input is empty.
#[wasm_bindgen]
pub fn process(
    input: &str,
    agent_json: &JsValue,
    payment_json: &JsValue,
) -> Result<JsValue, JsError> {
    let translation = match translate(input) {
        Some(t) => t,
        None => return Ok(JsValue::NULL),
    };

    let mut agent: Agent = serde_wasm_bindgen::from_value(agent_json.clone())
        .map_err(|e| JsError::new(&format!("Invalid agent state: {e}")))?;

    let payment: Option<PaymentConfirmation> = if payment_json.is_null() || payment_json.is_undefined() {
        None
    } else {
        Some(serde_wasm_bindgen::from_value(payment_json.clone())
            .map_err(|e| JsError::new(&format!("Invalid payment: {e}")))?)
    };

    let event = execute_event(translation.primitive, &mut agent, payment.as_ref())
        .map_err(|e| JsError::new(&e.to_string()))?;

    let responder = DefaultResponder;
    let response = responder.render(&event);

    #[derive(Serialize)]
    struct ProcessResult {
        event: oso_interpreter::ExecutionEvent,
        response: Response,
        agent: Agent,
        confidence: f64,
        slash_command: Option<oso_interpreter::SlashCommand>,
    }

    let result = ProcessResult {
        event,
        response,
        agent,
        confidence: translation.confidence,
        slash_command: translation.slash_command,
    };

    serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
}

// ─── Agent State Queries ────────────────────────────────────────────────────

/// Get the agent's current reputation.
#[wasm_bindgen]
pub fn get_reputation(agent_json: &JsValue) -> Result<u64, JsError> {
    let agent: Agent = serde_wasm_bindgen::from_value(agent_json.clone())
        .map_err(|e| JsError::new(&format!("Invalid agent state: {e}")))?;
    Ok(agent.reputation)
}

/// Get the agent's full state as a structured JSON object.
/// Useful for debugging and state inspection.
#[wasm_bindgen]
pub fn get_state(agent_json: &JsValue) -> Result<JsValue, JsError> {
    let agent: Agent = serde_wasm_bindgen::from_value(agent_json.clone())
        .map_err(|e| JsError::new(&format!("Invalid agent state: {e}")))?;

    #[derive(Serialize)]
    struct AgentSummary {
        id: String,
        name: String,
        reputation: u64,
        tier: u8,
        alignment: String,
        personality: oso_interpreter::Personality,
        thought_count: usize,
        private_thought_count: usize,
        action_count: usize,
        sandbox_mode: bool,
        garden_entry_count: usize,
        odu_epoch: u64,
        odu_evolution_count: u64,
    }

    let alignment = match agent.alignment {
        oso_interpreter::Alignment::Light => "Light",
        oso_interpreter::Alignment::Neutral => "Neutral",
        oso_interpreter::Alignment::Shadow => "Shadow",
    };

    let summary = AgentSummary {
        id: agent.id,
        name: agent.name,
        reputation: agent.reputation,
        tier: agent.tier.level(),
        alignment: alignment.to_string(),
        personality: agent.personality,
        thought_count: agent.thoughts.len(),
        private_thought_count: agent.private_thoughts.len(),
        action_count: agent.action_log.len(),
        sandbox_mode: agent.sandbox_mode,
        garden_entry_count: agent.garden_hashes.len(),
        odu_epoch: agent.odu_key.epoch,
        odu_evolution_count: agent.odu_key.evolution_count,
    };

    serde_wasm_bindgen::to_value(&summary).map_err(|e| JsError::new(&e.to_string()))
}

// ─── Ownership Transfer ─────────────────────────────────────────────────────

/// Transfer agent ownership to a new address.
///
/// Rotates the Odu key and ownership record so the old owner
/// permanently loses access to private memory.
///
/// Returns: { agent, transfer_result }
#[wasm_bindgen]
pub fn transfer_ownership(
    agent_json: &JsValue,
    current_owner: &str,
    new_owner: &str,
) -> Result<JsValue, JsError> {
    let mut agent: Agent = serde_wasm_bindgen::from_value(agent_json.clone())
        .map_err(|e| JsError::new(&format!("Invalid agent state: {e}")))?;

    // Create ownership record and transfer
    let mut record = OwnershipRecord::new(agent.id.clone(), current_owner.to_string());
    let transfer = record.transfer(new_owner.to_string());

    // Rotate the agent's Odu key — old owner can never derive the new key
    agent.odu_key.rotate_for_transfer(new_owner);

    #[derive(Serialize)]
    struct TransferOutput {
        agent: Agent,
        from_owner: String,
        to_owner: String,
        new_key_epoch: u64,
    }

    let output = TransferOutput {
        agent,
        from_owner: transfer.from_owner,
        to_owner: transfer.to_owner,
        new_key_epoch: transfer.new_key_version,
    };

    serde_wasm_bindgen::to_value(&output).map_err(|e| JsError::new(&e.to_string()))
}

// ─── Garden ─────────────────────────────────────────────────────────────────

/// Create a new Garden profile for an agent.
#[wasm_bindgen]
pub fn create_garden_profile(
    agent_id: &str,
    name: &str,
    owner: &str,
) -> Result<JsValue, JsError> {
    let profile = GardenProfile::new(
        agent_id.to_string(),
        name.to_string(),
        owner.to_string(),
    );
    serde_wasm_bindgen::to_value(&profile).map_err(|e| JsError::new(&e.to_string()))
}

// ─── Utilities ──────────────────────────────────────────────────────────────

/// Get the birth cost in MIST.
#[wasm_bindgen]
pub fn birth_cost_mist() -> u64 {
    BIRTH_COST_MIST
}

/// Parse raw Ọ̀ṣỌ́ syntax (strict 3-primitive parser).
/// Returns JSON: { command, args } or throws on invalid syntax.
#[wasm_bindgen]
pub fn parse(line: &str) -> Result<JsValue, JsError> {
    let primitive = oso_parser::parse(line)
        .map_err(|e| JsError::new(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&primitive).map_err(|e| JsError::new(&e.to_string()))
}

/// Validate whether a line is valid Ọ̀ṣỌ́ syntax.
#[wasm_bindgen]
pub fn validate(line: &str) -> bool {
    oso_parser::parse(line).is_ok()
}
