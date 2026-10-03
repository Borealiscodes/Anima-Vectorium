# ────────────────────────────────────────────────────────────────
#  Anima‑Vectorium — Safety Altitude
#  Artifact: Safety Validator (🛡️)
#  Path: vectorium/safety/validator.py
#  License: Dual — GVL‑1.1 (Geometric) + Stell Non‑Commercial (Expressive)
#  Altitude: 🟧 Safety
# ────────────────────────────────────────────────────────────────

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict


@dataclass
class ValidationResult:
    """Represents the outcome of a safety validation check."""
    allowed: bool
    violations: Dict[str, Any]
    membrane_score: float
    metadata: Dict[str, Any]


class SafetyValidator:
    """
    🛡️ Safety Validator (Safety Altitude)
    Enforces hard-boundary safety constraints across expressive‑geometric
    operations. The validator consumes:
      - unified diagnostics (holonomy, curvature, drift, stability)
      - membrane evaluation (rights‑aligned feasibility envelope)

    The validator is the final authority before the runtime kernel executes
    any geometric or expressive operation.
    """

    def __init__(self, membrane: Any):
        """
        Initialize the validator with a safety membrane reference.
        """
        self.membrane = membrane

    def validate(self, diagnostics: Any) -> ValidationResult:
        """
        Validate a diagnostic packet against the safety membrane.

        Parameters
        ----------
        diagnostics : DiagnosticPacket
            Unified telemetry from geometric altitude.

        Returns
        -------
        ValidationResult
            Indicates whether the operation is allowed and lists violations.
        """
        membrane_result = self.membrane.evaluate(diagnostics)

        violations = {}

        # Membrane-level violations
        if not membrane_result.allowed:
            violations.update(membrane_result.violations)

        # Additional hard-boundary checks
        # Stability must be true
        if not diagnostics.stability["is_stable"]:
            violations["hard_stability"] = diagnostics.stability["score"]

        # Drift cannot exceed extreme thresholds
        if abs(diagnostics.drift["curvature_influence"]) > (self.membrane.max_drift * 2):
            violations["hard_drift"] = diagnostics.drift["curvature_influence"]

        allowed = len(violations) == 0

        return ValidationResult(
            allowed=allowed,
            violations=violations,
            membrane_score=membrane_result.score,
            metadata={
                "operator": "safety_validator",
                "glyph": "🛡️",
                "mode": "hard-boundary",
                "membrane_thresholds": {
                    "max_curvature": self.membrane.max_curvature,
                    "max_drift": self.membrane.max_drift
                }
            }
        )


# ────────────────────────────────────────────────────────────────
#  Provenance Footer — Safety Validator (🛡️)
# ────────────────────────────────────────────────────────────────
# Artifact: validator.py
# Altitude: 🟧 Safety
# Glyph: 🛡️
# Roadmap: vectorium_roadmap_v2.json (artifact: safety-validator)
# License: Dual — GVL‑1.1 + Stell Non‑Commercial
# Description:
#     Implements the hard-boundary safety validator for expressive‑geometric
#     manifolds. Consumes unified diagnostics and membrane evaluations to enforce
#     strict feasibility envelopes before kernel execution. Provides structured
#     violation packets for the runtime kernel and simulator.
# ────────────────────────────────────────────────────────────────
