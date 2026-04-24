//! WASM bindings for the Ọ̀ṣỌ́ parser.
//! Exposes the strict 3-primitive parser to JavaScript/TypeScript.

use wasm_bindgen::prelude::*;
use serde::Serialize;

use crate::parser;

#[derive(Serialize)]
struct JsPrimitive {
    command: String,
    args: Vec<String>,
}

/// Parse a single line of Ọ̀ṣỌ́ code.
/// Returns a JSON object: { command: "birth"|"think"|"act", args: [...] }
/// Throws on invalid syntax.
#[wasm_bindgen]
pub fn parse(line: &str) -> Result<JsValue, JsError> {
    let primitive = parser::parse(line).map_err(|e| JsError::new(&e.to_string()))?;

    let js_prim = match primitive {
        crate::primitives::Primitive::Birth { name } => JsPrimitive {
            command: "birth".into(),
            args: vec![name],
        },
        crate::primitives::Primitive::Think { intent } => JsPrimitive {
            command: "think".into(),
            args: vec![intent],
        },
        crate::primitives::Primitive::Act { tool, params } => JsPrimitive {
            command: "act".into(),
            args: vec![tool, params],
        },
    };

    serde_wasm_bindgen::to_value(&js_prim).map_err(|e| JsError::new(&e.to_string()))
}

/// Validate whether a line is valid Ọ̀ṣỌ́ syntax.
/// Returns true/false without throwing.
#[wasm_bindgen]
pub fn validate(line: &str) -> bool {
    parser::parse(line).is_ok()
}
