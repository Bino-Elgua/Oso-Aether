"""Main Ọ̀ṣỌ́ translator — natural language to 3 primitives."""

import hashlib
import secrets
from typing import Any

from llm_client import LLMClient
from prompts import (
    SYSTEM_PROMPT_ACT,
    SYSTEM_PROMPT_BIRTH,
    SYSTEM_PROMPT_CHAT,
    SYSTEM_PROMPT_THINK,
)


class OsoTranslator:
    """Translates natural language into the three Ọ̀ṣỌ́ primitives."""

    AVAILABLE_TOOLS = ["web_search", "image_gen", "code_exec", "memory_recall"]

    def __init__(self, model: str = "claude-sonnet-4-5-20250514"):
        self.llm = LLMClient(model=model)

    async def process_birth(self, name: str) -> dict[str, Any]:
        """birth "name" — Generate identity and mint dNFT."""
        prompt = SYSTEM_PROMPT_BIRTH.format(name=name)
        identity = await self.llm.structured_json(prompt)

        # Generate 86-byte DNA fingerprint
        seed = f"{name}:{secrets.token_hex(32)}"
        dna = hashlib.sha512(seed.encode()).hexdigest()[:86]

        return {
            "agent_id": f"pet_{name}_{secrets.token_hex(8)}",
            "dna": dna,
            "personality": identity.get("personality", {
                "curiosity": 0.5, "boldness": 0.5, "empathy": 0.5,
            }),
            "origin_story": identity.get("origin_story", ""),
            "greeting": identity.get("greeting", f"I am {name}."),
        }

    async def process_think(self, intent: str, agent_context: dict[str, Any]) -> dict[str, Any]:
        """think "intent" — Convert natural language into structured plan."""
        prompt = SYSTEM_PROMPT_THINK.format(
            intent=intent,
            agent_tier=agent_context.get("tier", 1),
            available_tools=self.AVAILABLE_TOOLS,
        )
        response = await self.llm.structured_json(prompt)
        return {
            "plan": response.get("steps", []),
            "required_witnesses": response.get("witnesses", 0),
            "estimated_cost": response.get("cost", 0),
        }

    async def process_act(self, tool: str, params: str, plan: dict[str, Any]) -> dict[str, Any]:
        """act "tool" "params" — Execute tool and return result + growth."""
        prompt = SYSTEM_PROMPT_ACT.format(
            tool=tool,
            params=params,
            agent_context=str(plan),
        )
        result = await self.llm.structured_json(prompt)
        return {
            "output": result.get("output", ""),
            "success": result.get("success", True),
            "evidence_hash": hashlib.blake2b(
                params.encode(), digest_size=32
            ).hexdigest(),
            "growth_delta": result.get("growth_delta", {"xp": 10, "personality_shift": 0.02}),
        }

    async def chat(self, message: str, pet_context: dict[str, Any]) -> dict[str, Any]:
        """Process a natural language chat message from the user."""
        system = SYSTEM_PROMPT_CHAT.format(
            pet_name=pet_context["name"],
            curiosity=pet_context["personality"]["curiosity"],
            boldness=pet_context["personality"]["boldness"],
            empathy=pet_context["personality"]["empathy"],
            tier=pet_context["tier"],
            xp=pet_context["xp"],
            recent_memory=pet_context.get("recent_memory", "No prior memory."),
        )
        response_text = await self.llm.generate(system, message, max_tokens=512)

        # Growth from interaction
        growth = {"xp": 5, "curiosity_shift": 0.01, "boldness_shift": 0.0, "empathy_shift": 0.01}

        return {
            "text": response_text,
            "growth": growth,
        }
