'use client'

import { useAetherStore } from '@/lib/state'
import Link from 'next/link'
import ChatInterface from '@/components/chat/ChatInterface'
import PetDisplay from '@/components/pets/PetDisplay'

export default function Dashboard() {
  const { pets, activePetId } = useAetherStore()
  const activePet = pets.find((p) => p.id === activePetId)

  return (
    <div className="min-h-[calc(100vh-73px)] px-4 py-6">
      {activePet ? (
        <ChatInterface />
      ) : (
        <div className="grid gap-6 py-8 lg:grid-cols-[minmax(0,1.2fr)_320px]">
          <section className="dashboard-panel flex flex-col justify-between gap-8 px-8 py-10">
            <div>
              <p className="dashboard-kicker">Ọ̀ṣỌ́ Aether</p>
              <h1 className="mt-4 max-w-xl text-5xl font-semibold leading-none tracking-[0.06em] text-[var(--text-primary)]">
                Build a living mask, then raise it through communion.
              </h1>
              <p className="mt-5 max-w-2xl text-base leading-7 text-muted">
                The dashboard centers the ASCII pet, its reputation, and the tools it has earned. Birth a companion to start shaping the renderer with every exchange.
              </p>
            </div>

            <div className="flex flex-wrap gap-3">
              <Link
                href="/birth"
                className="rounded-2xl bg-gradient-to-r from-[#ff8a3d] to-[#f4d16f] px-6 py-3 text-sm font-semibold text-[#1b120c]"
              >
                Birth Your First Pet
              </Link>
              <Link
                href="/pets"
                className="rounded-2xl border border-white/10 px-6 py-3 text-sm font-medium text-[var(--text-primary)] transition-colors hover:bg-white/5"
              >
                Open Gallery
              </Link>
            </div>
          </section>

          <section className="dashboard-panel flex flex-col items-center justify-center gap-5 px-6 py-8 text-center">
            <div className="ascii-container ascii-container-hero w-full max-w-[280px]">
              <pre className="ascii-stage text-sm leading-[14px] text-[var(--accent-cyan)]">
{`   ✦╭═════╮✦
   ╭╣  △ ✧  ╠╮
  ╭╯   Ọ̀ṣỌ́   ╰╮
 ╱   ☉ ◉  ◉ ☉   ╲
 │    ╲───╱    │
 ╰╮  ╲_∞_╱  ╭╯
   ╰═══╧═══╯`}
              </pre>
            </div>
            <div>
              <h2 className="text-lg font-medium text-[var(--text-primary)]">The mask waits for a name.</h2>
              <p className="mt-2 text-sm text-muted">Birth unlocks the live renderer, reputation growth, tool ring, and the full communion dashboard.</p>
            </div>
          </section>
        </div>
      )}

      {pets.length > 0 && (
        <div className="mt-6">
          <h2 className="mb-4 text-sm uppercase tracking-[0.22em] text-muted">Your Companions ({pets.length})</h2>
          <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
            {pets.map((pet) => (
              <Link
                key={pet.id}
                href={`/chat/${pet.id}`}
                className="dashboard-panel transition-transform hover:-translate-y-1"
              >
                <PetDisplay pet={pet} interactive showMeta={false} />
                <div className="mt-4 flex items-center justify-between text-sm">
                  <span className="font-medium text-[var(--text-primary)]">{pet.name}</span>
                  <span className="text-xs text-muted">Tier {pet.tier} &middot; {pet.xp} rep</span>
                </div>
              </Link>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}
