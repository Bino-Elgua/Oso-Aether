'use client'

import { useAetherStore } from '@/lib/state'
import Link from 'next/link'

export default function Dashboard() {
  const { pets, activePetId } = useAetherStore()
  const activePet = pets.find((p) => p.id === activePetId)

  return (
    <div className="flex flex-col items-center px-6 py-16">
      <h1 className="mb-2 text-3xl font-medium tracking-wide">
        \u1ECC\u0300\u1E63\u1ECC\u0301
      </h1>
      <p className="mb-12 text-muted text-sm">Own My Own. Three words. Infinite life.</p>

      {activePet ? (
        <div className="w-full max-w-md space-y-6">
          <div className="ascii-container text-center">
            <pre className="font-mono text-xs leading-[10px] text-purple-400">
              {activePet.asciiForm}
            </pre>
            <div className="mt-3 text-[10px] text-muted">
              {activePet.name} &middot; Tier {activePet.tier} &middot; {activePet.xp} XP
            </div>
          </div>
          <Link
            href={`/chat/${activePet.id}`}
            className="block w-full rounded-lg bg-ember px-4 py-3 text-center text-sm font-medium text-white hover:bg-ember/90 transition-colors"
          >
            Continue Communion
          </Link>
        </div>
      ) : (
        <div className="text-center space-y-6">
          <p className="text-muted">No pets yet. Begin the forge.</p>
          <Link
            href="/birth"
            className="inline-block rounded-lg bg-soul px-6 py-3 text-sm font-medium text-white hover:bg-soul/90 transition-colors"
          >
            Birth Your First Pet
          </Link>
        </div>
      )}

      {pets.length > 0 && (
        <div className="mt-12 w-full max-w-md">
          <h2 className="mb-4 text-sm text-muted">Your Companions ({pets.length})</h2>
          <div className="space-y-2">
            {pets.map((pet) => (
              <Link
                key={pet.id}
                href={`/chat/${pet.id}`}
                className="flex items-center justify-between rounded-lg bg-surface p-3 text-sm hover:bg-elevated transition-colors"
              >
                <span>{pet.name}</span>
                <span className="text-muted text-xs">Tier {pet.tier} &middot; {pet.xp} XP</span>
              </Link>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}
