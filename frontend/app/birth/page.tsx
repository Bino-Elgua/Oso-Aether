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
      const trimmedName = name.trim()

      // Generate unique ID and DNA
      const entropy = crypto.randomUUID()
      const agentId = `pet_${trimmedName.toLowerCase().replace(/\s+/g, '-')}_${entropy.slice(0, 8)}`
      const dna = entropy.replace(/-/g, '').slice(0, 86).padEnd(86, '0')

      // Create agent in Rust WASM (rep 0, Tier 0, default personality)
      const agent = await aether.createAgent(agentId, trimmedName, dna)

      // Execute birth primitive to get the greeting
      // Payment required — build a confirmation from the Sui tx
      let suiTxHash = ''
      let greeting = ''

      if (account) {
        setStage('minting')
        try {
          const tx = buildBirthTx({
            name: trimmedName,
            dna,
            asciiForm: '',
          })
          const txResult = await signAndExecute({ transaction: tx })
          suiTxHash = txResult.digest

          // Execute birth with valid payment
          const birthResult = await aether.execute(
            { Birth: { name: trimmedName } },
            agent,
            {
              tx_digest: txResult.digest,
              amount_mist: 100_000_000,
              sender: account.address,
            },
          )
          greeting = birthResult.response.message
        } catch (err) {
          console.warn('Sui mint skipped:', err)
        }
      }

      // If no wallet or mint failed, still show a greeting
      if (!greeting) {
        // Can't execute birth without payment, use a simple greeting
        greeting = `Hi. I'm ${trimmedName}. I don't really know what I am yet — what do you need me to be?`
      }

      const newPet = {
        id: agentId,
        name: trimmedName,
        dna,
        asciiForm: '',
        tier: 0 as const,
        xp: 0,
        personality: agent.personality,
        memoryRoot: '',
        createdAt: Date.now(),
      }

      addPet(newPet)

      // Store initial memory with greeting
      await aether.storeMemory(agentId, [
        {
          id: crypto.randomUUID(),
          role: 'pet',
          content: greeting,
          timestamp: Date.now(),
        },
      ]).catch(() => {})

      setBirthResult({ id: agentId, dna })
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
