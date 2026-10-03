# ────────────────────────────────────────────────────────────────
#  Anima‑Vectorium — Safety Altitude
#  Artifact: Rights‑Aligned Membrane (⚖️)
#  Path: vectorium/safety/membrane.py
#  License: Dual — GVL‑1.1 (Geometric) + Stell Non‑Commercial (Expressive)
#  Altitude: 🟧 Safety
# ────────────────────────────────────────────────────────────────

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict


@dataclass
class MembraneResult:
    """Represents the outcome of a safety membrane evaluation."""
    allowed: bool
    score: float
    violations: Dict[str, Any]
    metadata: Dict[str, Any]


class SafetyMembrane:
    """
    ⚖️ Rights‑Aligned Safety Membrane (Safety Altitude)
    Enforces altitude‑aware, rights‑aligned constraints across expressive‑geometric
    operations. The membrane ensures that geometric operators (holonomy, curvature,
    drift, stability) operate within safe feasibility envelopes.

    The membrane is used by:
    - Safety Validator (hard boundary)
    - Runtime Kernel (execution gating)
    - Manifold Simulator (trajectory feasibility)
    - Expressive‑Geometric Bridge (read‑only safety)
    """

    def __init__(self, max_curvature: float = 10.0, max_drift: float = 5.0):
        """
        Initialize the membrane with curvature and drift thresholds.
        These thresholds define the rights‑aligned feasibility envelope.
        """
        self.max_curvature = max_curvature
        self.max_drift = max_drift

    def evaluate(self, diagnostics: Any) -> MembraneResult:
        """
        Evaluate whether a diagnostic packet satisfies the safety membrane.

        Parameters
        ----------
        diagnostics : DiagnosticPacket
            Unified telemetry from geometric altitude.

        Returns
        -------
        MembraneResult
            Indicates whether the operation is allowed and lists violations.
        """
        violations = {}

        # Curvature boundary
        if abs(diagnostics.curvature["scalar"]) > self.max_curvature:
            violations["curvature"] = diagnostics.curvature["scalar"]

        # Drift boundary
        if abs(diagnostics.drift["curvature_influence"]) > self.max_drift:
            violations["drift"] = diagnostics.drift["curvature_influence"]

        # Stability boundary (must be stable)
        if not diagnostics.stability["is_stable"]:
            violations["stability"] = diagnostics.stability["score"]

        # Allowed if no violations
        allowed = len(violations) == 0

        # Score: inverse of violation magnitude
        score = 1.0 / (1.0 + sum(abs(v) for v in violations.values())) if violations else 1.0

        return MembraneResult(
            allowed=allowed,
            score=score,
            violations=violations,
            metadata={
                "operator": "safety_membrane",
                "glyph": "⚖️",
                "mode": "rights-aligned",
                "thresholds": {
                    "max_curvature": self.max_curvature,
                    "max_drift": self.max_drift
                }
            }
        )


# ────────────────────────────────────────────────────────────────
#  Provenance Footer — Rights‑Aligned Membrane (⚖️)
# ────────────────────────────────────────────────────────────────
# Artifact: membrane.py
# Altitude: 🟧 Safety
# Glyph: ⚖️
# Roadmap: vectorium_roadmap_v2.json (artifact: safety-membrane)
# License: Dual — GVL‑1.1 + Stell Non‑Commercial
# Description:
#     Implements the rights‑aligned safety membrane for expressive‑geometric
#     manifolds. Evaluates curvature, drift, and stability boundaries to enforce
#     feasibility envelopes used by the safety validator, runtime kernel, and
#     manifold simulator.
# ────────────────────────────────────────────────────────────────
