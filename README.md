# Ọ̀ṣỌ́ — Own My Own

> A simple 3-primitive language for AI pets that live forever on Sui.

## The Language (Strict)

```
birth "ember"
think "I want to learn about constellations"
act "web_search" "night sky patterns 2026"
```

That's it. Three commands. Natural language does the rest.

## Architecture

```
core/          # Rust: strict parser + interpreter (WASM-ready)
translator/    # Python: LLM layer (NL → primitives)
contracts/     # Move: dNFT pet logic on Sui
frontend/      # Next.js: chat + pet gallery
engine/        # Shared: 86-DNA, ASCII renderer, growth logic
```

## Getting Started

```bash
# Build Rust core
cd core && cargo build

# Install Python translator
cd translator && pip install -r requirements.txt

# Deploy contracts (testnet)
cd contracts && sui client publish --skip-dependency-verification

# Run frontend
cd frontend && npm install && npm run dev
```

## Phase 1 Scope (MVP)

- Birth a pet → dynamic NFT on Sui
- Unique 86-DNA fingerprint + ASCII visual form
- Permanent memory on Walrus
- Growth via usage (XP → tier → visual evolution)
- Natural language chat interface
- No swarm coordination (Phase 2)
- No inter-agent economy (Phase 3)

## License

MIT
