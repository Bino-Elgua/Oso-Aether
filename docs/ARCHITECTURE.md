# Ọ̀ṣỌ́ Architecture

## Data Flow

```
User Types: "I want to learn about constellations"
   |
   v
Python Translator (LLM):
   Parses intent → decides tool: "web_search", params: "constellations 2026"
   Updates personality: curiosity +0.05, empathy +0.02
   Adds 15 XP → checks tier threshold
   |
   v
Rust Interpreter:
   Receives: think "I want to learn..." → routes to translator
   Receives: act "web_search" "constellations 2026" → executes
   Triggers: ascii-renderer.updateFrame(newTier, newPersonality)
   |
   v
Frontend:
   PetDisplay swaps ASCII frame → CSS pulse animation
   Zustand updates pet.xp, pet.tier, pet.personality
   Memory persisted to Walrus → receipt hashed to Sui
```

## Stack

| Layer         | Language   | Purpose                              |
|---------------|------------|--------------------------------------|
| Parser        | Rust       | Strict 3-primitive enforcement       |
| Interpreter   | Rust       | Execution routing + state management |
| Translator    | Python     | LLM bridge (NL → primitives)        |
| Contracts     | Move       | dNFT, registry, memory on Sui       |
| Frontend      | TypeScript | Next.js web app                      |
| Engine        | TypeScript | 86-DNA, ASCII renderer, growth       |

## Storage

- **Sui**: Pet identity (dNFT), XP, tier, DNA fingerprint
- **Walrus**: Permanent memory blobs, conversation history
- **Local**: Zustand persisted state (optimistic cache)
