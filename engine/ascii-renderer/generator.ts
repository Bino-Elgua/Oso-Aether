export type Personality = { curiosity: number; boldness: number; empathy: number }
export type Tier = 1 | 2 | 3 | 4 | 5
export type Mood = 'neutral' | 'happy' | 'focused' | 'resting'

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

const moodMap: Record<Mood, { mouth: string; aura: string }> = {
  neutral: { mouth: '> ^ <', aura: '' },
  happy: { mouth: '~ \u25e1 ~', aura: '\u2728' },
  focused: { mouth: '> \u2012 <', aura: '\u26a1' },
  resting: { mouth: '\u23f8 \u23f8', aura: '\ud83d\udca4' },
}

/**
 * Generate a deterministic ASCII pet from DNA + tier + personality + mood.
 * Pure data transformation: DNA + Tier + Personality + Mood -> String
 */
export function generatePetAscii(
  dna: string,
  tier: Tier,
  personality: Personality,
  mood: Mood = 'neutral',
): string {
  const { mouth: baseMouth, aura: moodAura } = moodMap[mood]

  // Face carries the \u1ECC\u0300\u1E63\u1ECC\u0301 signature at tier 5
  const face =
    tier === 5
      ? '( \u1ECC\u0300\u1E63\u1ECC\u0301 )'
      : pick(dna, 'face', [
          '( O m O )',
          '(O m O)',
          '[ O m O ]',
          '{ O m O }',
          '\u27e8 O m O \u27e9',
        ])

  // Ears scale with tier
  const ears = pick(
    dna,
    'ears',
    tier <= 2
      ? ['/\\_/\\  ', '~ ~ ~  ', '(\u25d5\u25d5)   ', '\u256d\u2500\u2500\u2500\u256e  ']
      : ['/===/\\\\ ', '\u256d\u2501\u2726\u2501\u256e ', '/\u2571\u2572\u2572/\\\\ ', '\u263e   \u263d '],
  )

  // Mouth adapts to mood + personality
  let mouth = baseMouth
  if (personality.boldness > 0.7 && tier >= 3) mouth = '>---<'
  if (personality.empathy > 0.7) mouth = mouth.replace('>', '~').replace('<', '~')

  // Aura particles driven by personality
  const leftAura = personality.curiosity > 0.6 ? '\u2728 ' : personality.empathy > 0.6 ? '\u263e ' : '  '
  const rightAura = personality.curiosity > 0.6 ? ' \u2728' : personality.empathy > 0.6 ? ' \u263d' : '  '
  const boldAura = personality.boldness > 0.7 && tier >= 3 ? '\u26a1 ' : '  '
  const boldAuraRight = personality.boldness > 0.7 && tier >= 3 ? ' \u26a1' : '  '

  switch (tier) {
    case 1:
      return `  ${ears}\n ${face}\n  ${mouth}`
    case 2:
      return `   ${ears}\n  ${face}\n   ${mouth}\n   ~~~`
    case 3:
      return `${leftAura}${ears}${rightAura}\n ${boldAura}${face}${boldAuraRight}\n  ${mouth}\n  ${leftAura} ${rightAura}`
    case 4:
      return `${leftAura}\u256d\u2500\u2500\u2500\u256e${rightAura}\n ${boldAura}${face}${boldAuraRight}\n\u2570${mouth}\u256f\n  ${moodAura || leftAura}\u25c6 ${rightAura}`
    case 5:
      return `${leftAura}\u256d\u2501\u2501\u2501\u2501\u2501\u256e${rightAura}\n${boldAura}/ \u1ECC\u0300\u1E63\u1ECC\u0301 \\${boldAuraRight}\n(  ${mouth}  )\n \u2570\u2501\u2501\u2501\u2501\u2501\u256f\n    ${moodAura || '\u2728'} \u221e ${moodAura || '\u2728'}`
  }
}
