'use client'

import { useEffect, useRef, useState } from 'react'
import { useAetherStore } from '@/lib/state'
import { aether } from '@/lib/aether'
import type { AgentState } from '@/lib/aether'
import PetDisplay from '@/components/pets/PetDisplay'
import MessageBubble from './MessageBubble'
import TypingIndicator from './TypingIndicator'
import { TOOL_UNLOCKS, TIER_TITLES, getUnlockedTools, progressToNextTier } from '@engine/growth'
import type { AnimationMode } from '@engine/ascii-renderer'
import type { Tier } from '@engine/ascii-renderer'

interface Message {
  id: string
  role: 'user' | 'pet'
  content: string
  timestamp: number
  metadata?: { growth?: boolean; evolution?: boolean }
}

export default function ChatInterface() {
  const { getActivePet, updatePet } = useAetherStore()
  const pet = getActivePet()

  const [messages, setMessages] = useState<Message[]>([])
  const [input, setInput] = useState('')
  const [isPetThinking, setIsPetThinking] = useState(false)
  const [petActivity, setPetActivity] = useState<AnimationMode>('idle')
  const [agentState, setAgentState] = useState<AgentState | null>(null)
  const scrollRef = useRef<HTMLDivElement>(null)

  // Initialize agent state from pet data
  useEffect(() => {
    if (!pet) return
    setPetActivity('idle')

    // Create agent state from stored pet data
    aether.createAgent(pet.id, pet.name, pet.dna).then((agent) => {
      // Restore reputation by replaying thought count from stored xp
      // The agent starts fresh each session — reputation is rebuilt
      // from the stored pet.xp value on the next interaction
      setAgentState(agent)
    })

    aether.loadMemory(pet.id).then((history) => {
      if (history?.length) setMessages(history)
    }).catch(() => {})
  }, [pet?.id])

  useEffect(() => {
    scrollRef.current?.scrollTo({ top: scrollRef.current.scrollHeight, behavior: 'smooth' })
  }, [messages, isPetThinking])

  useEffect(() => {
    if (petActivity === 'idle') return

    const timeout = setTimeout(() => {
      setPetActivity('idle')
    }, petActivity === 'evolving' ? 2200 : 1200)

    return () => clearTimeout(timeout)
  }, [petActivity])

  const tier = (pet?.tier ?? 0) as Tier
  const unlockedTools = getUnlockedTools(tier)
  const progress = pet ? progressToNextTier(pet.xp) : null

  const handleSend = async () => {
    if (!input.trim() || !pet || !agentState) return

    const intent = input.trim()
    const userMessage: Message = {
      id: crypto.randomUUID(),
      role: 'user',
      content: intent,
      timestamp: Date.now(),
    }

    setMessages((prev) => [...prev, userMessage])
    setInput('')
    setIsPetThinking(true)
    setPetActivity('thinking')

    try {
      // Process through Rust WASM: translate → execute → respond
      const result = await aether.process(intent, agentState, undefined)

      if (!result) {
        setIsPetThinking(false)
        setPetActivity('idle')
        return
      }

      // Update local agent state with the mutated version from Rust
      setAgentState(result.agent)

      const evolved = result.response.evolved
      const newRep = Number(result.agent.reputation)

      const petMessage: Message = {
        id: crypto.randomUUID(),
        role: 'pet',
        content: result.response.message,
        timestamp: Date.now(),
        metadata: result.response.reputation_gained > 0
          ? { growth: true, evolution: evolved }
          : undefined,
      }

      setMessages((prev) => [...prev, petMessage])

      // Sync pet store with new state from Rust
      const newTier = typeof result.agent.tier === 'object'
        ? (result.agent.tier as { Awakened: number }).Awakened
        : 0
      const oldTier = pet.tier

      updatePet(pet.id, {
        xp: newRep,
        tier: newTier,
        personality: result.agent.personality,
      })

      const reachedSovereign = oldTier < 5 && newTier === 5
      setPetActivity(reachedSovereign ? 'evolving' : evolved ? 'evolving' : result.response.reputation_gained > 0 ? 'happy' : 'idle')

      const nextMessages = [...messages, userMessage, petMessage]
      await aether.storeMemory(pet.id, nextMessages).catch(() => {})
    } catch {
      setMessages((prev) => [
        ...prev,
        {
          id: crypto.randomUUID(),
          role: 'pet',
          content: "Something went wrong. I'm still here though — try again.",
          timestamp: Date.now(),
        },
      ])
      setPetActivity('idle')
    } finally {
      setIsPetThinking(false)
    }
  }

  if (!pet) {
    return (
      <div className="flex h-full items-center justify-center text-muted">
        Select or birth a pet to begin communion
      </div>
    )
  }

  return (
    <div className="grid h-full min-h-0 gap-4 xl:grid-cols-[250px_minmax(0,1fr)_260px]">
      <aside className="dashboard-panel order-2 xl:order-1">
        <div>
          <p className="dashboard-kicker">Reputation</p>
          <div className="mt-3 flex items-end justify-between gap-4">
            <div>
              <div className="text-4xl font-semibold text-[var(--text-primary)]">{Math.floor(pet.xp)}</div>
              <p className="mt-1 text-sm text-muted">Tier {pet.tier} — {TIER_TITLES[tier]}</p>
            </div>
            <div className="rounded-full border border-white/10 px-3 py-1 text-[10px] uppercase tracking-[0.24em] text-[var(--accent-cyan)]">
              Tier {pet.tier}
            </div>
          </div>
        </div>

        {progress && (
          <div className="mt-8">
            <div className="flex items-center justify-between text-[10px] uppercase tracking-[0.22em] text-muted">
              <span>toward next form</span>
              <span>{pet.tier === 5 ? 'Sovereign' : `${Math.round(progress.progress * 100)}%`}</span>
            </div>
            <div className="mt-3 h-2 overflow-hidden rounded-full bg-white/5">
              <div
                className="h-full rounded-full bg-gradient-to-r from-[#ff8a3d] via-[#f6d47a] to-[#77dfff] transition-all duration-500"
                style={{ width: `${pet.tier === 5 ? 100 : Math.max(8, Math.round(progress.progress * 100))}%` }}
              />
            </div>
            <p className="mt-2 text-xs text-muted">
              {pet.tier === 5 ? 'The sacred mask has awakened and now remains.' : `${progress.nextThreshold - pet.xp} reputation until the next unveiling.`}
            </p>
          </div>
        )}

        <div className="mt-8 space-y-3">
          <p className="dashboard-kicker">Temperament</p>
          {[
            ['Curiosity', pet.personality.curiosity],
            ['Boldness', pet.personality.boldness],
            ['Empathy', pet.personality.empathy],
          ].map(([label, value]) => (
            <div key={label}>
              <div className="mb-2 flex items-center justify-between text-xs text-muted">
                <span>{label}</span>
                <span>{Math.round(Number(value) * 100)}</span>
              </div>
              <div className="h-2 overflow-hidden rounded-full bg-white/5">
                <div
                  className="h-full rounded-full bg-[linear-gradient(90deg,#ff8a3d_0%,#77dfff_100%)] transition-all duration-500"
                  style={{ width: `${Math.max(8, Math.round(Number(value) * 100))}%` }}
                />
              </div>
            </div>
          ))}
        </div>
      </aside>

      <section className="order-1 flex min-h-0 flex-col gap-4 xl:order-2">
        <div className="dashboard-panel dashboard-sanctum">
          <p className="dashboard-kicker">Live Mask</p>
          <div className="mt-3 flex flex-col items-center gap-4 text-center">
            <PetDisplay
              pet={pet}
              interactive
              isEvolving={petActivity === 'evolving'}
              animationState={petActivity}
              variant="hero"
              showMeta={false}
            />
            <div>
              <h2 className="text-2xl font-semibold tracking-[0.08em] text-[var(--text-primary)]">{pet.name}</h2>
              <p className="mt-1 text-sm text-muted">Tier {pet.tier} — {TIER_TITLES[tier]} &middot; a living Ọ̀ṣỌ́ form shaped by conversation.</p>
            </div>
          </div>
        </div>

        <div ref={scrollRef} className="dashboard-panel min-h-[220px] flex-1 space-y-3 overflow-y-auto">
          <div className="mb-2 flex items-center justify-between">
            <p className="dashboard-kicker">Communion Log</p>
            <span className="text-[10px] uppercase tracking-[0.22em] text-muted">{messages.length} entries</span>
          </div>
          {messages.map((msg) => (
            <MessageBubble key={msg.id} message={msg} />
          ))}
          {isPetThinking && <TypingIndicator />}
          {messages.length === 0 && !isPetThinking && (
            <div className="rounded-2xl border border-dashed border-white/10 p-4 text-sm text-muted">
              Speak first. The first exchange sets the tone of the mask.
            </div>
          )}
        </div>

        <div className="dashboard-panel">
          <div className="flex gap-3 max-md:flex-col">
            <textarea
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' && !e.shiftKey) {
                  e.preventDefault()
                  handleSend()
                }
              }}
              placeholder="Speak to your pet..."
              className="min-h-[86px] flex-1 resize-none rounded-2xl border border-white/10 bg-black/20 px-4 py-3 text-sm placeholder:text-muted/60"
              rows={3}
              disabled={isPetThinking}
            />
            <button
              onClick={handleSend}
              disabled={!input.trim() || isPetThinking}
              className="rounded-2xl bg-gradient-to-r from-[#ff8a3d] to-[#f4d16f] px-5 py-3 text-sm font-semibold text-[#1b120c] transition-opacity hover:opacity-90 disabled:opacity-50"
            >
              {isPetThinking ? 'Listening...' : 'Send'}
            </button>
          </div>
          <p className="mt-3 text-[11px] text-muted/70">
            Your words raise reputation, unlock tools, and change how the pet breathes, thinks, and celebrates back at you.
          </p>
        </div>
      </section>

      <aside className="dashboard-panel order-3">
        <div className="flex items-center justify-between">
          <p className="dashboard-kicker">Tool Ring</p>
          <span className="text-[10px] uppercase tracking-[0.22em] text-muted">{unlockedTools.length}/{TOOL_UNLOCKS.length} open</span>
        </div>

        <div className="mt-5 space-y-3">
          {TOOL_UNLOCKS.map((tool) => {
            const unlocked = tool.tier <= tier
            return (
              <div
                key={tool.name}
                className={`rounded-2xl border p-4 transition-colors ${
                  unlocked
                    ? 'border-[rgba(119,223,255,0.28)] bg-[rgba(119,223,255,0.08)]'
                    : 'border-white/8 bg-black/10 opacity-55'
                }`}
              >
                <div className="flex items-center justify-between gap-3">
                  <div>
                    <h3 className="text-sm font-medium text-[var(--text-primary)]">{tool.label}</h3>
                    <p className="mt-1 text-xs text-muted">{tool.description}</p>
                  </div>
                  <span className="rounded-full border border-white/10 px-2 py-1 text-[10px] uppercase tracking-[0.2em] text-muted">
                    T{tool.tier}
                  </span>
                </div>
              </div>
            )
          })}
        </div>
      </aside>
    </div>
  )
}
