'use client'

import { useEffect, useRef, useState } from 'react'
import { motion } from 'framer-motion'
import { generateAsciiFrames } from '@engine/ascii-renderer/animator'
import type { Pet } from '@/lib/state'

interface PetDisplayProps {
  pet: Pet
  isEvolving?: boolean
  interactive?: boolean
}

export default function PetDisplay({ pet, isEvolving = false, interactive = false }: PetDisplayProps) {
  const [frameIndex, setFrameIndex] = useState(0)
  const [isHovered, setIsHovered] = useState(false)
  const animationRef = useRef<ReturnType<typeof setInterval>>(undefined)

  const frames = generateAsciiFrames(
    pet.dna,
    pet.tier as 1 | 2 | 3 | 4 | 5,
    pet.personality,
  )

  useEffect(() => {
    if (isEvolving || (interactive && isHovered)) {
      animationRef.current = setInterval(() => {
        setFrameIndex((prev) => (prev + 1) % frames.length)
      }, 200)
    }
    return () => clearInterval(animationRef.current)
  }, [isEvolving, interactive, isHovered, frames.length])

  const getAccentColor = () => {
    const { curiosity, boldness, empathy } = pet.personality
    if (curiosity > 0.7) return 'text-blue-400'
    if (boldness > 0.7) return 'text-amber-400'
    if (empathy > 0.7) return 'text-emerald-400'
    return 'text-purple-400'
  }

  return (
    <motion.div
      className={`ascii-container ${isEvolving ? 'evolving' : ''}`}
      onMouseEnter={() => interactive && setIsHovered(true)}
      onMouseLeave={() => interactive && setIsHovered(false)}
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ duration: 0.4 }}
    >
      <pre className={`font-mono text-xs leading-[10px] ${getAccentColor()}`}>
        {frames[frameIndex]}
      </pre>
      <div className="mt-3 flex items-center justify-between text-[10px] text-muted">
        <span>{pet.name} &middot; Tier {pet.tier}</span>
        <span>{Math.floor(pet.xp)} XP</span>
      </div>
      {isEvolving && (
        <motion.div
          className="absolute inset-0 pointer-events-none"
          initial={{ opacity: 0 }}
          animate={{ opacity: [0, 0.3, 0] }}
          transition={{ duration: 2, repeat: Infinity }}
          style={{ background: 'radial-gradient(circle, rgba(109,93,252,0.2) 0%, transparent 70%)' }}
        />
      )}
    </motion.div>
  )
}
