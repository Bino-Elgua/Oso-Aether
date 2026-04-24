import { generatePetAscii, Personality, Tier, Mood } from './generator'

/** Generates 3 mood frames for CSS transition animation. */
export function generateAsciiFrames(
  dna: string,
  tier: Tier,
  personality: Personality,
): string[] {
  const moods: Mood[] = ['neutral', 'happy', 'focused']
  return moods.map((mood) => generatePetAscii(dna, tier, personality, mood))
}

/** Interpolates visual weight between tiers for evolution animation. */
export function getEvolutionProgress(currentXp: number, targetTier: Tier): number {
  const thresholds = [0, 100, 500, 2000, 10000]
  if (targetTier === 1) return 0
  const prev = thresholds[targetTier - 2]
  const next = thresholds[targetTier - 1]
  return Math.min(1, Math.max(0, (currentXp - prev) / (next - prev)))
}
