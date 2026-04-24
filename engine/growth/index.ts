/**
 * Growth & Evolution System for Ọ̀ṣỌ́
 *
 * Pets grow through usage. Every interaction earns XP.
 * XP thresholds trigger tier evolution (visual + capability changes).
 */

import type { Tier, Personality } from '../ascii-renderer/generator'

/** XP thresholds for each tier. */
export const TIER_THRESHOLDS: Record<Tier, number> = {
  1: 0,
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
  return 1
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

/** Apply personality shift with damping (traits converge slower at extremes). */
export function shiftPersonality(
  current: Personality,
  delta: Partial<Personality>,
): Personality {
  const damp = (val: number, shift: number) => {
    // Shifts are dampened near 0 and 1
    const dampingFactor = 1 - Math.abs(val - 0.5) * 0.5
    return Math.min(1, Math.max(0, val + shift * dampingFactor))
  }

  return {
    curiosity: damp(current.curiosity, delta.curiosity ?? 0),
    boldness: damp(current.boldness, delta.boldness ?? 0),
    empathy: damp(current.empathy, delta.empathy ?? 0),
  }
}
