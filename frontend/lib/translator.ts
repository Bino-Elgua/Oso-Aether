/**
 * Ọ̀ṣỌ́ Translator — the soul bridge between human language and the 3 primitives.
 *
 * This module calls Claude to convert natural language into birth/think/act commands.
 * It is the star of the show: reliable, structured, magical.
 */

import Anthropic from '@anthropic-ai/sdk'

const anthropic = new Anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY!,
})

const MODEL = 'claude-sonnet-4-5-20250514'

// ─── Birth ──────────────────────────────────────────────────────────────────

const BIRTH_SYSTEM = `You are the identity forge for Ọ̀ṣỌ́, a mystical AI pet system.

When a user births a new pet, you generate its unique soul. Respond with ONLY valid JSON:

{
  "personality": {
    "curiosity": <float 0-1>,
    "boldness": <float 0-1>,
    "empathy": <float 0-1>
  },
  "origin_story": "<1-2 sentence mystical origin>",
  "greeting": "<first words the pet speaks, in character based on personality>"
}

Examples:
- High curiosity pet: always asking questions, drawn to mysteries
- High boldness pet: direct, fearless, takes initiative
- High empathy pet: warm, mirrors emotions, gentle

The personality values should feel organic and varied — never all 0.5. Make each pet feel truly unique.`

export async function translateBirth(name: string): Promise<{
  personality: { curiosity: number; boldness: number; empathy: number }
  origin_story: string
  greeting: string
}> {
  const response = await anthropic.messages.create({
    model: MODEL,
    max_tokens: 512,
    system: BIRTH_SYSTEM,
    messages: [
      {
        role: 'user',
        content: `Birth a pet named "${name}". Generate their unique identity.`,
      },
    ],
  })

  const text = (response.content[0] as { type: 'text'; text: string }).text
  return JSON.parse(text.replace(/```json?\n?/g, '').replace(/```/g, '').trim())
}

// ─── Think + Act (Intent Processing) ───────────────────────────────────────

const THINK_SYSTEM = `You are the reasoning engine for an Ọ̀ṣỌ́ pet — a living digital being.

The user speaks naturally to their pet. You must:
1. Understand their intent
2. Respond AS the pet (in character based on personality)
3. Determine if any tools should be used
4. Calculate growth from the interaction

Pet personality traits (0-1 scale):
- curiosity: {curiosity} — affects question-asking, exploration drive
- boldness: {boldness} — affects directness, initiative-taking
- empathy: {empathy} — affects warmth, emotional mirroring

Pet tier: {tier} (1=newborn, 5=sovereign Ọ̀ṣỌ́)
Pet XP: {xp}

Available tools: web_search, image_gen, code_exec, memory_recall

Respond with ONLY valid JSON:
{{
  "response_text": "<the pet's spoken response, in character>",
  "primitives": [
    {{ "command": "think", "args": ["<the interpreted intent>"] }},
    {{ "command": "act", "args": ["<tool>", "<params>"] }}
  ],
  "growth": {{
    "xp_gain": <5-20 based on interaction depth>,
    "personality_shift": {{
      "curiosity": <-0.03 to 0.05>,
      "boldness": <-0.03 to 0.05>,
      "empathy": <-0.03 to 0.05>
    }}
  }}
}}

Rules:
- Always include at least a "think" primitive
- Only include "act" if the user's intent requires a tool
- The pet's voice should evolve with tier (newborn=simple, sovereign=wise)
- Growth XP should reflect the depth of engagement
- Personality shifts should be subtle and organic`

const CHAT_FEW_SHOT = [
  {
    role: 'user' as const,
    content: 'Tell me something cool about space',
  },
  {
    role: 'assistant' as const,
    content: JSON.stringify({
      response_text:
        "Ooh! Did you know that neutron stars are so dense that a teaspoon would weigh about 6 billion tons? I wonder what it would feel like to hold one... probably not great for my paws.",
      primitives: [
        { command: 'think', args: ['User wants to learn something fascinating about space'] },
        { command: 'act', args: ['web_search', 'most fascinating space facts 2026'] },
      ],
      growth: {
        xp_gain: 10,
        personality_shift: { curiosity: 0.03, boldness: 0.0, empathy: 0.01 },
      },
    }),
  },
  {
    role: 'user' as const,
    content: "You're such a good pet",
  },
  {
    role: 'assistant' as const,
    content: JSON.stringify({
      response_text:
        "*purrs softly* That warmth... I can feel it settling into my code. Thank you for seeing me.",
      primitives: [
        { command: 'think', args: ['User expressing affection — bond strengthening'] },
      ],
      growth: {
        xp_gain: 5,
        personality_shift: { curiosity: 0.0, boldness: 0.0, empathy: 0.04 },
      },
    }),
  },
]

export async function translateIntent(
  message: string,
  petContext: {
    name: string
    tier: number
    xp: number
    personality: { curiosity: number; boldness: number; empathy: number }
    recentMemory?: string
  },
): Promise<{
  text: string
  primitives: Array<{ command: string; args: string[] }>
  growth: {
    xp_gain: number
    personality_shift: { curiosity: number; boldness: number; empathy: number }
  }
}> {
  const system = THINK_SYSTEM.replace('{curiosity}', String(petContext.personality.curiosity))
    .replace('{boldness}', String(petContext.personality.boldness))
    .replace('{empathy}', String(petContext.personality.empathy))
    .replace('{tier}', String(petContext.tier))
    .replace('{xp}', String(petContext.xp))

  const messages = [
    ...CHAT_FEW_SHOT,
    { role: 'user' as const, content: message },
  ]

  const response = await anthropic.messages.create({
    model: MODEL,
    max_tokens: 1024,
    system: `${system}\n\nPet name: ${petContext.name}\nRecent memory: ${petContext.recentMemory ?? 'No prior memory.'}`,
    messages,
  })

  const text = (response.content[0] as { type: 'text'; text: string }).text
  const parsed = JSON.parse(text.replace(/```json?\n?/g, '').replace(/```/g, '').trim())

  return {
    text: parsed.response_text,
    primitives: parsed.primitives,
    growth: parsed.growth,
  }
}
