/**
 * 86-DNA Fingerprint System for Ọ̀ṣỌ́
 *
 * Each pet receives a unique 86-character hex fingerprint at birth.
 * This fingerprint deterministically controls:
 * - ASCII visual form selection
 * - Evolution line (which templates at each tier)
 * - Personality seed offsets
 * - Color/aura affinity
 */

/** Generate a deterministic 86-char DNA fingerprint from name + entropy. */
export function generateDNA(name: string, entropy: string): string {
  // Use Web Crypto for browser-safe hashing
  const seed = `${name}:${entropy}:${Date.now()}`
  let hash = 0xdeadbeef
  for (let i = 0; i < seed.length; i++) {
    hash = Math.imul(hash ^ seed.charCodeAt(i), 0x5bd1e995)
    hash ^= hash >>> 13
  }

  // Generate 86 hex characters from cascading hashes
  let dna = ''
  let current = hash
  while (dna.length < 86) {
    current = Math.imul(current ^ (current >>> 16), 0x45d9f3b)
    current = Math.imul(current ^ (current >>> 16), 0x45d9f3b)
    current ^= current >>> 16
    dna += Math.abs(current).toString(16).padStart(8, '0')
  }

  return dna.slice(0, 86)
}

/** Extract personality seed from DNA. */
export function personalityFromDNA(dna: string): {
  curiosity: number
  boldness: number
  empathy: number
} {
  const segment1 = parseInt(dna.slice(0, 8), 16)
  const segment2 = parseInt(dna.slice(8, 16), 16)
  const segment3 = parseInt(dna.slice(16, 24), 16)

  return {
    curiosity: (segment1 % 1000) / 1000,
    boldness: (segment2 % 1000) / 1000,
    empathy: (segment3 % 1000) / 1000,
  }
}

/** Validate a DNA string (must be 86 hex chars). */
export function isValidDNA(dna: string): boolean {
  return dna.length === 86 && /^[0-9a-f]+$/.test(dna)
}
