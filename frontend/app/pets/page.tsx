'use client'

import { useAetherStore } from '@/lib/state'
import Link from 'next/link'
import PetDisplay from '@/components/pets/PetDisplay'
import { progressToNextTier } from '@engine/growth'

export default function PetsGallery() {
  const { pets, setActivePet } = useAetherStore()

  return (
    <div className="px-4 py-8">
      <div className="mb-8 flex items-center justify-between">
        <div>
          <p className="dashboard-kicker">Gallery</p>
          <h1 className="mt-2 text-3xl font-medium text-[var(--text-primary)]">My Pets</h1>
        </div>
        <Link
          href="/birth"
          className="rounded-2xl bg-gradient-to-r from-[#ff8a3d] to-[#f4d16f] px-4 py-2 text-sm font-semibold text-[#1b120c]"
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
          {pets.map((pet) => {
            const progress = progressToNextTier(pet.xp)
            const width = pet.tier === 5 ? 100 : Math.max(8, Math.round(progress.progress * 100))

            return (
              <Link
                key={pet.id}
                href={`/chat/${pet.id}`}
                onClick={() => setActivePet(pet.id)}
                className="dashboard-panel group transition-transform hover:-translate-y-1"
              >
                <PetDisplay pet={pet} interactive showMeta={false} />
                <h3 className="mt-4 font-medium text-[var(--text-primary)]">{pet.name}</h3>
                <div className="mt-1 flex items-center justify-between text-xs text-muted">
                  <span>Tier {pet.tier}</span>
                  <span>{pet.xp} rep</span>
                </div>
                <div className="mt-3 h-1 w-full overflow-hidden rounded-full bg-white/5">
                  <div
                    className="h-full rounded-full bg-gradient-to-r from-[#ff8a3d] to-[#77dfff] transition-all"
                    style={{ width: `${width}%` }}
                  />
                </div>
              </Link>
            )
          })}
        </div>
      )}
    </div>
  )
}
