/**
 * Ọ̀ṣỌ́ Client — the bridge between the frontend and all backend systems.
 *
 * Connects: WASM parser + API routes (Claude translator) + Walrus + Sui
 */

type BirthResult = {
  agentId: string
  dna: string
  asciiPreview: string
  personality: { curiosity: number; boldness: number; empathy: number }
  greeting: string
  originStory: string
  walrusCid: string
  suiTxHash: string
}

type IntentResponse = {
  text: string
  primitives: Array<{ command: string; args: string[] }>
  growth?: boolean
  newXp?: number
  newTier?: number
  evolved?: boolean
  personalityShift?: { curiosity: number; boldness: number; empathy: number }
  newAsciiForm?: string
}

type ParsedCommand = {
  command: 'birth' | 'think' | 'act'
  args: string[]
}

// ─── WASM Parser (lazy-loaded) ─────────────────────────────────────────────

let wasmParser: { parse: (line: string) => ParsedCommand; validate: (line: string) => boolean } | null = null

async function loadWasmParser() {
  if (wasmParser) return wasmParser
  try {
    const wasm = await import('@/core/wasm/pkg/oso_parser')
    await wasm.default()
    wasmParser = {
      parse: (line: string) => wasm.parse(line),
      validate: (line: string) => wasm.validate(line),
    }
    return wasmParser
  } catch {
    // WASM not built — return null, API routes handle parsing server-side
    return null
  }
}

// ─── Public API ────────────────────────────────────────────────────────────

export const aether = {
  /**
   * Parse a line of Ọ̀ṣỌ́ syntax client-side (WASM).
   * Returns null if WASM is not available.
   */
  parse: async (line: string): Promise<ParsedCommand | null> => {
    const parser = await loadWasmParser()
    if (!parser) return null
    return parser.parse(line)
  },

  /**
   * Validate syntax client-side (WASM).
   */
  validate: async (line: string): Promise<boolean> => {
    const parser = await loadWasmParser()
    if (!parser) return true // Assume valid if WASM unavailable; server validates
    return parser.validate(line)
  },

  /**
   * Birth a new pet — calls Claude to forge the soul, generates 86-DNA.
   */
  birthPet: async (name: string): Promise<BirthResult> => {
    const response = await fetch('/api/translator/birth', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name }),
    })
    if (!response.ok) {
      const err = await response.json().catch(() => ({}))
      throw new Error(err.error ?? 'Birth failed')
    }
    return response.json()
  },

  /**
   * Process natural language intent — Claude translates to primitives,
   * calculates growth, and returns the pet's in-character response.
   */
  processIntent: async (params: {
    agentId: string
    intent: string
    context: {
      name?: string
      tier: number
      xp?: number
      dna?: string
      personality: Record<string, number>
      recentMemory?: string
    }
  }): Promise<IntentResponse> => {
    const response = await fetch('/api/translator/think', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(params),
    })
    if (!response.ok) {
      const err = await response.json().catch(() => ({}))
      throw new Error(err.error ?? 'Think failed')
    }
    return response.json()
  },

  /**
   * Load pet conversation history from Walrus.
   */
  loadPetMemory: async (agentId: string) => {
    const response = await fetch(`/api/memory/${agentId}`)
    return response.json()
  },

  /**
   * Store new messages to Walrus.
   */
  storeMemory: async (agentId: string, messages: unknown[]) => {
    await fetch(`/api/memory/${agentId}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ messages }),
    })
  },
}
