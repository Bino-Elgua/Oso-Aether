/**
 * Ọ̀ṣỌ́ Client — the bridge between the frontend and Rust.
 *
 * All game logic runs in WASM: translation, execution, response generation.
 * The frontend only handles UI, Walrus storage, and Sui transactions.
 *
 * No Python. No API proxy. Rust is the backend.
 */

import { walrus } from '@/lib/walrus'

// ─── Types from the Rust WASM module ──────────────────────────────────────

type Primitive =
  | { Birth: { name: string } }
  | { Think: { intent: string } }
  | { Act: { tool: string; params: string } }

type AgentState = {
  id: string
  name: string
  dna: string
  tier: 'Zero' | { Awakened: number }
  reputation: number
  alignment: 'Light' | 'Neutral' | 'Shadow'
  light_score: number
  shadow_score: number
  soul: string[]
  thoughts: string[]
  action_log: unknown[]
  decay_log: unknown[]
  personality: { curiosity: number; boldness: number; empathy: number }
  memory_root: string | null
  session: Record<string, string>
  born_at: number
}

type ExecutionEvent = Record<string, unknown>

type ProcessResult = {
  event: ExecutionEvent
  response: { message: string; evolved: boolean; reputation_gained: number }
  agent: AgentState
  confidence: number
  slash_command: unknown | null
}

type ExecuteResult = {
  event: ExecutionEvent
  response: { message: string; evolved: boolean; reputation_gained: number }
  agent: AgentState
}

type TranslationResult = {
  primitive: Primitive
  original: string
  confidence: number
  slash_command: unknown | null
}

// ─── WASM Module (lazy-loaded) ────────────────────────────────────────────

type WasmModule = {
  default: () => Promise<void>
  create_agent: (id: string, name: string, dna: string) => AgentState
  translate_input: (input: string) => TranslationResult | null
  execute: (primitive: Primitive, agent: AgentState, payment: unknown | null) => ExecuteResult
  process: (input: string, agent: AgentState, payment: unknown | null) => ProcessResult | null
  birth_cost_mist: () => bigint
  parse: (line: string) => Primitive
  validate: (line: string) => boolean
}

let wasmModule: WasmModule | null = null

async function loadWasm(): Promise<WasmModule> {
  if (wasmModule) return wasmModule
  const wasm = await import('@/core/wasm/pkg/oso_wasm')
  await wasm.default()
  wasmModule = wasm as unknown as WasmModule
  return wasmModule
}

// ─── Public API ───────────────────────────────────────────────────────────

export const aether = {
  /**
   * Parse strict Ọ̀ṣỌ́ syntax (birth/think/act).
   */
  parse: async (line: string): Promise<Primitive | null> => {
    try {
      const wasm = await loadWasm()
      return wasm.parse(line)
    } catch {
      return null
    }
  },

  /**
   * Validate syntax.
   */
  validate: async (line: string): Promise<boolean> => {
    try {
      const wasm = await loadWasm()
      return wasm.validate(line)
    } catch {
      return true
    }
  },

  /**
   * Translate natural language → primitive.
   * Handles slash commands, birth detection, act detection.
   * Most input becomes think (the default).
   */
  translate: async (input: string): Promise<TranslationResult | null> => {
    const wasm = await loadWasm()
    return wasm.translate_input(input)
  },

  /**
   * Create a new agent. Returns the initial agent state.
   * Call this after birth payment is confirmed on Sui.
   */
  createAgent: async (id: string, name: string, dna: string): Promise<AgentState> => {
    const wasm = await loadWasm()
    return wasm.create_agent(id, name, dna)
  },

  /**
   * Process user input end-to-end: translate → execute → respond.
   * This is the main entry point. Takes natural language, returns
   * the response message and updated agent state.
   *
   * Returns null if input is empty.
   */
  process: async (
    input: string,
    agent: AgentState,
    payment?: { tx_digest: string; amount_mist: number; sender: string },
  ): Promise<ProcessResult | null> => {
    const wasm = await loadWasm()
    return wasm.process(input, agent, payment ?? null)
  },

  /**
   * Execute a specific primitive against an agent.
   * Use this when you already have the primitive (e.g., from translate).
   */
  execute: async (
    primitive: Primitive,
    agent: AgentState,
    payment?: { tx_digest: string; amount_mist: number; sender: string },
  ): Promise<ExecuteResult> => {
    const wasm = await loadWasm()
    return wasm.execute(primitive, agent, payment ?? null)
  },

  /**
   * Get the birth cost in MIST.
   */
  birthCostMist: async (): Promise<bigint> => {
    const wasm = await loadWasm()
    return wasm.birth_cost_mist()
  },

  /**
   * Load agent conversation history from Walrus.
   */
  loadMemory: async (agentId: string) => {
    const response = await fetch(`/api/memory/${agentId}`)
    return response.json()
  },

  /**
   * Store messages to Walrus.
   */
  storeMemory: async (agentId: string, messages: unknown[]) => {
    await fetch(`/api/memory/${agentId}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ messages }),
    })
  },
}

export type { AgentState, ProcessResult, ExecuteResult, TranslationResult, Primitive }
