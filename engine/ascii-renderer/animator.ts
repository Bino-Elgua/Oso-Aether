import { generatePetAscii, type Personality, type Tier, type Mood } from './generator.ts'

export type AnimationMode = 'idle' | 'thinking' | 'happy' | 'evolving'

const animationFrames: Record<AnimationMode, Mood[]> = {
  idle: ['idle', 'inhale', 'idle', 'blink', 'exhale', 'drift-left', 'idle', 'drift-right'],
  thinking: ['thinking', 'drift-left', 'thinking', 'blink', 'thinking', 'drift-right'],
  happy: ['happy', 'inhale', 'happy', 'blink', 'happy', 'drift-right'],
  evolving: ['evolve-a', 'evolve-b', 'evolve-a', 'blink', 'evolve-b', 'happy'],
}

/** Generates animation frames for the chosen pet state. */
export function generateAsciiFrames(
  dna: string,
  tier: Tier,
  personality: Personality,
  mode: AnimationMode = 'idle',
): string[] {
  return animationFrames[mode].map((mood) => generatePetAscii(dna, tier, personality, mood))
}

/** Interpolates visual weight between tiers for evolution animation. */
export function getEvolutionProgress(currentXp: number, targetTier: Tier): number {
  const thresholds = [0, 25, 100, 500, 2000, 10000]
  if (targetTier === 0) return 0
  const prev = thresholds[targetTier - 1]
  const next = thresholds[targetTier]
  return Math.min(1, Math.max(0, (currentXp - prev) / (next - prev)))
}
