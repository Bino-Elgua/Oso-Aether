'use client'

import { useAetherStore } from '@/lib/state'
import Link from 'next/link'

export default function PetsGallery() {
  const { pets, setActivePet } = useAetherStore()

  return (
    <div className="px-6 py-12">
      <div className="mb-8 flex items-center justify-between">
        <h1 className="text-2xl font-medium">My Pets</h1>
        <Link
          href="/birth"
          className="rounded-lg bg-soul px-4 py-2 text-sm text-white hover:bg-soul/90 transition-colors"
        >
          + Birth New
        </Link>
      </div>

      {pets.length === 0 ? (
        <div className="flex flex-col items-center py-20 text-center">
          <p className="text-muted mb-4">The gallery is empty.</p>
          <Link
            href="/birth"
            className="text-ember hover:underline text-sm"
          >
            Birth your first companion
          </Link>
        </div>
      ) : (
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {pets.map((pet) => (
            <Link
              key={pet.id}
              href={`/chat/${pet.id}`}
              onClick={() => setActivePet(pet.id)}
              className="group rounded-lg border border-iron/20 bg-surface p-4 hover:border-soul/40 transition-colors"
            >
              <div className="ascii-container mb-3">
                <pre className="font-mono text-xs leading-[10px] text-purple-400 text-center">
                  {pet.asciiForm}
                </pre>
              </div>
              <h3 className="font-medium">{pet.name}</h3>
              <div className="mt-1 flex items-center justify-between text-xs text-muted">
                <span>Tier {pet.tier}</span>
                <span>{pet.xp} XP</span>
              </div>
              <div className="mt-2 h-1 w-full rounded-full bg-elevated overflow-hidden">
                <div
                  className="h-full rounded-full bg-ember transition-all"
                  style={{ width: `${Math.min(100, (pet.xp / [100, 500, 2000, 10000, 10000][pet.tier - 1]) * 100)}%` }}
                />
              </div>
            </Link>
          ))}
        </div>
      )}
    </div>
  )
}
