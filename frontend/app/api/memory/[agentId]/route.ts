import { NextRequest, NextResponse } from 'next/server'
import { walrus } from '@/lib/walrus'

export async function GET(
  _request: NextRequest,
  { params }: { params: Promise<{ agentId: string }> },
) {
  try {
    const { agentId } = await params

    const memory = await walrus.load(agentId)

    if (!memory) {
      return NextResponse.json([])
    }

    return NextResponse.json(memory.messages ?? [])
  } catch (error) {
    console.error('Memory load failed:', error)
    return NextResponse.json([])
  }
}

export async function POST(
  request: NextRequest,
  { params }: { params: Promise<{ agentId: string }> },
) {
  try {
    const { agentId } = await params
    const { messages } = await request.json()

    // Load existing memory, merge, store back
    const existing = await walrus.load(agentId)
    const updated = {
      ...existing,
      agentId,
      messages,
      updatedAt: Date.now(),
    }

    const cid = await walrus.store(agentId, updated)

    return NextResponse.json({ success: true, cid })
  } catch (error) {
    console.error('Memory store failed:', error)
    return NextResponse.json(
      { error: 'Memory could not be inscribed.' },
      { status: 500 },
    )
  }
}
