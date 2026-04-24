'use client'

import { useState } from 'react'
import { useRouter } from 'next/navigation'
import { motion } from 'framer-motion'
import { useSignAndExecuteTransaction, useCurrentAccount } from '@mysten/dapp-kit'
import { useAetherStore } from '@/lib/state'
import { aether } from '@/lib/aether'
import { buildBirthTx } from '@/lib/sui'

export default function BirthPage() {
  const [name, setName] = useState('')
  const [isBirthing, setIsBirthing] = useState(false)
  const [stage, setStage] = useState<'input' | 'forging' | 'minting' | 'done'>('input')
  const [birthResult, setBirthResult] = useState<{ id: string; dna: string } | null>(null)
  const { addPet, setActivePet } = useAetherStore()
  const router = useRouter()
  const account = useCurrentAccount()
  const { mutateAsync: signAndExecute } = useSignAndExecuteTransaction()

  const handleBirth = async () => {
    if (!name.trim()) return
    setIsBirthing(true)
    setStage('forging')

    try {
      // Step 1: Forge the soul (API call to Claude + 86-DNA generation)
      const result = await aether.birthPet(name.trim())

      // Step 2: Mint on Sui (if wallet connected)
      let suiTxHash = ''
      if (account) {
        setStage('minting')
        try {
          const tx = buildBirthTx({
            name: name.trim(),
            dna: result.dna,
            asciiForm: result.asciiPreview,
          })
          const txResult = await signAndExecute({ transaction: tx })
          suiTxHash = txResult.digest
        } catch (err) {
          console.warn('Sui mint skipped:', err)
          // Continue without on-chain mint — pet still lives locally + Walrus
        }
      }

      const newPet = {
        id: result.agentId,
        name: name.trim(),
        dna: result.dna,
        asciiForm: result.asciiPreview,
        tier: 1 as const,
        xp: 0,
        personality: result.personality ?? {
          curiosity: Math.random(),
          boldness: Math.random(),
          empathy: Math.random(),
        },
        memoryRoot: result.walrusCid,
        suiTxHash,
        createdAt: Date.now(),
      }

      addPet(newPet)
      setBirthResult({ id: result.agentId, dna: result.dna })
      setStage('done')

      setTimeout(() => {
        setActivePet(newPet.id)
        router.push(`/chat/${newPet.id}`)
      }, 2500)
    } catch (err) {
      console.error('Birth failed:', err)
      setStage('input')
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

        {stage === 'input' && (
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

            {!account && (
              <p className="text-[10px] text-amber-400/80 text-center">
                Connect a Sui wallet to mint as dNFT. Without wallet, pet lives off-chain.
              </p>
            )}

            <button
              onClick={handleBirth}
              disabled={!name.trim() || isBirthing}
              className="w-full rounded-lg bg-ember px-4 py-3 text-sm font-medium text-white disabled:opacity-50 hover:bg-ember/90 transition-colors"
            >
              Birth Pet
            </button>

            <p className="text-[10px] text-muted/60 text-center">
              Each pet receives a unique 86-DNA fingerprint &middot;
              Stored forever on Walrus &middot; Dynamic NFT on Sui
            </p>
          </div>
        )}

        {(stage === 'forging' || stage === 'minting') && (
          <div className="text-center space-y-4">
            <motion.div
              animate={{ rotate: 360 }}
              transition={{ duration: 2, repeat: Infinity, ease: 'linear' }}
              className="text-4xl inline-block"
            >
              &#10022;
            </motion.div>
            <p className="text-sm text-muted">
              {stage === 'forging' ? 'Forging soul from the void...' : 'Inscribing on Sui...'}
            </p>
          </div>
        )}

        {stage === 'done' && birthResult && (
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
              Entering communion chamber...
            </p>
          </motion.div>
        )}
      </motion.div>
    </div>
  )
}
