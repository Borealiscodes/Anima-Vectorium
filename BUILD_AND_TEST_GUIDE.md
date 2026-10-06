# Vectorium v5.0 — BUILD & TEST VERIFICATION KIT

This guide walks you through the full build and test cycle for v5.0 compliance.

## STEP 1: Clean Build

```bash
cd /path/to/Anima-Vectorium
cargo clean
cargo build --release 2>&1 | tee build.log
```

**Expected:** Zero errors. Any compilation errors should be reported.

---

## STEP 2: Run Invariant Tests

```bash
cargo test --test invariants -- --nocapture 2>&1 | tee test.log
```

**Expected:**
- ✅ curvature_bound_is_respected
- ✅ holonomy_bound_is_respected
- ✅ free_energy_dissipates
- ✅ jacobian_stability_is_preserved
- ✅ ffi_consistency_is_preserved
- ✅ operator_ordering_enforced (new)

---

## STEP 3: Run All Tests

```bash
cargo test --release 2>&1 | tee full-test.log
```

**Expected:** All tests pass.

---

## STEP 4: Lint for Fossilized Constructs

```bash
grep -r "spectral_curvature\|narrative_gravity\|collapse.fractal\|continuous.*spectral" src/ && echo "FAIL: Fossilized constructs found" || echo "PASS: Fossilization firewall clean"
```

**Expected:** PASS

---

## STEP 5: Report Format

If any tests fail, report:

```
❌ TEST FAILURE: [test_name]
Error:
[full error message from cargo test]

Expected behavior:
[what the test should do]

Actual behavior:
[what it's doing]
```

---

## Compilation Errors vs. Test Failures

**Compilation errors** block the build entirely.  
**Test failures** allow build but fail validation.

Report each separately so we can fix in order.

---

## File Sync Check

Verify these critical files exist and are in sync:

```bash
ls -la src/runtime/update_rule.rs
ls -la src/runtime/state.rs
ls -la src/thermodynamics/free_energy.rs
ls -la src/thermodynamics/gradients.rs
ls -la src/thermodynamics/dissipation.rs
ls -la src/operators/drive_arbitration.rs
ls -la tests/invariants.rs
```

**All must exist and have content.**

---

## Quick Validation

Run this to identify immediate issues:

```bash
echo "=== CHECKING STRUCTURE ==="
echo "✓ Checking Cargo.toml..."
grep -q 'name = "vectorium"' Cargo.toml && echo "  PASS" || echo "  FAIL"

echo "✓ Checking lib.rs exports..."
grep -q 'pub mod runtime' src/lib.rs && echo "  PASS" || echo "  FAIL"

echo "✓ Checking runtime modules..."
for f in state.rs update_rule.rs tick.rs handle.rs; do
  test -f src/runtime/$f && echo "  $f: PASS" || echo "  $f: FAIL"
done

echo "✓ Checking thermodynamics modules..."
for f in free_energy.rs gradients.rs dissipation.rs; do
  test -f src/thermodynamics/$f && echo "  $f: PASS" || echo "  $f: FAIL"
done

echo "✓ Checking operators drive_arbitration..."
test -f src/operators/drive_arbitration.rs && echo "  PASS" || echo "  FAIL: MISSING"

echo "=== END STRUCTURE CHECK ==="
```

---

## Next Steps

1. Run: `cargo build --release`
2. Report any compilation errors
3. We fix them
4. Run: `cargo test --test invariants`
5. Report any test failures
6. We fix them until all pass

---
