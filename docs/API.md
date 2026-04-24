# Ọ̀ṣỌ́ API Reference

## Translator Endpoints

### `POST /api/translator/birth`

Birth a new pet.

**Request:**
```json
{ "name": "ember" }
```

**Response:**
```json
{
  "agentId": "pet_ember_a1b2c3d4",
  "dna": "a1b2c3...86chars",
  "personality": { "curiosity": 0.7, "boldness": 0.4, "empathy": 0.8 },
  "asciiPreview": "  /\\_/\\\n ( o o )\n  >--<",
  "walrusCid": "bafyrei...",
  "suiTxHash": "0x..."
}
```

### `POST /api/translator/think`

Process a natural language intent.

**Request:**
```json
{
  "agentId": "pet_ember_a1b2c3d4",
  "intent": "I want to learn about constellations",
  "context": { "tier": 2, "personality": { "curiosity": 0.7 } }
}
```

**Response:**
```json
{
  "text": "Ooh, constellations! Let me look into that...",
  "growth": true,
  "newXp": 115,
  "newTier": 2,
  "personalityShift": { "curiosity": 0.72, "boldness": 0.4, "empathy": 0.81 }
}
```

### `GET /api/memory/:agentId`

Load pet conversation history from Walrus.

### `POST /api/memory/:agentId`

Store new memory to Walrus.

**Request:**
```json
{
  "messages": [
    { "role": "user", "content": "Hello", "timestamp": 1714000000 },
    { "role": "pet", "content": "Hi there!", "timestamp": 1714000001 }
  ]
}
```
