# Copilot Map for Vectorium v5.0

This repository implements the mathematical backbone defined in:
preprints/anima_vectorium_v5.0_mathématique/anima_vectorium_v5.0_mathématique.tex

Copilot must follow these rules:

## 1. Operator Ordering (strict)
Activation → Normalization → Drive Arbitration → Residual Mixing → Curvature Update → Holonomy Transport

## 2. Invariants (must be preserved)
- Curvature bound: ||κ(x)|| ≤ K_max
- Holonomy bound: |∮ κ · dγ| ≤ H_max
- Free-energy dissipation: F(s_{t+1}) ≤ F(s_t)
- Jacobian stability: ||J|| ≤ 1
- FFI consistency: κ, H, F identical across Rust/Python

## 3. Fossilized Constructs (never implement)
- spectral curvature operators
- narrative gravity fields
- collapse-fractal recursion
- continuous spectral manifolds

## 4. Module Responsibilities
- src/thermodynamics: F(s), gradients, dissipation
- src/spectral: translation + fossilization rules
- src/runtime: update rule + state evolution
- src/ffi: cross-language invariants
- src/membrane: boundary conditions
- src/haptics: interface layer
- src/utils: errors, helpers

Copilot must consult the math backbone before generating code.
