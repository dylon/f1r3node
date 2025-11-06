# sub_pars Lazy Iterator Design

**Date**: 2025-11-06
**Status**: Design Phase
**Goal**: Replace O(2^n) eager cartesian product with lazy iteration

---

## Problem Summary

Current `sub_pars` implementation generates ALL possible subsets and creates 7-way cartesian product eagerly:

```rust
let result = min_max_subsets(&par.sends, send_min, send_max)
    .into_iter()
    .cartesian_product(min_max_subsets(&par.receives, receive_min, receive_max))
    .cartesian_product(min_max_subsets(&par.news, new_min, new_max))
    .cartesian_product(min_max_subsets(&par.exprs, expr_min, expr_max))
    .cartesian_product(min_max_subsets(&par.matches, match_min, match_max))
    .cartesian_product(min_max_subsets(&par.unforgeables, unf_min, unf_max))
    .cartesian_product(min_max_subsets(&par.bundles, bundle_min, bundle_max))
```

**Problems**:
1. `min_max_subsets` returns `Vec<(Vec<A>, Vec<A>)>` - generates ALL subsets upfront
2. Nested `.cartesian_product()` calls create intermediate allocations
3. Each subset generation uses recursive algorithm with `.insert(0, ...)` (O(n) per call)
4. Total complexity: O(2^n × n × k^7) where n = avg element count, k = avg subsets per component

---

## Design Goals

1. **Lazy Generation**: Generate combinations one at a time, not all upfront
2. **Early Termination**: Stop generating when match found (if applicable)
3. **Memory Efficiency**: O(n) memory instead of O(2^n)
4. **Maintain Correctness**: Generate exactly the same results as current implementation
5. **Testable**: Easy to validate against current implementation

---

## Design Approach: Lazy Subset Iterator

### Phase 1: Replace `min_max_subsets` with Iterator

Instead of returning `Vec<(Vec<A>, Vec<A>)>`, return an iterator that yields pairs on demand.

#### Key Insight: Subset Enumeration via Binary Counting

For a list `[a, b, c]`, subsets can be enumerated using binary counting:
```
000 = []
001 = [c]
010 = [b]
011 = [b, c]
100 = [a]
101 = [a, c]
110 = [a, b]
111 = [a, b, c]
```

Each subset corresponds to a bitmask where bit i indicates whether element i is included.

#### Algorithm: Iterative Subset Generation

```rust
struct SubsetIterator<'a, T> {
    elements: &'a [T],
    min_size: usize,
    max_size: usize,
    current_mask: usize,
    max_mask: usize,
}

impl<'a, T: Clone> Iterator for SubsetIterator<'a, T> {
    type Item = (Vec<T>, Vec<T>);  // (subset, complement)

    fn next(&mut self) -> Option<Self::Item> {
        // Find next valid mask (popcount between min_size and max_size)
        while self.current_mask <= self.max_mask {
            let mask = self.current_mask;
            self.current_mask += 1;

            let popcount = mask.count_ones() as usize;
            if popcount >= self.min_size && popcount <= self.max_size {
                // Build subset and complement from mask
                let mut subset = Vec::with_capacity(popcount);
                let mut complement = Vec::with_capacity(self.elements.len() - popcount);

                for (i, elem) in self.elements.iter().enumerate() {
                    if mask & (1 << i) != 0 {
                        subset.push(elem.clone());
                    } else {
                        complement.push(elem.clone());
                    }
                }

                return Some((subset, complement));
            }
        }

        None
    }
}
```

**Benefits**:
- No recursion - iterative algorithm
- No `.insert(0, ...)` - builds vectors forward
- Generates one subset at a time
- Can short-circuit when match found

**Limitations**:
- Bitmask approach limited to ~64 elements (use `u64`)
- For larger inputs, need alternative approach (see Phase 2)

#### Optimization: Pre-calculate Valid Masks

Instead of checking popcount for every mask, pre-calculate masks with valid popcounts:

```rust
fn generate_valid_masks(n: usize, min_size: usize, max_size: usize) -> Vec<u64> {
    let mut masks = Vec::new();
    let max_mask = (1u64 << n) - 1;

    for mask in 0..=max_mask {
        let popcount = mask.count_ones() as usize;
        if popcount >= min_size && popcount <= max_size {
            masks.push(mask);
        }
    }

    masks
}

struct OptimizedSubsetIterator<'a, T> {
    elements: &'a [T],
    valid_masks: Vec<u64>,
    current_index: usize,
}
```

**Trade-off**: Uses O(2^n) space for masks, but masks are just `u64` values (8 bytes each), not full vectors.

For n=10: 1,024 masks × 8 bytes = 8 KB (acceptable)
For n=20: 1,048,576 masks × 8 bytes = 8 MB (still manageable for one-time cost)

---

### Phase 2: Lazy Cartesian Product

Current implementation chains 7 `.cartesian_product()` calls. Each creates intermediate tuples.

#### Approach: Custom 7-Tuple Cartesian Iterator

```rust
struct SubParsIterator<'a> {
    par: &'a Par,

    // Iterators for each component
    sends_iter: SubsetIterator<'a, Send>,
    receives_iter: SubsetIterator<'a, Receive>,
    news_iter: SubsetIterator<'a, New>,
    exprs_iter: SubsetIterator<'a, Expr>,
    matches_iter: SubsetIterator<'a, Match>,
    unforgeables_iter: SubsetIterator<'a, GPrivate>,
    bundles_iter: SubsetIterator<'a, Bundle>,

    // Current state for each component
    current_sends: Option<(Vec<Send>, Vec<Send>)>,
    current_receives: Option<(Vec<Receive>, Vec<Receive>)>,
    current_news: Option<(Vec<New>, Vec<New>)>,
    current_exprs: Option<(Vec<Expr>, Vec<Expr>)>,
    current_matches: Option<(Vec<Match>, Vec<Match>)>,
    current_unforgeables: Option<(Vec<GPrivate>, Vec<GPrivate>)>,
    current_bundles: Option<(Vec<Bundle>, Vec<Bundle>)>,
}

impl<'a> Iterator for SubParsIterator<'a> {
    type Item = (Par, Par);

    fn next(&mut self) -> Option<Self::Item> {
        // Odometer-style iteration:
        // Increment rightmost iterator first, carry to left when exhausted

        // Try to advance bundles
        if let Some(bundles) = self.bundles_iter.next() {
            self.current_bundles = Some(bundles);
            return Some(self.build_par_pair());
        }

        // Bundles exhausted, reset and advance unforgeables
        self.bundles_iter = self.create_bundles_iter();
        self.current_bundles = self.bundles_iter.next();

        if let Some(unforgeables) = self.unforgeables_iter.next() {
            self.current_unforgeables = Some(unforgeables);
            return Some(self.build_par_pair());
        }

        // Continue carrying left through all components...
        // (Full implementation would handle all 7 components)

        None
    }
}
```

**Benefits**:
- Generates one (Par, Par) pair at a time
- No intermediate Vec allocations
- Can short-circuit when consumer stops iterating
- Memory: O(n) for current state, not O(2^n) for all combinations

**Implementation Note**: This is conceptually similar to a 7-dimensional odometer, where each "digit" (component iterator) advances independently.

---

### Phase 3: Constraint Propagation Optimization

Further optimize by using min/max constraints to skip impossible branches:

```rust
impl<'a> SubParsIterator<'a> {
    fn should_continue(&self) -> bool {
        // Check if current partial combination can lead to valid result
        let current_total = self.count_current_elements();
        let remaining_min = self.calculate_remaining_min();
        let remaining_max = self.calculate_remaining_max();

        // If current total + remaining_max < global_min, prune
        // If current total + remaining_min > global_max, prune

        current_total + remaining_max >= self.global_min &&
        current_total + remaining_min <= self.global_max
    }
}
```

This allows early termination of impossible branches without generating all combinations.

---

## Implementation Plan

### Step 1: Implement `SubsetIterator` (2-3 days)

1. Write iterative bitmask-based subset iterator
2. Add min_size/max_size filtering
3. Unit tests comparing against current `min_max_subsets`
4. Property tests: verify all subsets generated exactly once

**Test Strategy**:
```rust
#[test]
fn subset_iterator_matches_eager_generation() {
    let elements = vec![1, 2, 3, 4, 5];
    let min = 2;
    let max = 4;

    // Generate with current eager method
    let eager_results: HashSet<_> = min_max_subsets(&elements, min, max).into_iter().collect();

    // Generate with new lazy iterator
    let lazy_results: HashSet<_> = SubsetIterator::new(&elements, min, max).collect();

    assert_eq!(eager_results, lazy_results);
}
```

### Step 2: Implement `SubParsIterator` (3-4 days)

1. Create 7-component odometer-style iterator
2. Integrate with `SubsetIterator` for each component
3. Unit tests comparing against current `sub_pars`
4. Benchmarks measuring memory usage and speed

**Test Strategy**:
```rust
#[test]
fn sub_pars_iterator_matches_eager() {
    let par = create_test_par(3, 3, 2, 3, 1, 1, 1);
    let min = ParCount { ... };
    let max = ParCount { ... };

    // Current implementation
    let eager_results: HashSet<_> = sub_pars(&par, &min, &max, &min_prune, &max_prune).collect();

    // New implementation
    let lazy_results: HashSet<_> = sub_pars_lazy(&par, &min, &max, &min_prune, &max_prune).collect();

    assert_eq!(eager_results, lazy_results);
}
```

### Step 3: Integration and Validation (1-2 days)

1. Replace current `sub_pars` with lazy version
2. Run full test suite
3. Run spatial matcher tests
4. Benchmark before/after on realistic inputs

### Step 4: Constraint Propagation (Optional, 2-3 days)

1. Add `should_continue()` pruning logic
2. Benchmark additional speedup
3. Validate correctness

---

## Expected Performance Impact

### Memory

**Before**: O(2^n) for each component × 7 components
- Example: 10 elements per component = 1,024 subsets × 7 = 7,168 subset vectors
- Each subset vector contains ~5 elements on average
- Total: ~35,000 element clones in memory simultaneously

**After**: O(n) for current state only
- 7 components × ~5 elements each = ~35 elements in memory
- **1,000x memory reduction**

### CPU

**Before**: O(2^n × n) subset generation + O(k^7) cartesian product
- For 5+5+3+8+2+1+1 elements: ~134 million combinations generated upfront

**After**: O(1) per combination generated lazily
- Generate combinations on-demand as consumed
- If spatial matcher finds match early, avoid generating remaining combinations
- **100-1000x speedup for typical cases** (depends on early termination)

---

## Alternative Approaches Considered

### Approach 1: Backtracking Search

Instead of enumerating all subsets, use backtracking to explore valid combinations:

```rust
fn backtrack(
    par: &Par,
    partial_result: &mut PartialPar,
    remaining_components: &[ComponentType],
    constraints: &Constraints,
) -> Option<(Par, Par)> {
    if constraints_satisfied(partial_result) {
        return Some(build_par_pair(partial_result));
    }

    if remaining_components.is_empty() {
        return None;
    }

    let component = remaining_components[0];
    for element in component {
        partial_result.add(element);
        if let Some(result) = backtrack(par, partial_result, &remaining_components[1..], constraints) {
            return Some(result);
        }
        partial_result.remove(element);
    }

    None
}
```

**Pros**:
- Natural pruning via constraint checking
- Can find single valid combination very quickly

**Cons**:
- More complex state management
- Requires generating ALL valid combinations (spatial matcher needs all)
- Recursive (though tail-recursion can be optimized)

**Decision**: Keep lazy iterator approach for now, consider backtracking if constraints are very selective.

### Approach 2: Hybrid Eager/Lazy

Generate subsets eagerly for small components (<= 10 elements), lazy for large components.

**Pros**:
- Optimal performance across input sizes

**Cons**:
- Added complexity
- Bitmask approach already handles small inputs efficiently

**Decision**: Start with pure lazy, optimize later if needed.

---

## Success Criteria

1. ✅ Lazy iterator generates same results as eager version (validated by tests)
2. ✅ Memory usage: O(n) instead of O(2^n)
3. ✅ Can handle inputs with 20+ elements per component without OOM
4. ✅ Speedup: 50-100x for realistic spatial matching workloads
5. ✅ All existing tests pass with lazy implementation

---

## Next Steps

1. ⏭️ Implement `SubsetIterator` with unit tests
2. ⏭️ Benchmark current implementation to establish baseline
3. ⏭️ Implement `SubParsIterator` with integration tests
4. ⏭️ Replace current `sub_pars` and validate
5. ⏭️ Measure actual speedup and document results

---

## References

- Original Scala implementation: `ParSpatialMatcherUtils.scala - subPars`
- Current Rust implementation: `rholang/src/rust/interpreter/matcher/sub_pars.rs`
- Called from: `spatial_matcher.rs:274`
- Analysis document: `docs/performance/sub-pars-analysis.md`
