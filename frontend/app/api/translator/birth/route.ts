import { NextRequest, NextResponse } from 'next/server'
import { translateBirth } from '@/lib/translator'
import { generateDNA, personalityFromDNA } from '@engine/86-dna'
import { generatePetAscii } from '@engine/ascii-renderer/generator'
import { walrus } from '@/lib/walrus'

export async function POST(request: NextRequest) {
  try {
    const { name } = await request.json()

    if (!name || typeof name !== 'string' || name.length > 24) {
      return NextResponse.json(
        { error: 'Name required (max 24 chars)' },
        { status: 400 },
      )
    }

    // Generate unique 86-DNA fingerprint
    const entropy = crypto.randomUUID()
    const dna = generateDNA(name, entropy)

    // Ask Claude to forge the pet's soul
    const identity = await translateBirth(name)

    // Merge LLM personality with DNA-seeded personality for uniqueness
    const dnaSeedPersonality = personalityFromDNA(dna)
    const personality = {
      curiosity: (identity.personality.curiosity + dnaSeedPersonality.curiosity) / 2,
      boldness: (identity.personality.boldness + dnaSeedPersonality.boldness) / 2,
      empathy: (identity.personality.empathy + dnaSeedPersonality.empathy) / 2,
    }

    // Generate initial ASCII form (tier 1 newborn)
    const asciiPreview = generatePetAscii(dna, 1, personality)

    // Create agent ID
    const agentId = `pet_${name.toLowerCase().replace(/\s+/g, '-')}_${entropy.slice(0, 8)}`

    // Store initial memory to Walrus
    const initialMemory = {
      agentId,
      name,
      dna,
      personality,
      origin_story: identity.origin_story,
      messages: [
        {
          id: crypto.randomUUID(),
          role: 'pet',
          content: identity.greeting,
          timestamp: Date.now(),
        },
      ],
      createdAt: Date.now(),
    }

    const walrusCid = await walrus.store(agentId, initialMemory)

    return NextResponse.json({
      agentId,
      dna,
      personality,
      asciiPreview,
      greeting: identity.greeting,
      originStory: identity.origin_story,
      walrusCid,
      suiTxHash: '', // Populated after Sui mint — handled client-side
    })
  } catch (error) {
    console.error('Birth failed:', error)
    return NextResponse.json(
      { error: 'The forge faltered. Soul could not be formed.' },
      { status: 500 },
    )
  }
}
