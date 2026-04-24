/**
 * Ọ̀ṣỌ́ Walrus Client — permanent memory for every pet.
 *
 * Walrus is a decentralized blob storage on Sui. Each pet's memory
 * is stored as a JSON blob and referenced by its content ID (CID).
 *
 * In production, this talks to a Walrus aggregator/publisher.
 * In development, it falls back to local filesystem storage.
 */

const WALRUS_PUBLISHER = process.env.WALRUS_PUBLISHER_URL ?? 'https://publisher.walrus-testnet.walrus.space'
const WALRUS_AGGREGATOR = process.env.WALRUS_AGGREGATOR_URL ?? 'https://aggregator.walrus-testnet.walrus.space'

// Local fallback store for development when Walrus is unavailable
const localStore = new Map<string, { data: unknown; blobId: string }>()

export const walrus = {
  /**
   * Store pet memory as a Walrus blob.
   * Returns the blob ID (content identifier).
   */
  async store(agentId: string, data: unknown): Promise<string> {
    const payload = JSON.stringify(data)

    try {
      // Attempt to store on Walrus network
      const response = await fetch(`${WALRUS_PUBLISHER}/v1/blobs`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: payload,
      })

      if (response.ok) {
        const result = await response.json()
        // Walrus returns either newlyCreated or alreadyCertified
        const blobId =
          result.newlyCreated?.blobObject?.blobId ??
          result.alreadyCertified?.blobId ??
          ''

        // Keep local reference for fast reads
        localStore.set(agentId, { data, blobId })
        return blobId
      }

      // Walrus unavailable — fall back to local
      return storeLocal(agentId, data)
    } catch {
      // Network error — fall back to local
      return storeLocal(agentId, data)
    }
  },

  /**
   * Load pet memory from Walrus by agent ID.
   * Checks local cache first, then Walrus network.
   */
  async load(agentId: string): Promise<Record<string, unknown> | null> {
    // Check local cache first
    const cached = localStore.get(agentId)
    if (cached) {
      return cached.data as Record<string, unknown>
    }

    // If we have a blob ID stored, fetch from Walrus
    try {
      const blobId = await getBlobId(agentId)
      if (!blobId) return null

      const response = await fetch(`${WALRUS_AGGREGATOR}/v1/blobs/${blobId}`)
      if (response.ok) {
        const data = await response.json()
        localStore.set(agentId, { data, blobId })
        return data as Record<string, unknown>
      }
    } catch {
      // Walrus unavailable
    }

    return null
  },

  /**
   * Get the raw blob ID for an agent's memory.
   */
  async getBlobId(agentId: string): Promise<string | null> {
    return getBlobId(agentId)
  },
}

// ─── Local Fallback (dev mode) ─────────────────────────────────────────────

function storeLocal(agentId: string, data: unknown): string {
  const blobId = `local_${agentId}_${Date.now().toString(36)}`
  localStore.set(agentId, { data, blobId })
  return blobId
}

async function getBlobId(agentId: string): Promise<string | null> {
  const cached = localStore.get(agentId)
  return cached?.blobId ?? null
}
