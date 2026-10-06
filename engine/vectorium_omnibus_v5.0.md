# 📚 VECTORIUM v5.0 — DEVELOPER OMNIBUS

Unified Copilot Initialization + Architecture Codex (v5.0)

This Omnibus is the single authoritative reference for GitHub Copilot when generating or modifying any part of the Vectorium v5.0 engine.

Copilot must treat this document as the root of truth.

---

SECTION 1 — INITIALIZATION LAYER

1.1 VS Code Snippet (Copilot Activation)
File: .vscode/snippets/vectorium.code-snippets

`json
{
  "Vectorium v5.0 Copilot Training Prompt": {
    "prefix": "vectorium-init",
    "body": [
      "You are generating code for the Vectorium v5.0 engine.",
      "",
      "Before writing any code, consult:",
      "- engine/vectorium_scaffolding.md",
      "- engine/vectorium_manifest.json",
      "- engine/mathbackbonesummary.md",
      "- engine/runtime_invariants.md",
      "- src/DONOTIMPLEMENT.md",
      "- src/COPILOT_MAP.md",
      "",
      "Follow these rules:",
      "",
      "1. Preserve all invariants:",
      "   - curvature_bound",
      "   - holonomy_bound",
      "   - dissipation",
      "   - jacobian_stability",
      "   - operator_ordering",
      "   - ffi_consistency",
      "",
      "2. Respect operator ordering:",
      "   Activation → Normalization → Drive Arbitration → Residual Mixing → Curvature Update → Holonomy Transport",
      "",
      "3. Implement only geometric + thermodynamic constructs.",
      "   Do NOT implement fossilized constructs:",
      "   - spectral curvature",
      "   - narrative gravity",
      "   - collapse-fractal recursion",
      "   - continuousspectralmanifolds",
      "",
      "4. Align code with module scaffolding:",
      "   - curvature/",
      "   - holonomy/",
      "   - operators/",
      "   - thermodynamics/",
      "   - runtime/state",
      "   - runtime/update_rule",
      "   - ffi/",
      "",
      "5. Use the JSON manifest as the module map and dependency graph.",
      "",
      "6. All code must satisfy the test harness in tests/invariants.rs.",
      "",
      "Generate code that is mathematically aligned with the v5.0 backbone and structurally aligned with the scaffolding."
    ],
    "description": "Injects the Vectorium v5.0 Copilot training prompt."
  }
}
`

---

1.2 Copilot Chat Preset
Paste this at the start of every Copilot Chat session:

`
Load Vectorium v5.0 mode.

Use:
- vectorium_scaffolding.md
- vectorium_manifest.json
- runtime_invariants.md
- DONOTIMPLEMENT.md
- COPILOT_MAP.md

All code must satisfy:
- curvature_bound
- holonomy_bound
- dissipation
- jacobian_stability
- operator_ordering
- ffi_consistency

Never generate fossilized spectral constructs.
Follow operator ordering strictly.
Align all modules with the JSON manifest.
`

---

1.3 Copilot CLI Preset
File: engine/vectoriumclipreset.txt

`
--context engine/vectorium_scaffolding.md
--context engine/vectorium_manifest.json
--context engine/runtime_invariants.md
--context src/DONOTIMPLEMENT.md
--context src/COPILOT_MAP.md
--require-invariants
--no-fossilized-constructs
--respect-operator-ordering
--ffi-consistency
`

---

SECTION 2 — ARCHITECTURE LAYER

2.1 Module Dependency Graph (Canonical Map)

`
curvature
  └── laplacian
  └── activation_curvature

holonomy
  └── transport
      └── depends on curvature

operators
  └── activation
  └── normalization
  └── residual
      └── depends on curvature + holonomy

thermodynamics
  └── free_energy
  └── gradients
  └── dissipation
      └── depends on operators

runtime
  └── state
  └── update_rule
      └── depends on:
          - operators
          - holonomy
          - thermodynamics

ffi
  └── depends on:
      - curvature
      - holonomy
      - thermodynamics
      - runtime/state
`

---

2.2 Machine‑Readable JSON Manifest (Single Source of Truth)
File: engine/vectorium_manifest.json

`json
{
  "vectorium_version": "5.0",
  "modules": {
    "curvature": {
      "files": [
        "src/curvature/mod.rs",
        "src/curvature/laplacian.rs",
        "src/curvature/activation_curvature.rs"
      ],
      "invariants": ["curvature_bound"]
    },
    "holonomy": {
      "files": [
        "src/holonomy/mod.rs",
        "src/holonomy/transport.rs"
      ],
      "invariants": ["holonomy_bound"]
    },
    "operators": {
      "files": [
        "src/operators/mod.rs",
        "src/operators/activation.rs",
        "src/operators/normalization.rs",
        "src/operators/residual.rs"
      ],
      "ordering": true
    },
    "thermodynamics": {
      "files": [
        "src/thermodynamics/mod.rs",
        "src/thermodynamics/free_energy.rs",
        "src/thermodynamics/gradients.rs",
        "src/thermodynamics/dissipation.rs"
      ],
      "invariants": ["dissipation"]
    },
    "runtime": {
      "files": [
        "src/runtime/mod.rs",
        "src/runtime/state.rs",
        "src/runtime/update_rule.rs"
      ],
      "invariants": ["jacobianstability", "operatorordering"]
    },
    "ffi": {
      "files": ["src/ffi/mod.rs"],
      "invariants": ["ffi_consistency"]
    }
  },
  "global_invariants": [
    "curvature_bound",
    "holonomy_bound",
    "dissipation",
    "jacobian_stability",
    "operator_ordering",
    "ffi_consistency"
  ],
  "fossilized_constructs": [
    "spectral_curvature",
    "narrative_gravity",
    "collapse_fractal",
    "continuousspectralmanifold"
  ]
}
`

---

2.3 Scaffolding Summary (Not Full Code)

`
curvature/: laplacian, activation_curvature
holonomy/: transport
operators/: activation, normalization, residual
thermodynamics/: free_energy, gradients, dissipation
runtime/: state, update_rule
ffi/: boundary consistency
`

This summary prevents duplication while giving Copilot the structural map.

---

SECTION 3 — GOVERNANCE LAYER

3.1 Invariants (Canonical)

- curvature_bound  
- holonomy_bound  
- dissipation  
- jacobian_stability  
- operator_ordering  
- ffi_consistency

---

3.2 Fossilization Rules (Permanent Exclusions)

Never implement:

- spectral curvature  
- narrative gravity  
- collapse‑fractal recursion  
- continuous spectral manifolds  

---

3.3 Operator Ordering (Strict)

Activation  
→ Normalization  
→ Drive Arbitration  
→ Residual Mixing  
→ Curvature Update  
→ Holonomy Transport

---

3.4 FFI Contract (Boundary Consistency)

Rust and Python must produce identical:

- κ(s)  
- H(s)  
- F(s)  

Serialization must be lossless.

---

3.5 Vectorium Coding Style Guide

- reflect v5.0 math backbone  
- enforce invariants  
- preserve operator ordering  
- avoid fossilized constructs  
- clear error semantics  
- maintain FFI symmetry  

---

SECTION 4 — PROVENANCE FOOTER

`
---

Provenance

The Vectorium v5.0 Developer Omnibus consolidates all architectural, governance,
and initialization artifacts into a single authoritative reference for GitHub
Copilot. It unifies scaffolding, JSON manifest, invariants, fossilization rules,
operator-ordering constraints, Copilot presets, CLI presets, dependency graph,
and coding style guide.

Authored by Borealis S. Hedling during the Vectorium v5.0 engine construction
cycle. Serves as the canonical source of truth for Copilot-aligned code
generation across the Vectorium runtime.
`

---

