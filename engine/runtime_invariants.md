# Vectorium v5.0 — Runtime Invariants

This file defines the core invariants that all Vectorium runtime logic must
preserve. Copilot must consult this file before generating code.

These invariants apply to:
- src/runtime/*
- src/thermodynamics/*
- src/spectral/*
- src/ffi/*
- src/membrane/*
- src/haptics/*
- src/utils/*
- tests/invariants.rs

They are the backbone of safe execution.

---

## 1. Curvature Bound

Curvature must satisfy:
||κ(x)|| ≤ K_max

Where:
κ(x) = ∇²x + Φ(x)

Φ(x) is activation-induced curvature.

This prevents curvature blowout.

---

## 2. Holonomy Bound

Holonomy transport must satisfy:
|∮ κ · dγ| ≤ H_max

Holonomy must remain stable across:
- update steps
- residual mixing
- FFI boundaries

---

## 3. Free-Energy Dissipation

The free-energy functional must decrease or remain constant:

F(s_{t+1}) ≤ F(s_t)

Where:
F(s) = A(s) + C(s) + S(s)

This ensures thermodynamic stability.

---

## 4. Jacobian Stability

The Jacobian of the update rule must satisfy:
||J|| ≤ 1

This prevents geometric instability and runaway dynamics.

---

## 5. Operator Ordering

All runtime logic must preserve the strict operator ordering:

Activation  
→ Normalization  
→ Drive Arbitration  
→ Residual Mixing  
→ Curvature Update  
→ Holonomy Transport

This ordering is mandatory.

---

## 6. FFI Consistency

Rust and Python must preserve:

κ(s_rust) == κ(s_python)  
H(s_rust) == H(s_python)  
F(s_rust) == F(s_python)

Operator ordering must be identical across the boundary.

---

## 7. Spectral Fossilization Rules

The following constructs must never be implemented:

- spectral curvature operators  
- narrative gravity fields  
- collapse-fractal recursion  
- continuous spectral manifolds  

These constructs are permanently excluded.

---

## 8. State Evolution Safety

All state evolution must satisfy:

- curvature bound  
- holonomy bound  
- dissipation  
- Jacobian stability  
- operator ordering  
- FFI consistency  

These are mandatory for safe execution.

---

This file is the authoritative reference for runtime invariants in Vectorium v5.0.
