/**
 * Growth & Evolution System for Ọ̀ṣỌ́
 *
 * Pets grow through usage. Every interaction earns XP.
 * XP thresholds trigger tier evolution (visual + capability changes).
 */

import type { Tier, Personality } from '../ascii-renderer/generator.ts'

export const TIER_TITLES: Record<Tier, string> = {
  0: 'Newborn',
  1: 'Common',
  2: 'Uncommon',
  3: 'Rare',
  4: 'Epic',
  5: 'Ọ̀ṣỌ́ Sovereign',
}

export const TOOL_UNLOCKS = [
  { name: 'memory_recall', label: 'Memory Recall', tier: 0 as Tier, description: 'Holds on to your recent bond and returns to it.' },
  { name: 'web_search', label: 'Web Search', tier: 2 as Tier, description: 'Looks outward and brings knowledge back to the mask.' },
  { name: 'image_gen', label: 'Image Generation', tier: 3 as Tier, description: 'Turns visions into form and symbols.' },
  { name: 'code_exec', label: 'Code Helper', tier: 4 as Tier, description: 'Acts directly on structured tasks and experiments.' },
] as const

/** XP thresholds for each tier. */
export const TIER_THRESHOLDS: Record<Tier, number> = {
  0: 0,
  1: 25,
  2: 100,
  3: 500,
  4: 2000,
  5: 10000,
}

/** Calculate current tier from XP. */
export function tierFromXP(xp: number): Tier {
  if (xp >= 10000) return 5
  if (xp >= 2000) return 4
  if (xp >= 500) return 3
  if (xp >= 100) return 2
  if (xp >= 25) return 1
  return 0
}

/** Calculate progress percentage toward next tier. */
export function progressToNextTier(xp: number): { current: Tier; progress: number; nextThreshold: number } {
  const current = tierFromXP(xp)
  if (current === 5) return { current, progress: 1, nextThreshold: 10000 }

  const nextTier = (current + 1) as Tier
  const currentThreshold = TIER_THRESHOLDS[current]
  const nextThreshold = TIER_THRESHOLDS[nextTier]
  const progress = (xp - currentThreshold) / (nextThreshold - currentThreshold)

  return { current, progress: Math.min(1, Math.max(0, progress)), nextThreshold }
}

/** XP rewards for different interaction types. */
export const XP_REWARDS = {
  chat: 5,
  think: 5,
  act_success: 10,
  act_failure: 2,
  birth: 0,
} as const

export function getUnlockedTools(tier: Tier) {
  return TOOL_UNLOCKS.filter((tool) => tool.tier <= tier)
}

/** Apply personality shift with damping (traits converge slower at extremes). */
export function shiftPersonality(
  current: Personality,
  delta: Partial<Personality>,
): Personality {
  const damp = (val: number, shift: number) => {
    const dampingFactor = 1 - Math.abs(val - 0.5) * 0.5
    return Math.min(1, Math.max(0, val + shift * dampingFactor))
  }

  return {
    curiosity: damp(current.curiosity, delta.curiosity ?? 0),
    boldness: damp(current.boldness, delta.boldness ?? 0),
    empathy: damp(current.empathy, delta.empathy ?? 0),
  }
}
