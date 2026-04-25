import { dnaSeed, generatePetAscii, type Personality, type Tier } from './generator.ts'

function personalityFromSeed(seed: number): Personality {
  return {
    curiosity: ((seed & 0xff) % 100) / 100,
    boldness: (((seed >> 8) & 0xff) % 100) / 100,
    empathy: (((seed >> 16) & 0xff) % 100) / 100,
  }
}

/** Get the full 0-5 evolution line for a given DNA. Deterministic. */
export function getEvolutionLine(dna: string): string[] {
  const seed = dnaSeed(dna)
  const personality = personalityFromSeed(seed)
  const tiers: Tier[] = [0, 1, 2, 3, 4, 5]

  return tiers.map((tier) => generatePetAscii(dna, tier, personality))
}
