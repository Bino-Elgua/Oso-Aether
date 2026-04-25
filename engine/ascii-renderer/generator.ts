import { OMO_TEMPLATES, type PetTemplate } from './registry.ts'

export type Personality = { curiosity: number; boldness: number; empathy: number }
export type Tier = 0 | 1 | 2 | 3 | 4 | 5
export type Mood =
  | 'idle'
  | 'blink'
  | 'inhale'
  | 'exhale'
  | 'drift-left'
  | 'drift-right'
  | 'thinking'
  | 'happy'
  | 'evolve-a'
  | 'evolve-b'

export type PetPalette = {
  foreground: string
  accent: string
  glow: string
  ansi: string
}

/** Deterministic seed from DNA string. */
export function dnaSeed(dna: string, salt: string = ''): number {
  let h = 0xdeadbeef
  const seedStr = dna + salt
  for (let i = 0; i < seedStr.length; i++) {
    const c = seedStr.charCodeAt(i)
    h = Math.imul(h ^ c, 0x5bd1e995)
    h ^= h >>> 13
  }
  return Math.abs((h ^ (h >>> 16)) >>> 0)
}

/** Deterministic picker from an array based on DNA + salt. */
function pick<T>(dna: string, salt: string, options: T[]): T {
  return options[dnaSeed(dna, salt) % options.length]
}

function renderLine(line: string, slots: Record<string, string>): string {
  return line.replace(/\{(\w+)\}/g, (_, key: string) => slots[key] ?? '')
}

function dominantTrait(personality: Personality): keyof Personality | 'balanced' {
  const traits: Array<[keyof Personality, number]> = [
    ['curiosity', personality.curiosity],
    ['boldness', personality.boldness],
    ['empathy', personality.empathy],
  ]

  let trait: keyof Personality = 'curiosity'
  let score = personality.curiosity

  for (const [candidateTrait, candidateScore] of traits) {
    if (candidateScore > score) {
      trait = candidateTrait
      score = candidateScore
    }
  }

  return score < 0.62 ? 'balanced' : trait
}

export function getPetPalette(tier: Tier, personality: Personality): PetPalette {
  const intensity = 0.2 + tier * 0.12
  switch (dominantTrait(personality)) {
    case 'curiosity':
      return {
        foreground: '#8ce7ff',
        accent: '#d7fbff',
        glow: `rgba(72, 209, 255, ${intensity.toFixed(2)})`,
        ansi: '\u001b[96;1m',
      }
    case 'boldness':
      return {
        foreground: '#ffb067',
        accent: '#ffd4a5',
        glow: `rgba(255, 138, 61, ${intensity.toFixed(2)})`,
        ansi: '\u001b[93;1m',
      }
    case 'empathy':
      return {
        foreground: '#8cf0c1',
        accent: '#d8ffe9',
        glow: `rgba(84, 225, 156, ${intensity.toFixed(2)})`,
        ansi: '\u001b[92;1m',
      }
    default:
      return {
        foreground: '#f2dfb4',
        accent: '#fff4d7',
        glow: `rgba(255, 208, 108, ${intensity.toFixed(2)})`,
        ansi: '\u001b[97;1m',
      }
  }
}

export function getPetTemplate(dna: string, tier: Tier): PetTemplate {
  return pick(dna, `template-${tier}`, OMO_TEMPLATES[tier])
}

function eyePair(dna: string, tier: Tier, mood: Mood): string {
  if (mood === 'blink') return tier >= 4 ? '─ ─' : '– –'
  if (mood === 'thinking') return tier >= 3 ? '◔ ◕' : tier === 0 ? '· •' : 'o O'
  if (mood === 'happy') return tier >= 3 ? '◕ ◕' : '^ ^'
  if (mood === 'evolve-a' || mood === 'evolve-b') return tier >= 4 ? '◆ ◆' : '◉ ◉'

  switch (tier) {
    case 0:
      return pick(dna, 'eyes-0', ['· ·', '• •', '° °'])
    case 1:
      return pick(dna, 'eyes-1', ['o o', '• •', '. .'])
    case 2:
      return pick(dna, 'eyes-2', ['◕ ◕', '◉ ◉', '• •'])
    case 3:
      return pick(dna, 'eyes-3', ['◉ ◉', '◆ ◆', '◈ ◈'])
    case 4:
      return pick(dna, 'eyes-4', ['◈ ◈', '✦ ✦', '◆ ◆'])
    case 5:
      return pick(dna, 'eyes-5', ['◆ ◆', '◈ ◈', '◉ ◉'])
    default:
      return '• •'
  }
}

function mouthGlyph(personality: Personality, tier: Tier, mood: Mood): string {
  if (mood === 'thinking') return tier >= 3 ? '≋≋≋' : '···'
  if (mood === 'happy') return tier >= 3 ? '⌣⌣⌣' : '⌣⌣'
  if (mood === 'evolve-a') return '✶✶✶'
  if (mood === 'evolve-b') return '∞∞∞'
  if (mood === 'blink') return tier === 0 ? '˘' : '──'

  if (personality.empathy > 0.68) return tier >= 3 ? '﹀﹀﹀' : '﹀﹀'
  if (personality.boldness > 0.68) return tier >= 3 ? '>═<' : '><'
  if (personality.curiosity > 0.68) return tier >= 3 ? '◡◇◡' : '◡◡'
  return tier === 0 ? '·' : tier >= 3 ? '───' : '~~'
}

function auraPair(personality: Personality, tier: Tier, mood: Mood): [string, string] {
  if (mood === 'thinking') return ['⌁', '⌁']
  if (mood === 'happy') return personality.boldness > 0.68 ? ['🔥', '🔥'] : ['✨', '✨']
  if (mood === 'evolve-a') return ['✶', '✶']
  if (mood === 'evolve-b') return ['⚡', '⚡']
  if (tier <= 1) return [' ', ' ']

  if (personality.boldness > 0.68) return ['⚡', '⚡']
  if (personality.empathy > 0.68) return ['☾', '☽']
  if (personality.curiosity > 0.68) return ['✦', '✦']
  return ['·', '·']
}

function sigilPair(dna: string, personality: Personality, mood: Mood): [string, string] {
  if (mood === 'thinking') return ['◌', '◍']
  if (mood === 'happy') return ['✦', '✦']
  if (mood === 'evolve-a' || mood === 'evolve-b') return ['☉', '☉']

  switch (dominantTrait(personality)) {
    case 'curiosity':
      return pick(dna, 'sigil-curiosity', [
        ['☾', '☽'],
        ['✦', '✦'],
        ['◐', '◑'],
      ])
    case 'boldness':
      return pick(dna, 'sigil-boldness', [
        ['⚡', '⚡'],
        ['✧', '✧'],
        ['▲', '▲'],
      ])
    case 'empathy':
      return pick(dna, 'sigil-empathy', [
        ['☽', '☾'],
        ['✿', '✿'],
        ['◌', '◌'],
      ])
    default:
      return pick(dna, 'sigil-balanced', [
        ['◇', '◇'],
        ['·', '·'],
        ['✧', '✧'],
      ])
  }
}

function baseGlyph(tier: Tier, mood: Mood): string {
  if (mood === 'inhale') return tier >= 3 ? '─┬─' : '──'
  if (mood === 'exhale') return tier >= 3 ? '───' : '~~'
  if (mood === 'thinking') return tier >= 3 ? '≋≋≋' : '··'
  if (mood === 'happy') return tier >= 3 ? '⌣⌣⌣' : '⌣⌣'
  if (mood === 'evolve-a') return '═∞═'
  if (mood === 'evolve-b') return '✶✶✶'
  return tier >= 3 ? '___' : '~~'
}

function withDrift(lines: string[], mood: Mood): string[] {
  const indent = mood === 'drift-left' ? ' ' : mood === 'drift-right' ? '   ' : '  '
  return lines.map((line) => `${indent}${line}`)
}

function buildTierLines(dna: string, tier: Tier, personality: Personality, mood: Mood): string[] {
  const template = getPetTemplate(dna, tier)
  const eyes = eyePair(dna, tier, mood)
  const mouth = mouthGlyph(personality, tier, mood)
  const [auraL, auraR] = auraPair(personality, tier, mood)
  const [sigilL, sigilR] = sigilPair(dna, personality, mood)
  const base = baseGlyph(tier, mood)

  const lines = template.lines.map((line) =>
    renderLine(line, {
      eyes,
      mouth,
      mask: 'Ọ̀ṣỌ́',
      auraL,
      auraR,
      sigilL,
      sigilR,
      base,
    }),
  )

  return withDrift(lines, mood)
}

/**
 * Generate a deterministic ASCII pet from DNA + tier + personality + mood.
 * Pure data transformation: DNA + Tier + Personality + Mood -> String
 */
export function generatePetAscii(
  dna: string,
  tier: Tier,
  personality: Personality,
  mood: Mood = 'idle',
): string {
  return buildTierLines(dna, tier, personality, mood).join('\n')
}

function escapeHtml(value: string): string {
  return value
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
}

export function renderPetAnsi(dna: string, tier: Tier, personality: Personality, mood: Mood = 'idle'): string {
  const palette = getPetPalette(tier, personality)
  const art = generatePetAscii(dna, tier, personality, mood)
  const softGlow = tier === 5 ? '\u001b[105;97;1m' : palette.ansi
  return `${softGlow}${art}\u001b[0m`
}

export function renderPetHtml(dna: string, tier: Tier, personality: Personality, mood: Mood = 'idle'): string {
  const palette = getPetPalette(tier, personality)
  const art = generatePetAscii(dna, tier, personality, mood)
  return `<span class="oso-ascii-html pet-tier-${tier}" style="--pet-fg:${palette.foreground};--pet-accent:${palette.accent};--pet-glow:${palette.glow}">${escapeHtml(art).replaceAll('\n', '<br />')}</span>`
}
