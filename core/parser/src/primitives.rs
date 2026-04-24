/// The only three primitives that exist in Ọ̀ṣỌ́.
/// Nothing else is permitted. No extensions. No escape hatches.
#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    /// birth "name" — Creates a new pet agent (mints a dNFT)
    Birth { name: String },
    /// think "natural language intent" — The pet's brain and reasoning
    Think { intent: String },
    /// act "tool" "parameters" — Executes the task and updates growth
    Act { tool: String, params: String },
}
