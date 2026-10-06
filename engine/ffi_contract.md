# Vectorium v5.0 — FFI Contract

This file defines the mandatory cross-language consistency rules for the
Vectorium engine. Copilot must consult this file before generating any FFI code.

These rules apply to:
- src/ffi/*
- src/runtime/*
- src/thermodynamics/*
- src/spectral/*
- Python bindings
- any future language bindings

The FFI boundary must preserve all geometric, thermodynamic, and operator
invariants defined in the v5.0 math backbone.

---

## 1. Curvature Consistency

Rust and Python must compute identical curvature:

κ(s_rust) == κ(s_python)

Where:
κ(x) = ∇²x + Φ(x)

Φ(x) is activation-induced curvature.

Curvature must never drift across the boundary.

---

## 2. Holonomy Consistency

Holonomy transport must match exactly:

H(s_rust) == H(s_python)

Holonomy is defined as:
H(x) = x + ∮ κ · dγ

Holonomy invariants must be preserved across:
- update steps
- residual mixing
- FFI calls

---

## 3. Free-Energy Consistency

Free-energy must be identical across languages:

F(s_rust) == F(s_python)

Where:
F(s) = A(s) + C(s) + S(s)

This ensures thermodynamic correctness.

---

## 4. Operator Ordering Consistency

The strict operator ordering must be preserved across the boundary:

Activation  
→ Normalization  
→ Drive Arbitration  
→ Residual Mixing  
→ Curvature Update  
→ Holonomy Transport

No FFI call may reorder operators.

---

## 5. State Evolution Consistency

State evolution must satisfy:

- curvature bound  
- holonomy bound  
- dissipation  
- Jacobian stability  
- fossilization rules  
- ordering constraints  

These must hold identically in Rust and Python.

---

## 6. Fossilization Enforcement

The following constructs must never cross the FFI boundary:

- spectral curvature operators  
- narrative gravity fields  
- collapse-fractal recursion  
- continuous spectral manifolds  

These constructs are permanently excluded.

---

## 7. Serialization Requirements

State serialization must preserve:
- tensor values  
- metadata  
- curvature fields  
- holonomy fields  
- free-energy values  

No lossy serialization is permitted.

---

## 8. Error Semantics

FFI errors must reflect invariant violations clearly:
- curvature blowout  
- holonomy drift  
- dissipation failure  
- Jacobian instability  
- ordering violation  
- fossilized construct usage  

Errors must be symmetric across languages.

---

This file is the authoritative FFI contract for Vectorium v5.0.
