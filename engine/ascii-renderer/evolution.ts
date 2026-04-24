import { OMO_TEMPLATES } from './registry'
import { dnaSeed } from './generator'

/** Get the full 5-tier evolution line for a given DNA. Deterministic. */
export function getEvolutionLine(dna: string): string[] {
  const seed = dnaSeed(dna)
  const pick = (arr: string[]) => arr[seed % arr.length]

  return [
    pick(OMO_TEMPLATES.tier1),
    pick(OMO_TEMPLATES.tier2),
    pick(OMO_TEMPLATES.tier3),
    pick(OMO_TEMPLATES.tier4),
    pick(OMO_TEMPLATES.tier5),
  ]
}
