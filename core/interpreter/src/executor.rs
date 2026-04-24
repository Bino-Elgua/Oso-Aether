use oso_parser::Primitive;
use crate::state::AgentState;
use anyhow::Result;

/// Result of executing a primitive.
#[derive(Debug)]
pub struct ExecutionResult {
    pub output: String,
    pub growth_delta: Option<GrowthDelta>,
}

#[derive(Debug)]
pub struct GrowthDelta {
    pub xp: u64,
    pub curiosity_shift: f64,
    pub boldness_shift: f64,
    pub empathy_shift: f64,
}

/// Execute a single Ọ̀ṣỌ́ primitive against the agent state.
pub async fn execute(primitive: Primitive, state: &mut AgentState) -> Result<ExecutionResult> {
    match primitive {
        Primitive::Birth { name } => {
            let agent_id = format!("pet_{}_{}", name, uuid::Uuid::new_v4().simple());
            state.agent_id = Some(agent_id.clone());
            state.name = Some(name.clone());
            state.tier = 1;
            state.xp = 0;

            Ok(ExecutionResult {
                output: format!("__BIRTH_OK__:{agent_id}"),
                growth_delta: None,
            })
        }

        Primitive::Think { intent } => {
            // Hash intent for audit trail
            let plan_hash = blake3::hash(intent.as_bytes()).to_hex().to_string();

            // Thinking rewards curiosity
            let growth = GrowthDelta {
                xp: 5,
                curiosity_shift: 0.02,
                boldness_shift: 0.0,
                empathy_shift: 0.01,
            };

            Ok(ExecutionResult {
                output: format!("__THINK_OK__:{plan_hash}"),
                growth_delta: Some(growth),
            })
        }

        Primitive::Act { tool, params } => {
            // Hash execution receipt
            let receipt = format!("executed:{tool} with:{params}");
            let receipt_hash = blake3::hash(receipt.as_bytes()).to_hex().to_string();

            // Acting rewards boldness
            let growth = GrowthDelta {
                xp: 10,
                curiosity_shift: 0.0,
                boldness_shift: 0.03,
                empathy_shift: 0.0,
            };

            Ok(ExecutionResult {
                output: format!("__ACT_OK__:{receipt_hash}"),
                growth_delta: Some(growth),
            })
        }
    }
}
