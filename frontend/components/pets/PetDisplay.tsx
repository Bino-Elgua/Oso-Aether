'use client'

import { useEffect, useMemo, useRef, useState } from 'react'
import { motion } from 'framer-motion'
import { generateAsciiFrames, type AnimationMode } from '@engine/ascii-renderer/animator'
import { getPetPalette, type Tier } from '@engine/ascii-renderer'
import type { Pet } from '@/lib/state'

interface PetDisplayProps {
  pet: Pet
  isEvolving?: boolean
  interactive?: boolean
  variant?: 'compact' | 'hero'
  showMeta?: boolean
  animationState?: AnimationMode
}

export default function PetDisplay({
  pet,
  isEvolving = false,
  interactive = false,
  variant = 'compact',
  showMeta = true,
  animationState = 'idle',
}: PetDisplayProps) {
  const [frameIndex, setFrameIndex] = useState(0)
  const [isHovered, setIsHovered] = useState(false)
  const animationRef = useRef<ReturnType<typeof setInterval>>(undefined)

  const tier = pet.tier as Tier
  const activeAnimation: AnimationMode = animationState !== 'idle'
    ? animationState
    : interactive && isHovered
      ? 'thinking'
      : 'idle'

  const frames = useMemo(
    () => generateAsciiFrames(pet.dna, tier, pet.personality, activeAnimation),
    [pet.dna, tier, pet.personality, activeAnimation],
  )
  const palette = useMemo(() => getPetPalette(tier, pet.personality), [tier, pet.personality])

  useEffect(() => {
    clearInterval(animationRef.current)
    const speed = activeAnimation === 'evolving'
      ? 180
      : activeAnimation === 'thinking'
        ? 250
        : activeAnimation === 'happy'
          ? 280
          : 760

    animationRef.current = setInterval(() => {
      setFrameIndex((prev) => (prev + 1) % frames.length)
    }, speed)

    return () => clearInterval(animationRef.current)
  }, [activeAnimation, frames.length])

  useEffect(() => {
    setFrameIndex(0)
  }, [activeAnimation, pet.id, tier])

  const isHero = variant === 'hero'
  const showEvolutionGlow = isEvolving || activeAnimation === 'evolving'

  return (
    <motion.div
      className={`ascii-container ${showEvolutionGlow ? 'evolving' : ''} ${isHero ? 'ascii-container-hero' : 'ascii-container-compact'}`}
      onMouseEnter={() => interactive && setIsHovered(true)}
      onMouseLeave={() => interactive && setIsHovered(false)}
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ duration: 0.4 }}
      style={{
        boxShadow: `0 0 28px ${palette.glow}`,
        borderColor: palette.glow,
      }}
    >
      <pre
        className={`ascii-stage ${isHero ? 'text-sm leading-[14px]' : 'text-[11px] leading-[12px]'}`}
        style={{ color: palette.foreground, textShadow: `0 0 14px ${palette.glow}` }}
      >
        {frames[frameIndex]}
      </pre>
      {showMeta && (
        <div className="mt-4 flex items-center justify-between text-[10px] uppercase tracking-[0.24em] text-muted">
          <span>{pet.name} &middot; Tier {pet.tier}</span>
          <span>{Math.floor(pet.xp)} reputation</span>
        </div>
      )}
      {showEvolutionGlow && (
        <motion.div
          className="absolute inset-0 pointer-events-none"
          initial={{ opacity: 0 }}
          animate={{ opacity: [0, 0.32, 0] }}
          transition={{ duration: 2, repeat: Infinity }}
          style={{ background: `radial-gradient(circle, ${palette.glow} 0%, transparent 70%)` }}
        />
      )}
    </motion.div>
  )
}
