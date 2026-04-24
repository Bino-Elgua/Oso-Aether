'use client'

import { useState } from 'react'
import { useRouter } from 'next/navigation'
import { motion } from 'framer-motion'
import { useAetherStore } from '@/lib/state'
import { aether } from '@/lib/aether'

export default function BirthPage() {
  const [name, setName] = useState('')
  const [isBirthing, setIsBirthing] = useState(false)
  const [birthResult, setBirthResult] = useState<{ id: string; dna: string } | null>(null)
  const { addPet, setActivePet } = useAetherStore()
  const router = useRouter()

  const handleBirth = async () => {
    if (!name.trim()) return
    setIsBirthing(true)

    try {
      const result = await aether.birthPet(name.trim())

      const newPet = {
        id: result.agentId,
        name: name.trim(),
        dna: result.dna,
        asciiForm: result.asciiPreview,
        tier: 1 as const,
        xp: 0,
        personality: {
          curiosity: Math.random(),
          boldness: Math.random(),
          empathy: Math.random(),
        },
        memoryRoot: result.walrusCid,
        createdAt: Date.now(),
      }

      addPet(newPet)
      setBirthResult({ id: result.agentId, dna: result.dna })

      setTimeout(() => {
        setActivePet(newPet.id)
        router.push(`/chat/${newPet.id}`)
      }, 2000)
    } catch (err) {
      console.error('Birth failed:', err)
    } finally {
      setIsBirthing(false)
    }
  }

  return (
    <div className="flex min-h-[80vh] flex-col items-center justify-center px-6">
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="w-full max-w-md"
      >
        <h1 className="mb-2 text-2xl font-medium text-center">Birth a New Pet</h1>
        <p className="mb-8 text-center text-muted text-sm">
          Give them a name. Their soul will forge the rest.
        </p>

        {!birthResult ? (
          <div className="space-y-4">
            <div>
              <label className="mb-1 block text-xs text-muted">Pet Name</label>
              <input
                value={name}
                onChange={(e) => setName(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleBirth()}
                placeholder="ember, void-whisper, iron-kin..."
                maxLength={24}
                disabled={isBirthing}
                className="w-full rounded-lg bg-surface border border-iron/30 px-4 py-3 text-sm placeholder:text-muted/60"
              />
            </div>

            <button
              onClick={handleBirth}
              disabled={!name.trim() || isBirthing}
              className="w-full rounded-lg bg-ember px-4 py-3 text-sm font-medium text-white disabled:opacity-50 hover:bg-ember/90 transition-colors"
            >
              {isBirthing ? (
                <span className="flex items-center justify-center gap-2">
                  <motion.span
                    animate={{ rotate: 360 }}
                    transition={{ duration: 1, repeat: Infinity, ease: 'linear' }}
                  >
                    &#10022;
                  </motion.span>
                  Forging soul...
                </span>
              ) : (
                'Birth Pet'
              )}
            </button>

            <p className="text-[10px] text-muted/60 text-center">
              Each pet receives a unique 86-DNA fingerprint &middot;
              Stored forever on Walrus &middot; Dynamic NFT on Sui
            </p>
          </div>
        ) : (
          <motion.div
            initial={{ scale: 0.9, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            className="text-center"
          >
            <div className="mb-4 text-4xl">&#10022;</div>
            <h2 className="text-xl mb-2">&ldquo;{name}&rdquo; has awakened</h2>
            <p className="text-muted text-sm">
              DNA: <span className="font-mono text-xs">{birthResult.dna.slice(0, 16)}...</span>
            </p>
            <p className="mt-4 text-sm text-muted">
              Redirecting to communion chamber...
            </p>
          </motion.div>
        )}
      </motion.div>
    </div>
  )
}
