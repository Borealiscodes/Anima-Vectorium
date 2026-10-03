# ────────────────────────────────────────────────────────────────
#  Anima‑Vectorium — Geometric Altitude
#  Artifact: Stability Operator (◎)
#  Path: vectorium/core/stability.py
#  License: GVL‑1.1 (Geometric Vectorium License)
#  Altitude: 🟦 Geometric
# ────────────────────────────────────────────────────────────────

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict


@dataclass
class StabilityResult:
    """Represents stability evaluation across expressive‑geometric flow."""
    is_stable: bool
    stability_score: float
    curvature_factor: float
    drift_factor: float
    metadata: Dict[str, Any]


class Stability:
    """
    ◎ Stability Operator (Geometric Altitude)
    Determines whether expressive‑geometric trajectories remain bounded.

    Stability integrates:
    - holonomy displacement (loop effects)
    - curvature (manifold bending)
    - drift (directional deviation)

    The runtime kernel uses stability to enforce feasibility envelopes.
    The simulator uses stability to prevent runaway trajectories.
    Diagnostics use stability to detect anomalies.
    """

    def __init__(self, manifold: Any, curvature: Any, drift: Any):
        """
        Initialize the stability operator with manifold, curvature, and drift.
        """
        self.manifold = manifold
        self.curvature = curvature
        self.drift = drift

    def evaluate(self, a: Any, b: Any) -> StabilityResult:
        """
        Evaluate stability between two manifold states.

        Parameters
        ----------
        a, b : Any
            Two states in the expressive‑geometric manifold.

        Returns
        -------
        StabilityResult
            Contains stability score, curvature factor, drift factor, and metadata.
        """
        # Curvature contribution
        curv_res = self.curvature.compute_between(a, b)
        curvature_factor = curv_res.scalar_curvature

        # Drift contribution
        drift_res = self.drift.compute(a, b)
        drift_factor = drift_res.curvature_influence

        # Stability score: inverse of combined curvature + drift magnitude
        score = 1.0 / (1.0 + abs(curvature_factor) + abs(drift_factor))

        # Stable if score above threshold
        is_stable = score >= 0.25

        return StabilityResult(
            is_stable=is_stable,
            stability_score=score,
            curvature_factor=curvature_factor,
            drift_factor=drift_factor,
            metadata={
                "operator": "stability",
                "glyph": "◎",
                "threshold": 0.25,
                "mode": "pairwise"
            }
        )


# ────────────────────────────────────────────────────────────────
#  Provenance Footer — Stability Operator (◎)
# ────────────────────────────────────────────────────────────────
# Artifact: stability.py
# Altitude: 🟦 Geometric
# Glyph: ◎
# Roadmap: vectorium_roadmap_v2.json (artifact: geom-stability)
# License: GVL‑1.1 (Geometric Vectorium License)
# Description:
#     Implements stability evaluation for expressive‑geometric manifolds.
#     Integrates curvature and drift contributions to compute feasibility
#     envelopes used by the runtime kernel, diagnostics, and the simulator.
# ────────────────────────────────────────────────────────────────
