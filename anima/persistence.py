# ────────────────────────────────────────────────────────────────
#  Anima‑Vectorium — Expressive Altitude
#  Artifact: Expressive Persistence Layer (🗄️)
#  Path: anima/persistence.py
#  License: Stell Non‑Commercial (Expressive)
#  Altitude: 🟥 Expressive
# ────────────────────────────────────────────────────────────────

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict, List
from anima.anima_core import AnimaState


@dataclass
class PersistenceRecord:
    """Represents a stored expressive state with lineage metadata."""
    state: AnimaState
    index: int
    metadata: Dict[str, Any]


class Persistence:
    """
    🗄️ Expressive Persistence Layer (Expressive Altitude)
    Stores expressive states and maintains lineage for Anima.

    Responsibilities:
    - Persist expressive states
    - Provide indexed retrieval
    - Maintain expressive lineage (ancestry + transformations)
    - Support snapshotting for read-vault
    """

    def __init__(self):
        """Initialize the persistence substrate."""
        self.records: List[PersistenceRecord] = []
        self.counter: int = 0

    def store(self, state: AnimaState) -> PersistenceRecord:
        """
        Store an expressive state.

        Parameters
        ----------
        state : AnimaState

        Returns
        -------
        PersistenceRecord
        """
        record = PersistenceRecord(
            state=state,
            index=self.counter,
            metadata={
                "altitude": "expressive",
                "glyph": "🗄️",
                "stored_from": state.metadata,
            }
        )
        self.records.append(record)
        self.counter += 1
        return record

    def get(self, index: int) -> PersistenceRecord:
        """
        Retrieve a stored expressive state by index.

        Parameters
        ----------
        index : int

        Returns
        -------
        PersistenceRecord

        Raises
        ------
        IndexError
            If the index is out of range.
        """
        if index < 0 or index >= len(self.records):
            raise IndexError(f"Invalid persistence index: {index}")
        return self.records[index]

    def snapshot(self) -> List[PersistenceRecord]:
        """
        Return a full snapshot of all stored expressive states.

        Returns
        -------
        List[PersistenceRecord]
        """
        return list(self.records)


# ────────────────────────────────────────────────────────────────
#  Provenance Footer — Expressive Persistence Layer (🗄️)
# ────────────────────────────────────────────────────────────────
# Artifact: persistence.py
# Altitude: 🟥 Expressive
# Glyph: 🗄️
# Roadmap: vectorium_roadmap_v2.json (artifact: expressive-persistence)
# License: Stell Non‑Commercial
# Description:
#     Implements the expressive persistence substrate for Anima-Vectorium.
#     Stores expressive states, maintains lineage, and provides snapshotting
#     functionality used by the read-vault and expressive-geometric replay.
# ────────────────────────────────────────────────────────────────
