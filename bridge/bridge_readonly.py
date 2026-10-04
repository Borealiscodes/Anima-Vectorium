# ────────────────────────────────────────────────────────────────
#  Anima‑Vectorium — Bridge Altitude
#  Artifact: Read‑Only Expressive Bridge (🪢)
#  Path: bridge/bridge_readonly.py
#  License: Stell Non‑Commercial (Expressive)
#  Altitude: 🟫 Bridge
# ────────────────────────────────────────────────────────────────

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict, List
from anima.read_vault import VaultEntry, ReadVault
from anima.anima_core import AnimaCore, AnimaState


@dataclass
class BridgePacket:
    """Represents a read-only expressive packet exposed to geometric altitude."""
    expressive_value: Any
    expressive_metadata: Dict[str, Any]
    geometric_projection: Any
    metadata: Dict[str, Any]


class ReadOnlyBridge:
    """
    🪢 Read‑Only Expressive Bridge (Bridge Altitude)
    Provides a safe, immutable interface between expressive altitude and
    geometric/infrastructure altitudes.

    Responsibilities:
    - Expose expressive states without mutation
    - Convert expressive states into geometric projections
    - Provide altitude-aware read surfaces for kernel + simulator
    - Maintain strict one-way expressive → geometric flow
    """

    def __init__(self, anima: AnimaCore, vault: ReadVault):
        """
        Initialize the read-only bridge with expressive core + vault.
        """
        self.anima = anima
        self.vault = vault

    def project(self, index: int) -> BridgePacket:
        """
        Project an expressive state into geometric-compatible form.

        Parameters
        ----------
        index : int
            Index of the expressive state in the read-vault.

        Returns
        -------
        BridgePacket
        """
        entry: VaultEntry = self.vault.get(index)

        # Convert expressive → geometric using AnimaCore
        geometric_projection = self.anima.to_geometric(
            AnimaState(
                value=entry.value,
                metadata=entry.metadata
            )
        )

        return BridgePacket(
            expressive_value=entry.value,
            expressive_metadata=entry.metadata,
            geometric_projection=geometric_projection,
            metadata={
                "altitude": "bridge",
                "glyph": "🪢",
                "source_index": index,
                "mode": "readonly"
            }
        )

    def list(self) -> List[BridgePacket]:
        """
        List all expressive states as geometric projections.

        Returns
        -------
        List[BridgePacket]
        """
        packets: List[BridgePacket] = []
        for entry in self.vault.list():
            geometric_projection = self.anima.to_geometric(
                AnimaState(
                    value=entry.value,
                    metadata=entry.metadata
                )
            )
            packets.append(
                BridgePacket(
                    expressive_value=entry.value,
                    expressive_metadata=entry.metadata,
                    geometric_projection=geometric_projection,
                    metadata={
                        "altitude": "bridge",
                        "glyph": "🪢",
                        "source_index": entry.index,
                        "mode": "readonly"
                    }
                )
            )
        return packets


# ────────────────────────────────────────────────────────────────
#  Provenance Footer — Read‑Only Expressive Bridge (🪢)
# ────────────────────────────────────────────────────────────────
# Artifact: bridge_readonly.py
# Altitude: 🟫 Bridge
# Glyph: 🪢
# Roadmap: vectorium_roadmap_v2.json (artifact: bridge-readonly)
# License: Stell Non‑Commercial
# Description:
#     Implements the read-only expressive bridge for Anima-Vectorium. Provides
#     immutable expressive → geometric projections used by the kernel, simulator,
#     and external integrations. Maintains strict altitude boundaries and
#     one-way expressive flow.
# ────────────────────────────────────────────────────────────────
