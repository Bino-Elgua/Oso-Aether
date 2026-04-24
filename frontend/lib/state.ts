import { create } from 'zustand'
import { persist } from 'zustand/middleware'

export type Pet = {
  id: string
  name: string
  dna: string                // 86-char hex fingerprint
  asciiForm: string
  tier: number
  xp: number
  personality: {
    curiosity: number        // 0-1
    boldness: number
    empathy: number
  }
  memoryRoot: string         // Walrus CID
  createdAt: number
}

type OsoState = {
  pets: Pet[]
  activePetId: string | null
  addPet: (pet: Pet) => void
  updatePet: (id: string, updates: Partial<Pet>) => void
  setActivePet: (id: string | null) => void
  getActivePet: () => Pet | null
}

export const useAetherStore = create<OsoState>()(
  persist(
    (set, get) => ({
      pets: [],
      activePetId: null,

      addPet: (pet) =>
        set((state) => ({ pets: [...state.pets, pet] })),

      updatePet: (id, updates) =>
        set((state) => ({
          pets: state.pets.map((p) => (p.id === id ? { ...p, ...updates } : p)),
        })),

      setActivePet: (id) => set({ activePetId: id }),

      getActivePet: () => {
        const { pets, activePetId } = get()
        return pets.find((p) => p.id === activePetId) ?? null
      },
    }),
    {
      name: 'oso-storage',
      partialize: (state) => ({
        pets: state.pets,
        activePetId: state.activePetId,
      }),
    },
  ),
)
