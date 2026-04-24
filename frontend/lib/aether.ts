type BirthResult = {
  agentId: string
  dna: string
  asciiPreview: string
  walrusCid: string
  suiTxHash: string
}

type IntentResponse = {
  text: string
  growth?: boolean
  newXp?: number
  newTier?: number
  personalityShift?: { curiosity: number; boldness: number; empathy: number }
  newAsciiForm?: string
}

export const aether = {
  birthPet: async (name: string): Promise<BirthResult> => {
    const response = await fetch('/api/translator/birth', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name }),
    })
    return response.json()
  },

  processIntent: async (params: {
    agentId: string
    intent: string
    context: { tier: number; personality: Record<string, number> }
  }): Promise<IntentResponse> => {
    const response = await fetch('/api/translator/think', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(params),
    })
    return response.json()
  },

  loadPetMemory: async (agentId: string) => {
    const response = await fetch(`/api/memory/${agentId}`)
    return response.json()
  },

  storeMemory: async (agentId: string, messages: unknown[]) => {
    await fetch(`/api/memory/${agentId}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ messages }),
    })
  },
}
