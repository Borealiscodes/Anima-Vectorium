# ────────────────────────────────────────────────────────────────
#  Anima‑Vectorium — Bridge Altitude
#  Artifact: Kernel Bridge (🧷)
#  Path: bridge/bridge_kernel.py
#  License: Stell Non‑Commercial (Expressive)
#  Altitude: 🟫 Bridge
# ────────────────────────────────────────────────────────────────

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict
from bridge.bridge_readonly import BridgePacket
from vectorium.kernel import Kernel


@dataclass
class KernelBridgeResult:
    """Represents the result of a kernel-bridged expressive execution."""
    allowed: bool
    output: Any
    diagnostics: Dict[str, Any]
    safety: Dict[str, Any]
    metadata: Dict[str, Any]


class KernelBridge:
    """
    🧷 Kernel Bridge (Bridge Altitude)
    Provides a safe, altitude-aware interface between expressive projections
    and the runtime kernel.

    Responsibilities:
    - Accept expressive → geometric projections (BridgePacket)
    - Prepare geometric inputs for kernel execution
    - Enforce altitude boundaries (expressive is read-only)
    - Provide structured kernel-bridged results
    """

    def __init__(self, kernel: Kernel):
        """Initialize the kernel bridge with a runtime kernel reference."""
        self.kernel = kernel

    def execute(self, packet: BridgePacket, a: Any, b: Any) -> KernelBridgeResult:
        """
        Execute a kernel operation using an expressive projection.

        Parameters
        ----------
        packet : BridgePacket
            Read-only expressive → geometric projection.
        a, b : Any
            Additional geometric states for curvature/drift/stability.

        Returns
        -------
        KernelBridgeResult
        """
        # The projection becomes the holonomy path input
        path = packet.geometric_projection

        kernel_result = self.kernel.execute(path, a, b)

        return KernelBridgeResult(
            allowed=kernel_result.allowed,
            output=kernel_result.output,
            diagnostics=kernel_result.diagnostics,
            safety=kernel_result.safety,
            metadata={
                "altitude": "bridge",
                "glyph": "🧷",
                "source_expressive": packet.expressive_metadata,
                "mode": "kernel-bridge"
            }
        )


# ────────────────────────────────────────────────────────────────
#  Provenance Footer — Kernel Bridge (🧷)
# ────────────────────────────────────────────────────────────────
# Artifact: bridge_kernel.py
# Altitude: 🟫 Bridge
# Glyph: 🧷
# Roadmap: vectorium_roadmap_v2.json (artifact: bridge-kernel)
# License: Stell Non‑Commercial
# Description:
#     Implements the kernel-facing expressive bridge for Anima-Vectorium.
#     Accepts expressive → geometric projections and prepares them for kernel
#     execution while maintaining strict altitude boundaries and read-only
#     expressive semantics.
# ────────────────────────────────────────────────────────────────
