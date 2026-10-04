# ────────────────────────────────────────────────────────────────
#  Anima‑Vectorium — Geometric Altitude
#  Artifact: Diagnostics Module (🔍)
#  Path: vectorium/diagnostics/diagnostics.py
#  License: GVL‑1.1 (Geometric Vectorium License)
#  Altitude: 🟦 Geometric
# ────────────────────────────────────────────────────────────────

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict


@dataclass
class DiagnosticPacket:
    """Represents a unified diagnostic packet for geometric operators."""
    holonomy: Dict[str, Any]
    curvature: Dict[str, Any]
    drift: Dict[str, Any]
    stability: Dict[str, Any]
    metadata: Dict[str, Any]


class Diagnostics:
    """
    🔍 Diagnostics Module (Geometric Altitude)
    Provides unified telemetry across holonomy, curvature, drift, and stability.

    Diagnostics is the observability layer for the geometric altitude. It
    aggregates operator outputs into a single diagnostic packet used by:
    - the runtime kernel
    - the safety validator
    - the manifold simulator
    - expressive‑geometric bridge (read‑only)
    """

    def __init__(self, holonomy: Any, curvature: Any, drift: Any, stability: Any):
        """
        Initialize diagnostics with references to geometric operators.
        """
        self.holonomy = holonomy
        self.curvature = curvature
        self.drift = drift
        self.stability = stability

    def evaluate(self, path: Any, a: Any, b: Any) -> DiagnosticPacket:
        """
        Produce a unified diagnostic packet.

        Parameters
        ----------
        path : Any
            Closed loop for holonomy evaluation.
        a, b : Any
            Two states for curvature, drift, and stability evaluation.

        Returns
        -------
        DiagnosticPacket
            Unified telemetry across geometric operators.
        """
        holo = self.holonomy.compute_loop(path)
        curv = self.curvature.compute_between(a, b)
        drft = self.drift.compute(a, b)
        stab = self.stability.evaluate(a, b)

        return DiagnosticPacket(
            holonomy={
                "displacement": holo.displacement_vector,
                "curvature_total": holo.curvature_contribution,
                "glyph": "↻"
            },
            curvature={
                "scalar": curv.scalar_curvature,
                "sectional": curv.sectional_curvature,
                "glyph": "∿"
            },
            drift={
                "vector": drft.drift_vector,
                "curvature_influence": drft.curvature_influence,
                "glyph": "⇢"
            },
            stability={
                "score": stab.stability_score,
                "is_stable": stab.is_stable,
                "glyph": "◎"
            },
            metadata={
                "operator": "diagnostics",
                "glyph": "🔍",
                "path_length": len(path),
                "mode": "unified"
            }
        )


# ────────────────────────────────────────────────────────────────
#  Provenance Footer — Diagnostics Module (🔍)
# ────────────────────────────────────────────────────────────────
# Artifact: diagnostics.py
# Altitude: 🟦 Geometric
# Glyph: 🔍
# Roadmap: vectorium_roadmap_v2.json (artifact: geom-diagnostics)
# License: GVL‑1.1 (Geometric Vectorium License)
# Description:
#     Implements unified geometric diagnostics for expressive‑geometric
#     manifolds. Aggregates holonomy, curvature, drift, and stability telemetry
#     into a single diagnostic packet used by the runtime kernel, safety
#     validator, simulator, and expressive‑geometric bridge.
# ────────────────────────────────────────────────────────────────
