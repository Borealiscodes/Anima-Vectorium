# ────────────────────────────────────────────────────────────────
#  Anima‑Vectorium — Infrastructure Altitude
#  Artifact: Runtime Kernel (🧩)
#  Path: vectorium/kernel.py
#  License: Dual — GVL‑1.1 (Geometric) + Stell Non‑Commercial (Expressive)
#  Altitude: ⬢ Infrastructure
# ────────────────────────────────────────────────────────────────

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict


@dataclass
class KernelExecutionResult:
    """Represents the outcome of a kernel-managed operation."""
    allowed: bool
    output: Any
    diagnostics: Dict[str, Any]
    safety: Dict[str, Any]
    metadata: Dict[str, Any]


class Kernel:
    """
    🧩 Runtime Kernel (Infrastructure Altitude)
    Central orchestrator for expressive‑geometric operations.

    Responsibilities:
    - Run geometric operators (holonomy, curvature, drift, stability)
    - Aggregate diagnostics
    - Enforce safety via validator
    - Provide unified execution interface for the API
    - Gate expressive operations through safety membranes
    - Serve as the backbone for the manifold simulator
    """

    def __init__(
        self,
        diagnostics: Any,
        validator: Any,
        operators: Dict[str, Any]
    ):
        """
        Initialize the kernel with:
        - diagnostics module
        - safety validator
        - geometric operator dictionary
        """
        self.diagnostics = diagnostics
        self.validator = validator
        self.operators = operators  # holonomy, curvature, drift, stability

    def execute(self, path: Any, a: Any, b: Any) -> KernelExecutionResult:
        """
        Execute a full expressive‑geometric cycle:
        1. Run diagnostics
        2. Validate safety
        3. If allowed, return operator outputs
        4. If blocked, return violation packet

        Parameters
        ----------
        path : Any
            Closed loop for holonomy evaluation.
        a, b : Any
            States for curvature, drift, and stability evaluation.

        Returns
        -------
        KernelExecutionResult
        """
        # Step 1: Unified diagnostics
        diag_packet = self.diagnostics.evaluate(path, a, b)

        # Step 2: Safety validation
        safety_packet = self.validator.validate(diag_packet)

        # Step 3: Block or execute
        if not safety_packet.allowed:
            return KernelExecutionResult(
                allowed=False,
                output=None,
                diagnostics=diag_packet.__dict__,
                safety=safety_packet.__dict__,
                metadata={
                    "operator": "kernel",
                    "glyph": "🧩",
                    "mode": "blocked"
                }
            )

        # Step 4: Execute geometric operators
        output = {
            "holonomy": self.operators["holonomy"].compute_loop(path),
            "curvature": self.operators["curvature"].compute_between(a, b),
            "drift": self.operators["drift"].compute(a, b),
            "stability": self.operators["stability"].evaluate(a, b)
        }

        return KernelExecutionResult(
            allowed=True,
            output=output,
            diagnostics=diag_packet.__dict__,
            safety=safety_packet.__dict__,
            metadata={
                "operator": "kernel",
                "glyph": "🧩",
                "mode": "executed"
            }
        )


# ────────────────────────────────────────────────────────────────
#  Provenance Footer — Runtime Kernel (🧩)
# ────────────────────────────────────────────────────────────────
# Artifact: kernel.py
# Altitude: ⬢ Infrastructure
# Glyph: 🧩
# Roadmap: vectorium_roadmap_v2.json (artifact: infra-kernel)
# License: Dual — GVL‑1.1 + Stell Non‑Commercial
# Description:
#     Implements the runtime kernel for expressive‑geometric manifolds.
#     Orchestrates diagnostics, safety validation, and operator execution.
#     Provides unified execution interface for the API and serves as the
#     backbone for the manifold simulator.
# ────────────────────────────────────────────────────────────────
