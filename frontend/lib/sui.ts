/**
 * Ọ̀ṣỌ́ Sui Client — on-chain identity for every pet.
 *
 * Handles dNFT minting (birth), XP evolution, and memory root updates.
 * Uses @mysten/sui for transaction building.
 */

import { SuiClient, getFullnodeUrl } from '@mysten/sui/client'
import { Transaction } from '@mysten/sui/transactions'

// Contract address — set after deployment
const PACKAGE_ID = process.env.NEXT_PUBLIC_SUI_PACKAGE_ID ?? '0x0'
const NETWORK = (process.env.NEXT_PUBLIC_SUI_NETWORK ?? 'testnet') as 'testnet' | 'mainnet' | 'devnet'

export const suiClient = new SuiClient({ url: getFullnodeUrl(NETWORK) })

/**
 * Build a transaction to mint a pet dNFT.
 * The caller signs and executes via their connected wallet.
 */
export function buildBirthTx(params: {
  name: string
  dna: string
  asciiForm: string
}): Transaction {
  const tx = new Transaction()

  tx.moveCall({
    target: `${PACKAGE_ID}::pet::birth`,
    arguments: [
      tx.pure.vector('u8', new TextEncoder().encode(params.name)),
      tx.pure.vector('u8', new TextEncoder().encode(params.dna)),
      tx.pure.vector('u8', new TextEncoder().encode(params.asciiForm)),
    ],
  })

  return tx
}

/**
 * Build a transaction to apply XP growth to a pet.
 */
export function buildEvolveTx(petObjectId: string, xpGain: number): Transaction {
  const tx = new Transaction()

  tx.moveCall({
    target: `${PACKAGE_ID}::pet::evolve`,
    arguments: [
      tx.object(petObjectId),
      tx.pure.u64(xpGain),
    ],
  })

  return tx
}

/**
 * Build a transaction to update the pet's Walrus memory root.
 */
export function buildStoreMemoryTx(petObjectId: string, walrusCid: string): Transaction {
  const tx = new Transaction()

  tx.moveCall({
    target: `${PACKAGE_ID}::pet::store_memory_root`,
    arguments: [
      tx.object(petObjectId),
      tx.pure.vector('u8', new TextEncoder().encode(walrusCid)),
    ],
  })

  return tx
}

/**
 * Build a transaction to update the pet's ASCII form after evolution.
 */
export function buildUpdateAsciiTx(petObjectId: string, newAscii: string): Transaction {
  const tx = new Transaction()

  tx.moveCall({
    target: `${PACKAGE_ID}::pet::update_ascii`,
    arguments: [
      tx.object(petObjectId),
      tx.pure.vector('u8', new TextEncoder().encode(newAscii)),
    ],
  })

  return tx
}
