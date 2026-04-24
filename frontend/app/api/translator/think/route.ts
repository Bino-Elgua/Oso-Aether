import { NextRequest, NextResponse } from 'next/server'
import { translateIntent } from '@/lib/translator'
import { generatePetAscii } from '@engine/ascii-renderer/generator'
import { tierFromXP, shiftPersonality } from '@engine/growth'
import type { Tier } from '@engine/ascii-renderer/generator'

export async function POST(request: NextRequest) {
  try {
    const { agentId, intent, context } = await request.json()

    if (!intent || !agentId) {
      return NextResponse.json(
        { error: 'Missing agentId or intent' },
        { status: 400 },
      )
    }

    const petContext = {
      name: context?.name ?? 'Unknown',
      tier: context?.tier ?? 1,
      xp: context?.xp ?? 0,
      personality: context?.personality ?? { curiosity: 0.5, boldness: 0.5, empathy: 0.5 },
      recentMemory: context?.recentMemory,
    }

    // Translate natural language → primitives + pet response
    const result = await translateIntent(intent, petContext)

    // Calculate new state
    const newXp = petContext.xp + result.growth.xp_gain
    const oldTier = petContext.tier
    const newTier = tierFromXP(newXp)
    const evolved = newTier !== oldTier

    // Apply personality shift
    const newPersonality = shiftPersonality(petContext.personality, result.growth.personality_shift)

    // Regenerate ASCII if tier changed
    let newAsciiForm: string | undefined
    if (evolved) {
      newAsciiForm = generatePetAscii(
        context?.dna ?? '',
        newTier as Tier,
        newPersonality,
      )
    }

    return NextResponse.json({
      text: result.text,
      primitives: result.primitives,
      growth: true,
      newXp,
      newTier,
      evolved,
      personalityShift: newPersonality,
      newAsciiForm,
    })
  } catch (error) {
    console.error('Think failed:', error)
    return NextResponse.json(
      { error: '*static flicker* ...the soul-fray trembles...' },
      { status: 500 },
    )
  }
}
