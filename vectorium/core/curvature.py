# ────────────────────────────────────────────────────────────────
#  Anima‑Vectorium — Geometric Altitude
#  Artifact: Curvature Operator (∿)
#  Path: vectorium/core/curvature.py
#  License: GVL‑1.1 (Geometric Vectorium License)
#  Altitude: 🟦 Geometric
# ────────────────────────────────────────────────────────────────

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict, Tuple


@dataclass
class CurvatureResult:
    """Represents curvature at a point or between two states."""
    scalar_curvature: float
    sectional_curvature: float
    metadata: Dict[str, Any]


class Curvature:
    """
    ∿ Curvature Operator (Geometric Altitude)
    Measures geometric bending across expressive‑geometric trajectories.

    Curvature quantifies how the manifold deviates from flatness. It is used
    by drift, stability, diagnostics, and the manifold simulator to determine
    how expressive trajectories deform over time.

    This operator is tightly coupled with holonomy: holonomy loops accumulate
    curvature, and curvature determines how loops behave.
    """

    def __init__(self, manifold: Any):
        """
        Initialize the curvature operator with a manifold reference.
        The manifold is expected to expose metric and holonomy functions.
        """
        self.manifold = manifold

    def compute_point_curvature(self, state: Any) -> CurvatureResult:
        """
        Compute curvature at a single manifold state.

        Parameters
        ----------
        state : Any
            A point in the expressive‑geometric manifold.

        Returns
        -------
        CurvatureResult
            Contains scalar curvature, sectional curvature, and metadata.
        """
        scalar = self.manifold.scalar_curvature(state)
        sectional = self.manifold.sectional_curvature(state)

        return CurvatureResult(
            scalar_curvature=scalar,
            sectional_curvature=sectional,
            metadata={
                "operator": "curvature",
                "glyph": "∿",
                "mode": "point"
            }
        )

    def compute_between(self, a: Any, b: Any) -> CurvatureResult:
        """
        Compute curvature contribution between two states.

        Parameters
        ----------
        a, b : Any
            Two states in the expressive‑geometric manifold.

        Returns
        -------
        CurvatureResult
            Curvature contribution between states.
        """
        scalar = self.manifold.scalar_curvature_between(a, b)
        sectional = self.manifold.sectional_curvature_between(a, b)

        return CurvatureResult(
            scalar_curvature=scalar,
            sectional_curvature=sectional,
            metadata={
                "operator": "curvature",
                "glyph": "∿",
                "mode": "between"
            }
        )


# ────────────────────────────────────────────────────────────────
#  Provenance Footer — Curvature Operator (∿)
# ────────────────────────────────────────────────────────────────
# Artifact: curvature.py
# Altitude: 🟦 Geometric
# Glyph: ∿
# Roadmap: vectorium_roadmap_v2.json (artifact: geom-curvature)
# License: GVL‑1.1 (Geometric Vectorium License)
# Description:
#     Implements scalar and sectional curvature measurement for expressive-
#     geometric manifolds. Provides pointwise and pairwise curvature operators
#     used by drift, stability, diagnostics, and the manifold simulator.
# ────────────────────────────────────────────────────────────────
