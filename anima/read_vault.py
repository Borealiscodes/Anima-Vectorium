# ────────────────────────────────────────────────────────────────
#  Anima‑Vectorium — Expressive Altitude
#  Artifact: Read‑Only Vault (🔒)
#  Path: anima/read_vault.py
#  License: Stell Non‑Commercial (Expressive)
#  Altitude: 🟥 Expressive
# ────────────────────────────────────────────────────────────────

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict, List
from anima.persistence import PersistenceRecord


@dataclass
class VaultEntry:
    """Represents an immutable view of a stored expressive record."""
    index: int
    value: Any
    metadata: Dict[str, Any]


class ReadVault:
    """
    🔒 Read‑Only Vault (Expressive Altitude)
    Provides immutable access to expressive persistence records.

    Responsibilities:
    - Expose expressive history without mutation
    - Provide altitude‑aware read‑only views
    - Support expressive replay and provenance inspection
    - Serve as the safe expressive memory surface for external tools
    """

    def __init__(self, persistence: Any):
        """
        Initialize the read‑vault with a persistence substrate.
        """
        self.persistence = persistence

    def list(self) -> List[VaultEntry]:
        """
        Return all expressive records as immutable vault entries.

        Returns
        -------
        List[VaultEntry]
        """
        entries: List[VaultEntry] = []
        for record in self.persistence.snapshot():
            entries.append(
                VaultEntry(
                    index=record.index,
                    value=record.state.value,
                    metadata={
                        "altitude": "expressive",
                        "glyph": "🔒",
                        "source": record.metadata,
                    }
                )
            )
        return entries

    def get(self, index: int) -> VaultEntry:
        """
        Retrieve a single expressive record as an immutable vault entry.

        Parameters
        ----------
        index : int

        Returns
        -------
        VaultEntry

        Raises
        ------
        IndexError
            If the index is invalid.
        """
        record: PersistenceRecord = self.persistence.get(index)
        return VaultEntry(
            index=record.index,
            value=record.state.value,
            metadata={
                "altitude": "expressive",
                "glyph": "🔒",
                "source": record.metadata,
            }
        )


# ────────────────────────────────────────────────────────────────
#  Provenance Footer — Read‑Only Vault (🔒)
# ────────────────────────────────────────────────────────────────
# Artifact: read_vault.py
# Altitude: 🟥 Expressive
# Glyph: 🔒
# Roadmap: vectorium_roadmap_v2.json (artifact: expressive-read-vault)
# License: Stell Non‑Commercial
# Description:
#     Implements the read‑only expressive vault for Anima-Vectorium. Provides
#     immutable access to expressive persistence records, enabling expressive
#     replay, provenance inspection, and safe external read surfaces.
# ────────────────────────────────────────────────────────────────
