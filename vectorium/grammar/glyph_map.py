# ────────────────────────────────────────────────────────────────
#  Anima‑Vectorium — Infrastructure Altitude
#  Artifact: Glyph Map (🏷️)
#  Path: vectorium/grammar/glyph_map.py
#  License: Dual — GVL‑1.1 (Geometric) + Stell Non‑Commercial (Expressive)
#  Altitude: ⬢ Infrastructure
# ────────────────────────────────────────────────────────────────

from __future__ import annotations
from dataclasses import dataclass
from typing import Dict, Any


@dataclass
class GlyphEntry:
    """Represents a glyph → operator mapping entry."""
    glyph: str
    operator: str
    altitude: str
    metadata: Dict[str, Any]


class GlyphMap:
    """
    🏷️ Glyph Map (Infrastructure Altitude)
    Defines the expressive‑geometric glyph → operator mapping.

    This module provides a stable lookup table used by:
    - the runtime kernel
    - the API layer
    - the simulator
    - expressive‑geometric bridge (read‑only)
    - documentation + diagrams

    Glyphs are altitude‑aware and non‑ambiguous.
    """

    def __init__(self):
        """Initialize the glyph map with governed entries."""
        self.map: Dict[str, GlyphEntry] = {
            "↻": GlyphEntry("↻", "holonomy", "geometric", {"id": "geom-holonomy"}),
            "∿": GlyphEntry("∿", "curvature", "geometric", {"id": "geom-curvature"}),
            "⇢": GlyphEntry("⇢", "drift", "geometric", {"id": "geom-drift"}),
            "◎": GlyphEntry("◎", "stability", "geometric", {"id": "geom-stability"}),
            "🔍": GlyphEntry("🔍", "diagnostics", "geometric", {"id": "geom-diagnostics"}),
            "⚖️": GlyphEntry("⚖️", "membrane", "safety", {"id": "safety-membrane"}),
            "🛡️": GlyphEntry("🛡️", "validator", "safety", {"id": "safety-validator"}),
            "🧩": GlyphEntry("🧩", "kernel", "infrastructure", {"id": "infra-kernel"}),
            "🔌": GlyphEntry("🔌", "api", "infrastructure", {"id": "infra-api"}),
            "🌀": GlyphEntry("🌀", "simulator", "infrastructure", {"id": "infra-simulator"})
        }

    def resolve(self, glyph: str) -> GlyphEntry:
        """
        Resolve a glyph into its operator entry.

        Parameters
        ----------
        glyph : str
            The glyph to resolve.

        Returns
        -------
        GlyphEntry
            The mapped operator entry.

        Raises
        ------
        KeyError
            If the glyph is not defined.
        """
        if glyph not in self.map:
            raise KeyError(f"Unknown glyph: {glyph}")
        return self.map[glyph]


# ────────────────────────────────────────────────────────────────
#  Provenance Footer — Glyph Map (🏷️)
# ────────────────────────────────────────────────────────────────
# Artifact: glyph_map.py
# Altitude: ⬢ Infrastructure
# Glyph: 🏷️
# Roadmap: vectorium_roadmap_v2.json (artifact: infra-glyph-map)
# License: Dual — GVL‑1.1 + Stell Non‑Commercial
# Description:
#     Implements the expressive-geometric glyph → operator mapping used across
#     the runtime kernel, API, simulator, and documentation surfaces. Provides
#     altitude-aware lookup entries for all governed operators.
# ────────────────────────────────────────────────────────────────
