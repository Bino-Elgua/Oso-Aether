# Ọ̀ṣỌ́ Aether

Rust-powered ASCII AI pets with a strict 3-primitive language, a browser-side WASM runtime, and on-chain identity on Sui.

This repo is no longer built around a Python translator. The active architecture is a Rust parser + interpreter compiled to WebAssembly, with the frontend calling that WASM bridge directly for translation, execution, and agent state updates.

## What It Is

Ọ̀ṣỌ́ Aether is a pet-agent framework where each companion is:

- born with a unique 86-character DNA fingerprint
- shaped through `birth`, `think`, and `act`
- rendered as a deterministic ASCII form that evolves over time
- persisted through Walrus-backed memory and Sui-linked ownership

The tone of the project is intentionally mystical, but the implementation is strict: the language surface is tiny, the parser is explicit, and the runtime behavior lives in Rust.

## The Language

The core language is deliberately minimal:

```txt
birth "ember"
think "I want to learn about constellations"
act "web_search" "night sky patterns 2026"
```

Those are the only three primitives:

- `birth` creates a new agent
- `think` builds identity, memory, and reputation
- `act` executes a tool once the pet has awakened beyond Tier 0

Natural-language input is translated locally by the Rust interpreter. Most user input becomes `think`; only explicit tool-use requests become `act`.

Supported slash commands in the current core include:

- `/private`
- `/publish`
- `/status`
- `/tools`
- `/help`
- `/clear`
- `/export`
- `/personality`
- `/sandbox on|off`

## Current State

What is implemented in this repo right now:

- Rust parser in `core/parser` for the strict 3-primitive syntax
- Rust interpreter in `core/interpreter` for translation, execution, state, tool gating, response generation, and memory rules
- WASM bridge in `core/wasm` exposing `create_agent`, `translate_input`, `execute`, and `process` directly to JavaScript
- Shared TypeScript engine for 86-DNA, ASCII rendering, animation, and growth logic
- Next.js frontend for birth flow, pet gallery, and the centered communion dashboard
- Move contracts for pet birth, XP evolution, ASCII updates, and memory-root storage on Sui
- Walrus-backed memory storage with a local in-memory fallback for development

## Evolution System

The pet renderer and contract now use a full Tier `0-5` evolution ladder:

- Tier 0 — Newborn
- Tier 1 — Common
- Tier 2 — Uncommon
- Tier 3 — Rare
- Tier 4 — Epic
- Tier 5 — Ọ̀ṣỌ́ Sovereign

The ASCII system currently includes:

- 31 named renderer templates in `engine/ascii-renderer/registry.ts`
- deterministic template selection from DNA
- mood-aware animation frames for idle breathing, blinking, thinking, happy states, and evolution
- Tier 5 mask forms where the sacred `Ọ̀ṣỌ́` face becomes the pet's final sovereign visage

The frontend dashboard centers the live pet and surrounds it with:

- reputation and tier progress
- temperament traits: curiosity, boldness, empathy
- unlocked tool ring driven by current growth state
- a communion log for chat history

## Core Concepts

### 86-DNA

Each pet receives an 86-character fingerprint that deterministically influences:

- template selection
- aura and palette tendencies
- personality seed offsets
- long-term visual identity

### Living Odu Memory

The interpreter includes an evolving entropy engine for private memory.

- the initial Odu state is derived from DNA
- each `think` and `act` evolves the active key state
- private memory is meant to move with the pet, not stay static
- ownership transfer rotates the Odu key so prior owners lose access to future private memory

### The Garden

The interpreter also includes a public publishing model called The Garden:

- public thoughts can be published as Garden entries
- public profiles can be assembled for discoverability
- private thoughts are kept separate from published memory

The current frontend is centered on birth, gallery, and chat, while The Garden support already exists in the Rust core data model.

## Architecture

```txt
core/
  parser/        Strict Rust parser for birth/think/act
  interpreter/   Rust runtime for translation, execution, reputation, tools,
                 Living Odu Memory, Garden data, and response generation
  wasm/          wasm-bindgen bridge consumed directly by the frontend

engine/
  86-dna/        Deterministic identity generation
  ascii-renderer/ ASCII template registry, generator, animation, evolution helpers
  growth/        Tier titles, thresholds, and tool unlock metadata

contracts/
  sources/pet.move  Sui dNFT-style pet object with birth/evolve/update calls

frontend/
  app/           Next.js App Router pages for dashboard, birth, gallery, and chat
  components/    Pet renderer, chat interface, and dashboard UI
  lib/           WASM bridge client, Sui tx builders, Walrus client, local state

scripts/
  preview-ascii.mjs  Terminal preview for pet variants and animation modes
```

## Frontend Flow

The current app experience is:

1. Birth a pet with a name and generated DNA.
2. Optionally mint that birth through a connected Sui wallet.
3. Create agent state in Rust via WASM.
4. Persist conversation memory through the Walrus client.
5. Render the live ASCII pet in the communion dashboard.
6. Grow the pet's reputation, temperament, and form through interaction.

## Local Setup

### Prerequisites

- Node.js 18+
- Rust toolchain
- `wasm-pack`
- Sui CLI if you want to publish contracts or test on-chain flows

If you use `rustup`, also add the browser target:

```bash
rustup target add wasm32-unknown-unknown
```

### Install And Run

```bash
cd frontend
npm install
npm run build:wasm
npm run dev
```

This starts the Next.js app locally.

The `build:wasm` script rebuilds the WASM artifacts expected by the frontend.

### Run Rust Tests

```bash
cargo test --workspace
```

### Preview The ASCII Renderer

```bash
node --experimental-strip-types scripts/preview-ascii.mjs --tier=5 --mode=idle
```

You can also animate previews:

```bash
node --experimental-strip-types scripts/preview-ascii.mjs --tier=3 --mode=thinking --animate=true
```

### Publish The Sui Contracts

```bash
cd contracts
sui client publish --gas-budget 100000000 --skip-dependency-verification
```

## Environment Notes

The frontend reads these environment variables when present:

- `NEXT_PUBLIC_SUI_PACKAGE_ID`
- `NEXT_PUBLIC_SUI_NETWORK`
- `WALRUS_PUBLISHER_URL`
- `WALRUS_AGGREGATOR_URL`

If Walrus is unavailable during development, the app falls back to a local in-memory store.

## Tech Stack

- Rust workspace for parser, interpreter, and WASM bridge
- `wasm-bindgen` + `serde-wasm-bindgen` for browser interop
- Next.js 15 + React 19
- Zustand for client state
- Framer Motion for pet animation and motion design
- Tailwind CSS for styling
- Move on Sui for on-chain pet identity and evolution data

## Important Clarification

Older docs referenced a Python translator layer. That is outdated.

The active system in this repository is:

- Rust for parsing and execution
- WASM for browser-side runtime delivery
- Next.js for the user interface
- Sui + Walrus for on-chain identity and durable memory plumbing

## License

MIT
