# ────────────────────────────────────────────────────────────────
#  Anima‑Vectorium — Examples Altitude
#  Artifact: API Usage Demo (📘)
#  Path: examples/vectorium_api.py
#  License: Dual — GVL‑1.1 + Stell Non‑Commercial
#  Altitude: Examples
# ────────────────────────────────────────────────────────────────

from anima.anima_core import AnimaCore
from anima.persistence import Persistence
from anima.read_vault import ReadVault
from bridge.bridge_readonly import ReadOnlyBridge
from bridge.bridge_kernel import KernelBridge
from vectorium.kernel import Kernel


def main():
    print("\n=== Anima‑Vectorium API Demo ===\n")

    # Initialize expressive altitude
    anima = AnimaCore()
    persistence = Persistence()
    vault = ReadVault(persistence)

    # Create expressive states
    s1 = anima.create("hello‑vectorium")
    s2 = anima.create({"msg": "expressive‑geometry"})

    # Store expressive states
    r1 = persistence.store(s1)
    r2 = persistence.store(s2)

    print("Stored expressive states:")
    print(f"  • index {r1.index}: {r1.state.value}")
    print(f"  • index {r2.index}: {r2.state.value}\n")

    # Initialize bridges
    readonly_bridge = ReadOnlyBridge(anima, vault)
    kernel = Kernel()
    kernel_bridge = KernelBridge(kernel)

    # Project expressive → geometric
    packet = readonly_bridge.project(0)
    print("Expressive → Geometric Projection:")
    print(f"  expressive: {packet.expressive_value}")
    print(f"  geometric:  {packet.geometric_projection}\n")

    # Run kernel execution using expressive projection
    print("Kernel Execution:")
    result = kernel_bridge.execute(packet, a={"x": 1}, b={"x": 2})

    print(f"  allowed:     {result.allowed}")
    print(f"  output:      {result.output}")
    print(f"  diagnostics: {result.diagnostics}")
    print(f"  safety:      {result.safety}\n")

    print("=== Demo Complete ===\n")


if __name__ == "__main__":
    main()


# ────────────────────────────────────────────────────────────────
#  Provenance Footer — API Usage Demo (📘)
# ────────────────────────────────────────────────────────────────
# Artifact: vectorium_api.py
# Altitude: Examples
# Glyph: 📘
# Roadmap: vectorium_roadmap_v2.json (artifact: examples-api-demo)
# License: Dual — GVL‑1.1 + Stell Non‑Commercial
# Description:
#     Demonstrates expressive state creation, persistence, read‑vault access,
#     expressive → geometric projection, and kernel execution using the bridge
#     altitude. Provides a runnable example of the full expressive‑geometric
#     pipeline.
# ────────────────────────────────────────────────────────────────
