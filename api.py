# ────────────────────────────────────────────────────────────────
#  Anima‑Vectorium — Infrastructure Altitude
#  Artifact: Unified API Layer (🔌)
#  Path: vectorium/api.py
#  License: Dual — GVL‑1.1 (Geometric) + Stell Non‑Commercial (Expressive)
#  Altitude: ⬢ Infrastructure
# ────────────────────────────────────────────────────────────────

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict


@dataclass
class APIResponse:
    """Represents a structured API response from the kernel."""
    allowed: bool
    result: Any
    diagnostics: Dict[str, Any]
    safety: Dict[str, Any]
    metadata: Dict[str, Any]


class VectoriumAPI:
    """
    🔌 Unified API Layer (Infrastructure Altitude)
    Provides a stable, altitude-aware interface for interacting with the
    expressive‑geometric runtime kernel.

    Responsibilities:
    - Expose kernel execution in a clean, stable API
    - Provide structured responses for external callers
    - Enforce altitude boundaries (no direct operator access)
    - Serve as the entry point for demos, notebooks, and external integrations
    """

    def __init__(self, kernel: Any):
        """
        Initialize the API with a runtime kernel reference.
        """
        self.kernel = kernel

    def run(self, path: Any, a: Any, b: Any) -> APIResponse:
        """
        Execute a full expressive‑geometric cycle through the kernel.

        Parameters
        ----------
        path : Any
            Closed loop for holonomy evaluation.
        a, b : Any
            States for curvature, drift, and stability evaluation.

        Returns
        -------
        APIResponse
        """
        kernel_result = self.kernel.execute(path, a, b)

        return APIResponse(
            allowed=kernel_result.allowed,
            result=kernel_result.output,
            diagnostics=kernel_result.diagnostics,
            safety=kernel_result.safety,
            metadata={
                "operator": "api",
                "glyph": "🔌",
                "mode": "kernel-delegation"
            }
        )


# ────────────────────────────────────────────────────────────────
#  Provenance Footer — Unified API Layer (🔌)
# ────────────────────────────────────────────────────────────────
# Artifact: api.py
# Altitude: ⬢ Infrastructure
# Glyph: 🔌
# Roadmap: vectorium_roadmap_v2.json (artifact: infra-api)
# License: Dual — GVL‑1.1 + Stell Non‑Commercial
# Description:
#     Implements the unified API layer for Anima-Vectorium. Provides a stable,
#     altitude-aware interface for executing expressive-geometric operations
#     through the runtime kernel. Used by demos, notebooks, and external
#     integrations. Includes structured API responses and provenance metadata.
# ────────────────────────────────────────────────────────────────
