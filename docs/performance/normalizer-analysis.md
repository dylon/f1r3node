# Normalizer O(n²) Pattern Analysis

**Date**: 2025-11-06  
**Author**: Claude (Scientific Investigation)  
**Goal**: Identify O(n²) patterns in other normalizers after fixing Par normalization

## Summary

After achieving a 6,158x speedup for Par normalization, investigated other normalizers for similar O(n²) issues.

**Result**: ✅ Most normalizers are already optimal, ✅ **Found and fixed 1 issue in Match normalizer**

---

## Investigated Normalizers

### 1. ✅ **Send Normalizer** (p_send_normalizer.rs)

**Pattern**: Accumulates send data using `Vec::push()`

```rust
let mut acc = (Vec::new(), ...);
for proc in inputs.iter() {
    acc.0.push(proc_match_result.par.clone());  // O(1) amortized
}
```

**Complexity**: O(n) ✅  
**Status**: No optimization needed

---

### 2. ✅ **Input/Receive Normalizer** (p_input_normalizer.rs)

**Pattern**: Accumulates sources and patterns using `Vec::push()`

```rust
for name in sources {
    vector_par.push(par.clone());  // O(1) amortized
}
```

**Complexity**: O(n) ✅  
**Status**: No optimization needed

---

### 3. ✅ **Match Normalizer** (p_match_normalizer.rs) - FIXED!

**Original Pattern**: Used `Vec::insert(0, ...)` to prepend cases

```rust
// Line 67-74 (before fix)
for case in cases {
    ...
    init_acc.0.insert(0, MatchCase { ... });  // O(k) where k grows
    ...
}
cases: init_acc.0.into_iter().rev().collect(),  // Double reversal!
```

**Problem**:
- `.insert(0, item)` shifts all existing elements → O(k) per iteration
- Loop runs n times with growing k → O(n²) total complexity
- Was reversing twice: insert(0) builds reversed, then .rev() reverses again

**Complexity Before**: O(n²) ❌

**Fix Applied**: Use accumulator pattern (same as Par normalization)
```rust
// Optimized (O(n)):
let mut match_cases = Vec::new();
for case in cases {
    match_cases.push(MatchCase { ... });  // O(1) amortized
}
// No reverse needed - push maintains original order!
```

**Complexity After**: O(n) ✅

**Status**: Fixed and tested - all 8 tests pass

---

## Other Normalizers Checked

Searched for `prepend_` usage across all normalizers:
- ✅ **New normalizer**: Uses `prepend_new` (single call per normalization)
- ✅ **Bundle normalizer**: Uses `prepend_bundle` (single call per normalization)  
- ✅ **Conjunction/Disjunction/Negation**: Use `prepend_connective` (single call per normalization)
- ✅ **Eval/Method/Contr**: Use various prepends (single call per normalization)

**All other normalizers**: ✅ No loops with prepend operations

---

## Implementation Summary

**Match Normalizer Fix**:
- ✅ Applied same accumulator pattern as Par normalization
- ✅ Changed from `init_acc.0.insert(0, ...)` to `match_cases.push(...)`
- ✅ Removed unnecessary double reversal (.insert(0) + .rev())
- ✅ All 8 tests pass
- ✅ Complexity improved from O(n²) to O(n)

**Time to implement**: ~15 minutes

**Real-world impact**:
- **Likely Low**: Match statements rarely have >100 cases in practice
- **But**: Zero-cost fix maintaining codebase consistency
- **Benefit**: Prevents pathological performance on edge cases

---

## Next Steps

1. ✅ Document findings
2. ✅ Implement Match normalizer fix
3. ✅ Validate with tests (all pass)
4. ⏭️ Optional: Benchmark if corpus contains large match statements
5. ⏭️ Continue with other optimization opportunities (see par-normalization-optimization.md)

