'use client'

import { useEffect } from 'react'
import { useParams } from 'next/navigation'
import { useAetherStore } from '@/lib/state'
import ChatInterface from '@/components/chat/ChatInterface'

export default function ChatPage() {
  const { petId } = useParams<{ petId: string }>()
  const { pets, setActivePet } = useAetherStore()

  useEffect(() => {
    if (petId) setActivePet(petId)
  }, [petId, setActivePet])

  const pet = pets.find((p) => p.id === petId)

  if (!pet) {
    return (
      <div className="flex h-[80vh] items-center justify-center text-muted">
        Pet not found. It may have wandered off.
      </div>
    )
  }

  return (
    <div className="h-[calc(100vh-65px)]">
      <ChatInterface />
    </div>
  )
}
