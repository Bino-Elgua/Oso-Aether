"""Ọ̀ṣỌ́ Translator — FastAPI server.

Alternative backend for the translator layer. The Next.js API routes
can call this directly, or it can run standalone for development/testing.

    uvicorn server:app --reload --port 8000
"""

import hashlib
import secrets

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

from translator import OsoTranslator

app = FastAPI(
    title="Ọ̀ṣỌ́ Translator",
    description="Natural language → 3 primitives (birth / think / act)",
    version="0.1.0",
)

translator = OsoTranslator()


# ─── Request / Response Models ──────────────────────────────────────────────

class BirthRequest(BaseModel):
    name: str

class BirthResponse(BaseModel):
    agentId: str
    dna: str
    personality: dict
    asciiPreview: str
    greeting: str
    originStory: str
    walrusCid: str

class ThinkRequest(BaseModel):
    agentId: str
    intent: str
    context: dict | None = None

class ThinkResponse(BaseModel):
    text: str
    primitives: list[dict]
    growth: bool
    newXp: int
    newTier: int
    personalityShift: dict | None = None

class ChatRequest(BaseModel):
    agentId: str
    message: str
    petContext: dict

class ChatResponse(BaseModel):
    text: str
    growth: dict


# ─── Endpoints ──────────────────────────────────────────────────────────────

@app.post("/api/translator/birth", response_model=BirthResponse)
async def birth(req: BirthRequest):
    """birth "name" — Forge a new pet soul."""
    if not req.name or len(req.name) > 24:
        raise HTTPException(status_code=400, detail="Name required (max 24 chars)")

    try:
        result = await translator.process_birth(req.name)

        # Generate 86-DNA fingerprint
        seed = f"{req.name}:{secrets.token_hex(32)}"
        dna = hashlib.sha512(seed.encode()).hexdigest()[:86]

        return BirthResponse(
            agentId=result["agent_id"],
            dna=dna,
            personality=result["personality"],
            asciiPreview="  .--.\n ( o o )\n  >--<",  # Tier 1 placeholder
            greeting=result["greeting"],
            originStory=result["origin_story"],
            walrusCid=f"local_{result['agent_id']}",
        )
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"The forge faltered: {e}")


@app.post("/api/translator/think", response_model=ThinkResponse)
async def think(req: ThinkRequest):
    """think "intent" — Process natural language into primitives."""
    if not req.intent:
        raise HTTPException(status_code=400, detail="Intent required")

    ctx = req.context or {"tier": 1, "personality": {"curiosity": 0.5, "boldness": 0.5, "empathy": 0.5}}

    try:
        plan = await translator.process_think(req.intent, ctx)

        # Build primitive list from plan
        primitives = [{"command": "think", "args": [req.intent]}]
        for step in plan.get("plan", []):
            primitives.append({
                "command": "act",
                "args": [step.get("tool", ""), step.get("params", "")],
            })

        current_xp = ctx.get("xp", 0)
        new_xp = current_xp + 10

        # Tier thresholds
        new_tier = 1
        if new_xp >= 10000:
            new_tier = 5
        elif new_xp >= 2000:
            new_tier = 4
        elif new_xp >= 500:
            new_tier = 3
        elif new_xp >= 100:
            new_tier = 2

        return ThinkResponse(
            text=f"I understand: {req.intent}",
            primitives=primitives,
            growth=True,
            newXp=new_xp,
            newTier=new_tier,
            personalityShift=ctx.get("personality"),
        )
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Soul-fray trembles: {e}")


@app.post("/api/translator/chat", response_model=ChatResponse)
async def chat(req: ChatRequest):
    """Chat with a pet in natural language."""
    try:
        result = await translator.chat(req.message, req.petContext)
        return ChatResponse(
            text=result["text"],
            growth=result["growth"],
        )
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Connection interrupted: {e}")


@app.get("/health")
async def health():
    return {"status": "alive", "forge": "burning"}
