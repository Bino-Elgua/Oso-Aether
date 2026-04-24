"""LLM client wrapper for Ọ̀ṣỌ́ — supports Claude via Anthropic SDK."""

import json
import os
from typing import Any

import anthropic


class LLMClient:
    def __init__(self, model: str = "claude-sonnet-4-5-20250514"):
        self.model = model
        self.client = anthropic.AsyncAnthropic(
            api_key=os.environ.get("ANTHROPIC_API_KEY", ""),
        )

    async def generate(self, system: str, user_message: str, max_tokens: int = 2048) -> str:
        """Generate a text response."""
        response = await self.client.messages.create(
            model=self.model,
            max_tokens=max_tokens,
            system=system,
            messages=[{"role": "user", "content": user_message}],
        )
        return response.content[0].text

    async def structured_json(self, prompt: str, max_tokens: int = 2048) -> dict[str, Any]:
        """Generate a structured JSON response."""
        response = await self.client.messages.create(
            model=self.model,
            max_tokens=max_tokens,
            system="You respond only with valid JSON. No markdown, no explanation.",
            messages=[{"role": "user", "content": prompt}],
        )
        text = response.content[0].text.strip()
        # Strip markdown code fences if present
        if text.startswith("```"):
            text = text.split("\n", 1)[1]
            text = text.rsplit("```", 1)[0]
        return json.loads(text)
