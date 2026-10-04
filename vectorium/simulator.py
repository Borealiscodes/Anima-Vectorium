# ────────────────────────────────────────────────────────────────
#  Anima‑Vectorium — Infrastructure Altitude
#  Artifact: Manifold Simulator (🌀)
#  Path: vectorium/simulator.py
#  License: Dual — GVL‑1.1 (Geometric) + Stell Non‑Commercial (Expressive)
#  Altitude: ⬢ Infrastructure
# ────────────────────────────────────────────────────────────────

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict, List


@dataclass
class SimulationStep:
    """Represents a single step in a manifold simulation."""
    state: Any
    diagnostics: Dict[str, Any]
    safety: Dict[str, Any]
    allowed: bool
    metadata: Dict[str, Any]


@dataclass
class SimulationTrace:
    """Represents the full trace of a manifold simulation."""
    steps: List[SimulationStep]
    terminated: bool
    reason: str
    metadata: Dict[str, Any]


class Simulator:
    """
    🌀 Manifold Simulator (Infrastructure Altitude)
    Traverses expressive‑geometric manifolds using the runtime kernel.

    Responsibilities:
    - Stepwise traversal of manifold states
    - Kernel execution at each step
    - Safety validation at each step
    - Prevent runaway trajectories
    - Produce structured simulation traces
    """

    def __init__(self, kernel: Any):
        """
        Initialize the simulator with a runtime kernel reference.
        """
        self.kernel = kernel

    def run(self, path: Any, trajectory: List[Any]) -> SimulationTrace:
        """
        Run a manifold simulation across a trajectory.

        Parameters
        ----------
        path : Any
            Closed loop for holonomy evaluation.
        trajectory : List[Any]
            Sequence of states to traverse.

        Returns
        -------
        SimulationTrace
        """
        steps: List[SimulationStep] = []

        for i in range(len(trajectory) - 1):
            a = trajectory[i]
            b = trajectory[i + 1]

            kernel_result = self.kernel.execute(path, a, b)

            step = SimulationStep(
                state=b,
                diagnostics=kernel_result.diagnostics,
                safety=kernel_result.safety,
                allowed=kernel_result.allowed,
                metadata={
                    "operator": "simulator",
                    "glyph": "🌀",
                    "step_index": i
                }
            )

            steps.append(step)

            # Terminate if safety blocks execution
            if not kernel_result.allowed:
                return SimulationTrace(
                    steps=steps,
                    terminated=True,
                    reason="safety_violation",
                    metadata={
                        "operator": "simulator",
                        "glyph": "🌀",
                        "mode": "terminated"
                    }
                )

        return SimulationTrace(
            steps=steps,
            terminated=False,
            reason="completed",
            metadata={
                "operator": "simulator",
                "glyph": "🌀",
                "mode": "completed"
            }
        )


# ────────────────────────────────────────────────────────────────
#  Provenance Footer — Manifold Simulator (🌀)
# ────────────────────────────────────────────────────────────────
# Artifact: simulator.py
# Altitude: ⬢ Infrastructure
# Glyph: 🌀
# Roadmap: vectorium_roadmap_v2.json (artifact: infra-simulator)
# License: Dual — GVL‑1.1 + Stell Non‑Commercial
# Description:
#     Implements the manifold simulator for expressive‑geometric systems.
#     Performs stepwise traversal using the runtime kernel, enforcing safety
#     boundaries and producing structured simulation traces for demos and
#     research. Prevents runaway trajectories and integrates altitude-aware
#     safety logic.
# ────────────────────────────────────────────────────────────────
