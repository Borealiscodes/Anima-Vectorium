# Vectorium v5.0 — TEST FAILURE TRACKER

Use this file to report test failures.

## Format

For each test failure:

```
❌ TEST FAILURE: [test_name]
Status: [PANIC | ASSERTION | TIMEOUT]
Location: tests/invariants.rs::[line]

Error Message:
[full assertion or panic message]

Expected Behavior:
[what the invariant should enforce]

Actual Behavior:
[what the test observed]

Test Code:
[relevant test function]
```

## Examples

### Example 1: Assertion Failure

```
❌ TEST FAILURE: curvature_bound_is_respected
Status: ASSERTION
Location: tests/invariants.rs::12

Error Message:
assertion failed: curvature_bound(&curvature)

Expected Behavior:
The laplacian of any 9-vector should satisfy ||κ(x)|| ≤ K_max = 1.0

Actual Behavior:
The computed curvature has values > 1.0

Test Code:
#[test]
fn curvature_bound_is_respected() {
    let sample = [0.5f32; 9];
    let curvature = laplacian(&sample);
    assert!(curvature_bound(&curvature), "Curvature bound violated");
}
```

### Example 2: Panic

```
❌ TEST FAILURE: free_energy_dissipates
Status: PANIC
Location: tests/invariants.rs::24

Error Message:
thread 'free_energy_dissipates' panicked at 'called `Option::unwrap()` on a `None` value'

Expected Behavior:
The update rule should compute F(s_{t+1}) and verify F(s_{t+1}) ≤ F(s_t)

Actual Behavior:
The free energy computation is returning None or panicking

Test Code:
#[test]
fn free_energy_dissipates() {
    let initial = RuntimeState::new();
    let next = update(&initial);
    assert!(next.free_energy_current <= initial.free_energy_current + 1e-5);
}
```

---

## Instructions

1. Run `cargo test --test invariants -- --nocapture 2>&1`
2. Copy full failure output here
3. Categorize by test name
4. Include the full test assertion
5. Report to Copilot with this format

---
