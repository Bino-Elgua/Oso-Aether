'use client'

import { useState, useRef, useEffect } from 'react'
import { useAetherStore } from '@/lib/state'
import { aether } from '@/lib/aether'
import PetDisplay from '@/components/pets/PetDisplay'
import MessageBubble from './MessageBubble'
import TypingIndicator from './TypingIndicator'

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
  const scrollRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    if (!pet) return
    aether.loadPetMemory(pet.id).then((history) => {
      if (history?.length) setMessages(history)
    }).catch(() => {})
  }, [pet?.id])

  useEffect(() => {
    scrollRef.current?.scrollTo({ top: scrollRef.current.scrollHeight, behavior: 'smooth' })
  }, [messages, isPetThinking])

  const handleSend = async () => {
    if (!input.trim() || !pet) return

    const userMessage: Message = {
      id: crypto.randomUUID(),
      role: 'user',
      content: input.trim(),
      timestamp: Date.now(),
    }

    setMessages((prev) => [...prev, userMessage])
    setInput('')
    setIsPetThinking(true)

    try {
      const response = await aether.processIntent({
        agentId: pet.id,
        intent: input.trim(),
        context: { tier: pet.tier, personality: pet.personality },
      })

      const petMessage: Message = {
        id: crypto.randomUUID(),
        role: 'pet',
        content: response.text,
        timestamp: Date.now(),
        metadata: response.growth
          ? { growth: true, evolution: response.newTier !== pet.tier }
          : undefined,
      }

      setMessages((prev) => [...prev, petMessage])

      if (response.growth) {
        updatePet(pet.id, {
          xp: response.newXp ?? pet.xp,
          tier: response.newTier ?? pet.tier,
          personality: response.personalityShift ?? pet.personality,
          asciiForm: response.newAsciiForm ?? pet.asciiForm,
        })
      }

      await aether.storeMemory(pet.id, [...messages, userMessage, petMessage]).catch(() => {})
    } catch {
      setMessages((prev) => [
        ...prev,
        {
          id: crypto.randomUUID(),
          role: 'pet',
          content: '*static flicker* ...connection to soul-fray interrupted...',
          timestamp: Date.now(),
        },
      ])
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
    <div className="flex h-full flex-col">
      <div className="border-b border-iron/30 p-4">
        <div className="flex items-center gap-4">
          <div className="w-24">
            <PetDisplay pet={pet} interactive />
          </div>
          <div>
            <h2 className="text-lg font-medium">{pet.name}</h2>
            <p className="text-sm text-muted">Tier {pet.tier} &middot; {Math.floor(pet.xp)} XP</p>
            <div className="mt-1 flex gap-2 text-[10px]">
              {pet.personality.curiosity > 0.6 && <span className="text-blue-400">&#10023; curious</span>}
              {pet.personality.boldness > 0.6 && <span className="text-amber-400">&#10023; bold</span>}
              {pet.personality.empathy > 0.6 && <span className="text-emerald-400">&#10023; gentle</span>}
            </div>
          </div>
        </div>
      </div>

      <div ref={scrollRef} className="flex-1 overflow-y-auto p-4 space-y-3">
        {messages.map((msg) => (
          <MessageBubble key={msg.id} message={msg} />
        ))}
        {isPetThinking && <TypingIndicator />}
      </div>

      <div className="border-t border-iron/30 p-4">
        <div className="flex gap-3">
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
            className="flex-1 resize-none rounded-lg bg-surface border border-iron/30 p-3 text-sm placeholder:text-muted/60"
            rows={2}
            disabled={isPetThinking}
          />
          <button
            onClick={handleSend}
            disabled={!input.trim() || isPetThinking}
            className="self-end rounded-lg bg-ember px-4 py-2 text-sm font-medium text-white disabled:opacity-50 hover:bg-ember/90 transition-colors"
          >
            {isPetThinking ? '&#10023;' : 'Send'}
          </button>
        </div>
        <p className="mt-2 text-[10px] text-muted/60">
          Your words shape their soul. Every exchange leaves a mark.
        </p>
      </div>
    </div>
  )
}
