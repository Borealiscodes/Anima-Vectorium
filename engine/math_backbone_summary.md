# Vectorium v5.0 — Developer Summary

This file summarizes the canonical mathematical backbone defined in:
preprints/anima_vectorium_v5.0_mathématique/anima_vectorium_v5.0_mathématique.tex

Copilot must consult this summary before generating code.

---

## 1. Operator Ordering (strict)

Activation  
→ Normalization  
→ Drive Arbitration  
→ Residual Mixing  
→ Curvature Update  
→ Holonomy Transport

This ordering must be preserved in all runtime logic, Rust modules, and FFI calls.

---

## 2. Curvature

Curvature is defined as:
κ(x) = ∇²x + Φ(x)

Metric:
g(x) = I + λ Φ(x)

Φ(x) is activation-induced curvature (GELU, SwiGLU, Softplus, Tanh).

---

## 3. Holonomy

Holonomy transport:
H(x) = x + ∮ κ · dγ

Parallel transport uses the Levi-Civita connection.

Holonomy invariants must be preserved across all update steps and FFI boundaries.

---

## 4. Free-Energy Functional

F(s) = A(s) + C(s) + S(s)

Accuracy:
A(s) = ||x - x̂(s)||²

Complexity:
C(s) = λ ||s||²

Surprise:
S(s) = -log p(x | s)

Dissipation:
F(s_{t+1}) ≤ F(s_t)

---

## 5. Update Rule

s_{t+1} = ResidualMix(
    s_t,
    H(s_t - η * SelectedDrive(s_t))
)

Jacobian stability:
||J|| ≤ 1

---

## 6. Spectral Translation

Spectral drift → ∇F(s)  
Spectral homotopy → holonomy  
Spectral resonance → drive arbitration  

Fossilized constructs (never implement):
- spectral curvature operators  
- narrative gravity fields  
- collapse-fractal recursion  
- continuous spectral manifolds  

---

## 7. FFI Consistency

Rust and Python must preserve:

κ(s_rust) == κ(s_python)  
H(s_rust) == H(s_python)  
F(s_rust) == F(s_python)  

Operator ordering must be identical across the boundary.

---

## 8. Runtime Invariants

- curvature bound: ||κ(x)|| ≤ K_max  
- holonomy bound: |∮ κ · dγ| ≤ H_max  
- dissipation: F(s_{t+1}) ≤ F(s_t)  
- Jacobian stability: ||J|| ≤ 1  

These invariants must be enforced in runtime, thermodynamics, and update rule modules.

---

This summary is the authoritative developer-facing reference for Vectorium v5.0.
