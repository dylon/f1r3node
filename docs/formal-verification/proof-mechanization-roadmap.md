# Proof-by-Proof Mechanization Roadmap

**Document Version**: 1.0
**Last Updated**: 2025-11-07
**Status**: Planning Phase
**Related Document**: `mechanization-strategy.md`

---

## Overview

This document provides detailed mechanization plans for each of the 11 optimization equivalence proofs. For each proof, we analyze:
- What can be mechanized vs what is empirical
- Recommended theorem prover and approach
- Dependencies on other proofs and foundational work
- Estimated effort and complexity
- Key lemmas and theorems to formalize
- Potential pitfalls and mitigation strategies

---

## Proof Dependency Graph

```
Foundational Work (4-6 months, 480 lemmas)
  ├─ ProcessTree, Par, State definitions
  ├─ 400-500 reusable lemmas (lists, sets, state, eval, complexity)
  ├─ Rust semantics (Vec, Rc, amortized analysis)
  └─ Custom automation (tactics, sledgehammer)
      │
      ├─→ Proof 1: Par Flattening (4-6 weeks) ─┐
      │                                          │
      ├─→ Proof 6: Lazy sub_pars (6-8 weeks) ───┼─→ Phase 1 Complete (6-8 months total)
      │                                          │
      ├─→ Proof 11: State Isolation (6-10 weeks)│
      │                                          │
      ├─→ Proof 4: Accumulator (8-12 weeks) ────┼─→ Proof 5: Match (3-5 weeks)
      │                                          │
      └─→ Proofs 2, 3, 7-10 (4-8 weeks each) ───┘
                                                 │
                                                 v
                                   Complete Library (480 lemmas)
```

**Critical Path**: Foundational Work (24 weeks) → Proof 1 (6 weeks) → Proof 6 (8 weeks) → Proof 11 (10 weeks)
**Estimated Duration**: 48 weeks (12 months) for Phase 1 critical path
**Total for All 11 Proofs**: 18-26 months (foundational + all proofs)

---

## Proof 1: Iterative Par Flattening (f5219577)

### Classification
- **Type**: Algorithm transformation (recursive → iterative)
- **Mechanization Potential**: **95%**
- **Recommended Tool**: Coq
- **Estimated Effort**: 4-6 weeks (with foundational library)
- **Priority**: **CRITICAL** (Phase 1, foundational for other proofs)

### What Can Be Mechanized

#### Core Theorem
```coq
Theorem iterative_recursive_equivalence :
  forall (t : ProcessTree) (s : State),
    ⟦t⟧_recursive(s) = ⟦t⟧_iterative(s).
```

#### Supporting Lemmas
1. **Fold Decomposition (Lemma 1.1)**
   ```coq
   Lemma fold_left_app :
     forall {A B} (f : A -> B -> A) (l1 l2 : list B) (acc : A),
       fold_left f (l1 ++ l2) acc =
       fold_left f l2 (fold_left f l1 acc).
   ```
   - Status: **Standard library theorem** (List.fold_left_app)
   - Effort: 0 (reuse existing)

2. **Flatten Preserves Structure**
   ```coq
   Lemma flatten_correct :
     forall t : ProcessTree,
       forall p : Process,
         In p (flatten t) <-> ProcessInTree p t.
   ```
   - Effort: 1 day (straightforward induction)

3. **Normalization Determinism**
   ```coq
   Lemma normalize_atomic_deterministic :
     forall (p : Process) (s1 s2 : State),
       s1 = s2 -> normalize_atomic p s1 = normalize_atomic p s2.
   ```
   - Effort: 1 day (follows from function definition)

#### Inductive Proof Structure
```coq
Proof.
  induction t as [p | left IHleft right IHright].
  - (* Base case: Atomic p *)
    unfold normalize_recursive, normalize_iterative.
    simpl. reflexivity.
  - (* Inductive case: Par left right *)
    unfold normalize_recursive, normalize_iterative.
    simpl.
    rewrite fold_left_app.  (* Use Lemma 1.1 *)
    rewrite IHleft, IHright.
    reflexivity.
Qed.
```

**Estimated proof length**: 30-50 lines of Coq

### What Cannot Be Mechanized

❌ **Stack overflow behavior** - This is an implementation detail:
```
Recursive: Stack depth = O(height(tree))
Iterative: Stack depth = O(1)
```
- Cannot prove in Coq (depends on Rust stack limit)
- Empirical validation only: Test with 50,000 nested Pars

❌ **Benchmark timing** - Performance measurements:
```
Empirical: Iterative is 11-15% faster
```
- Measures wall-clock time (hardware-dependent)
- Can prove complexity bounds (both O(n)), but not concrete speedup

### Manual Formalization Required

⚠️ **Process Tree Structure** (1 week effort):
```coq
Inductive ProcessTree : Type :=
  | Atomic : Process -> ProcessTree
  | Par : ProcessTree -> ProcessTree -> ProcessTree.
```

⚠️ **State and Normalization** (1 week effort):
```coq
Record State : Type := {
  par : Par;
  free_map : FreeMap;
  bound_map_chain : BoundMapChain
}.

Definition normalize_atomic (p : Process) (s : State) : State :=
  (* Formalize normalization logic *)
```

### Dependencies
- **Foundational**: ProcessTree definition, State record, List library
- **External**: None (self-contained)

### Potential Pitfalls
1. **Complexity of normalize_atomic**: May be very large to formalize
   - **Mitigation**: Abstract as uninterpreted function for equivalence proof
   ```coq
   Parameter normalize_atomic : Process -> State -> State.
   ```

2. **Par structure has 7 components**: Full definition is verbose
   - **Mitigation**: Use record notation, hide implementation details

### Success Criteria
- ✅ Main theorem proven and checked by Coq
- ✅ Proof reuses standard library fold_left_app lemma
- ✅ All tactics discharge automatically or with simple induction
- ✅ Proof time < 1 second (validates simplicity)

---

## Proof 4: Accumulator Pattern (52da5ee6)

### Classification
- **Type**: Complexity transformation (O(n²) → O(n))
- **Mechanization Potential**: **90%**
- **Recommended Tool**: Coq or Isabelle/HOL
- **Estimated Effort**: 8-12 weeks (most complex proof - requires amortized analysis, hidden prepending mechanics)
- **Priority**: **HIGH** (Phase 2, dramatic speedup validation, high complexity)

### What Can Be Mechanized

#### Core Theorems

**Theorem 1: Semantic Equivalence**
```coq
Theorem prepend_extend_equivalence :
  forall (procs : list Process) (acc : Par),
    reverse (fold_right prepend_to_par Par.empty procs) =
    fold_left extend_par Par.empty procs.
```

**Theorem 2: Quadratic Complexity Upper Bound**
```coq
Definition prepend_cost (n : nat) : nat :=
  n * (n + 1) / 2.

Theorem prepend_quadratic :
  forall (procs : list Process),
    time_complexity (fold_right prepend_to_par Par.empty procs) <=
    O(prepend_cost (length procs)).
```

**Theorem 3: Linear Complexity Upper Bound**
```coq
Definition extend_cost (n : nat) : nat := n.

Theorem extend_linear :
  forall (procs : list Process),
    time_complexity (fold_left extend_par Par.empty procs) <=
    O(extend_cost (length procs)).
```

#### Supporting Lemmas

1. **Prepend ≡ Cons + Append**
   ```coq
   Lemma prepend_def :
     forall (p : Process) (par : Par),
       prepend_to_par p par = Par.append (Par.singleton p) par.
   ```

2. **Extend ≡ Append**
   ```coq
   Lemma extend_def :
     forall (p : Process) (par : Par),
       extend_par par p = Par.append par (Par.singleton p).
   ```

3. **Reverse Fold Right = Fold Left**
   ```coq
   Lemma reverse_fold :
     forall {A B} (f : A -> B -> B) (g : B -> A -> B) (l : list A) (b : B),
       (forall x y, f x (g y x) = g y x) ->
       reverse (fold_right f b l) = fold_left g b l.
   ```

4. **Amortized Vec Growth** (Isabelle-specific, uses time monad)
   ```isabelle
   lemma vec_push_amortized:
     "amortized_cost (fold push_back [] xs) = O(length xs)"
   ```

### What Cannot Be Mechanized

❌ **Empirical speedup measurement**:
```
Benchmark: 6,158× speedup (50K nested Pars: 192.54s → 31.26ms)
```
- Hardware-dependent, cannot prove in formal system

❌ **Memory allocator behavior**:
```
Vec growth strategy: Doubles capacity (2× geometric growth)
```
- Implementation detail of Rust allocator (jemalloc)
- Can model abstractly, but not prove actual behavior

### Manual Formalization Required

⚠️ **Cost Model** (1-2 weeks):
```coq
(* Abstract cost model for Par operations *)
Inductive Cost : Type :=
  | C_Prepend : nat -> Cost  (* Cost proportional to accumulated size *)
  | C_Extend : nat -> Cost   (* Constant cost *)
  | C_Seq : Cost -> Cost -> Cost.

Definition eval_cost (c : Cost) : nat := (* ... *).
```

⚠️ **Par Append Semantics** (1 week):
```coq
Definition Par.append (p1 p2 : Par) : Par :=
  {| sends := p1.sends ++ p2.sends;
     receives := p1.receives ++ p2.receives;
     (* ... 7 components total ... *)
  |}.
```

### Dependencies
- **Foundational**: Par structure, List append properties
- **From Proof 1**: ProcessTree flattening (similar fold reasoning)
- **External**: Complexity theory (big-O notation formalization)

### Approach Comparison

| Tool | Semantic Proof | Complexity Proof | Automation | Estimated Time |
|------|----------------|------------------|------------|----------------|
| **Coq** | ✅ Strong | ⚠️ Manual | Low | 4 weeks |
| **Isabelle** | ✅ Strong | ✅ Time monad | High | 3 weeks |

**Recommendation**: Start with **Isabelle/HOL** for better automation on complexity bounds.

### Potential Pitfalls

1. **Complexity formalization is non-trivial**: Big-O notation is informal
   - **Mitigation**: Use concrete bounds (n*(n+1)/2 vs n) instead of asymptotic

2. **Amortized analysis requires potential method**: Vec doubling is amortized O(1)
   - **Mitigation**: Cite existing Isabelle theories (Amortized_Complexity AFP entry)

3. **7-component Par structure is verbose**: Each component needs append
   - **Mitigation**: Use Coq's record notation with `{ field := value }` syntax

### Success Criteria
- ✅ Semantic equivalence proven
- ✅ Quadratic upper bound n*(n+1)/2 proven
- ✅ Linear upper bound n proven
- ✅ Speedup ratio > n for all n > 10 (proven, not just measured)

---

## Proof 5: Match Optimization (6e2bf27e)

### Classification
- **Type**: Algebraic transformation (double-reversal cancellation)
- **Mechanization Potential**: **90%**
- **Recommended Tool**: Coq
- **Estimated Effort**: 2-3 weeks
- **Priority**: **MEDIUM** (similar to Proof 4, lower impact)

### What Can Be Mechanized

#### Core Theorem
```coq
Theorem double_reversal_cancellation :
  forall (matches : list Match),
    reverse (reverse matches) = matches.
```

#### Fold Law Application
```coq
Theorem foldr_foldl_reverse :
  forall {A B} (f : A -> B -> B) (l : list A) (b : B),
    fold_right f b (reverse l) = fold_left (flip f) b l.
```

### What Cannot Be Mechanized
❌ **Projected speedup** (not measured, only estimated)

### Dependencies
- **From Proof 4**: Fold reversal lemmas
- **Foundational**: List reversal properties

### Estimated Effort
- **Week 1**: Formalize Match structure
- **Week 2**: Prove double-reversal cancellation
- **Week 3**: Connect to fold laws, complete proof

---

## Proof 6: Lazy Iterator for sub_pars (e1a3d853)

### Classification
- **Type**: Data structure transformation (eager → lazy)
- **Mechanization Potential**: **95%**
- **Recommended Tool**: Isabelle/HOL or Lean 4
- **Estimated Effort**: 6-8 weeks (pure mathematics - bijection proofs, combinatorics, iterator properties)
- **Priority**: **CRITICAL** (Phase 1, excellent Isabelle showcase, self-contained)

### What Can Be Mechanized

#### Core Theorems

**Theorem 1: Bitmask-Subset Bijection (Lemma 6.1)**
```lean
theorem bitmask_subset_bijection (items : List α) :
  Bijective (λ mask : Fin (2^items.length) =>
    items.filterIdx (λ i _ => mask.val.testBit i)) :=
by
  apply bijective_iff_has_inverse.mpr
  use (λ subset => ⟨subsetToMask subset items, ...⟩)
  constructor <;> intro x <;> simp [...]
```

**Theorem 2: Cardinality Preservation**
```isabelle
theorem cartesian_product_cardinality:
  "card (A × B) = card A * card B"
```

**Theorem 3: Memory Complexity**
```coq
Theorem eager_memory :
  forall n : nat,
    memory_used (sub_pars_eager n) = O(2^n).

Theorem lazy_memory :
  forall n : nat,
    memory_used (sub_pars_lazy n) = O(1).
```

#### Supporting Lemmas

1. **Popcount Filtering**
   ```coq
   Lemma popcount_correct :
     forall (mask : nat) (n : nat),
       popcount mask = count_ones (nat_to_binary mask n).
   ```

2. **Subset Generation Completeness**
   ```coq
   Lemma subset_iterator_complete :
     forall (items : list A) (min max : nat),
       forall (subset : list A),
         subset ⊆ items ->
         length subset >= min ->
         length subset <= max ->
         exists mask, SubsetIterator.generates mask subset.
   ```

3. **No Duplicate Subsets**
   ```coq
   Lemma subset_iterator_nodups :
     forall items min max,
       NoDup (SubsetIterator.collect items min max).
   ```

### What Cannot Be Mechanized

❌ **Early termination benefit** (input-dependent):
```
Empirical: Pattern matcher finds match in first 10-100 iterations (out of millions)
```
- Highly dependent on input distribution
- Can prove worst-case O(2^n), but not average-case

❌ **Benchmark speedup** (40-46%):
- Hardware-dependent measurement

### Manual Formalization Required

⚠️ **Bitmask Enumeration** (1 week):
```coq
Fixpoint mask_to_subset (items : list A) (mask : nat) (index : nat) : list A :=
  match items with
  | [] => []
  | x :: xs =>
      if testBit mask index
      then x :: mask_to_subset xs mask (S index)
      else mask_to_subset xs mask (S index)
  end.
```

⚠️ **Cartesian Product** (1-2 weeks):
```coq
Fixpoint cartesian_product {A B} (xs : list A) (ys : list B) : list (A * B) :=
  match xs with
  | [] => []
  | x :: xs' =>
      map (pair x) ys ++ cartesian_product xs' ys
  end.
```

⚠️ **7-Way Cartesian Product** (1 week):
```coq
Definition sub_pars_cartesian
  (sends receives news exprs matches unforgeables bundles : list (list A * list A)) :
  list (Par * Par) :=
  (* Nested cartesian products of 7 component iterators *)
```

### Approach Comparison

| Tool | Bijection Proof | Automation | Mathematical Libraries | Time |
|------|----------------|------------|----------------------|------|
| **Isabelle** | ✅ Strong | ✅ Excellent | ✅ Comprehensive | 4 weeks |
| **Lean 4** | ✅ Strong | ⚠️ Good | ✅ Mathlib | 5 weeks |
| **Coq** | ✅ Strong | ⚠️ Manual | ⚠️ Moderate | 6 weeks |

**Recommendation**: **Isabelle/HOL** for superior automation on combinatorial proofs.

### Dependencies
- **Foundational**: Par structure (7 components)
- **External**: Combinatorics library (bijections, cardinality)

### Potential Pitfalls

1. **Bitmask arithmetic is tedious**: Bit operations, binary representation
   - **Mitigation**: Use Isabelle's Word library or Lean's BitVec

2. **7-way cartesian product is verbose**: Nested tuple destructuring
   - **Mitigation**: Use algebraic properties (associativity, cardinality)

3. **Spatial pattern matching context is complex**: Requires understanding bipartite matching
   - **Mitigation**: Abstract away - focus on subset enumeration bijection only

### Success Criteria
- ✅ Bitmask ↔ subset bijection proven
- ✅ Memory complexity O(2^n) vs O(1) proven
- ✅ Cardinality preservation proven
- ✅ No duplicate subsets property proven

---

## Proof 11: State Isolation for ListMatch (843268ae)

### Classification
- **Type**: Correctness bug fix (state contamination)
- **Mechanization Potential**: **95%**
- **Recommended Tool**: Coq (monad libraries)
- **Estimated Effort**: 6-10 weeks (monad laws, referential transparency, counterexample construction, bipartite matching context)
- **Priority**: **CRITICAL** (Phase 1, correctness not optimization, foundational for pattern matching)

### What Can Be Mechanized

#### Core Theorems

**Theorem 1: Referential Transparency**
```coq
Definition ReferentiallyTransparent {A B} (f : A -> B) : Prop :=
  forall x y, x = y -> f x = f y.

Theorem isolate_referential_transparent :
  forall {S A} (f : State S A),
    ReferentiallyTransparent (isolateState f).
```

**Theorem 2: State Contamination Prevented**
```coq
Theorem contamination_prevented :
  forall {S A} (f : State S A) (s0 s1 : S),
    fst (isolateState f s0) = fst (isolateState f s1) ->
    snd (isolateState f s0) = s0 /\
    snd (isolateState f s1) = s1.
```

**Theorem 3: Broken Implementation Counterexample**
```coq
Lemma contaminated_not_referential_transparent :
  exists (f : State FreeMap unit) (s1 s2 : FreeMap),
    s1 = s2 /\
    f s1 <> f s2.  (* Violates referential transparency *)
```

#### Supporting Lemmas

1. **Monad State Laws**
   ```coq
   Lemma isolate_state_monad_law1 :
     forall {S A} (a : A),
       isolateState (return a) = return a.

   Lemma isolate_state_monad_law2 :
     forall {S A B} (m : State S A) (f : A -> State S B),
       isolateState (m >>= f) = isolateState m >>= (fun x => isolateState (f x)).
   ```

2. **Clone Equivalence**
   ```coq
   Lemma clone_preserves_equality :
     forall (s : State),
       s.clone() = s.
   ```

### What Cannot Be Mechanized

❌ **Performance impact** (<5% overhead):
- Measurement-based, hardware-dependent

❌ **Bipartite matching algorithm**: Complex graph algorithm
- Can model abstractly, but full formalization is separate effort
- Focus on state isolation property only

### Manual Formalization Required

⚠️ **State Monad** (1 week):
```coq
Definition State (S A : Type) : Type := S -> (A * S).

Definition return {S A} (x : A) : State S A :=
  fun s => (x, s).

Definition bind {S A B} (m : State S A) (f : A -> State S B) : State S B :=
  fun s0 =>
    let (a, s1) := m s0 in
    f a s1.
```

⚠️ **isolateState Wrapper** (1 week):
```coq
Definition isolateState {S A} (m : State S A) : State S A :=
  fun s0 =>
    let (a, s1) := m s0 in  (* Run with initial state *)
    (a, s0).                 (* Return result, restore initial state *)
```

⚠️ **FreeMap Structure** (1 week):
```coq
Definition FreeMap : Type := String.Map VarSort.

Definition bind_var (m : FreeMap) (var : String) (val : VarSort) : FreeMap :=
  String.Map.add var val m.
```

### Dependencies
- **Foundational**: State monad definition, FreeMap
- **External**: Monad laws library (Coq Ext Lib or custom)

### Approach

**Week 1-2:** Formalize state monad, prove monad laws
**Week 3:** Prove `isolateState` preserves referential transparency
**Week 4:** Construct counterexample showing broken implementation fails
**Week 5:** Polish proof, write documentation

### Potential Pitfalls

1. **Monad formalization is intricate**: Type classes, notations
   - **Mitigation**: Use Coq Ext Lib (pre-existing monad library)

2. **Counterexample construction requires concrete execution**: Need to show specific inputs where contamination occurs
   - **Mitigation**: Use Coq's `compute` tactic to evaluate example

3. **Closure semantics are Rust-specific**: `move` vs `clone` capture
   - **Mitigation**: Abstract as "fresh context per invocation" vs "reused context"

### Success Criteria
- ✅ Referential transparency of `isolateState` proven
- ✅ Counterexample of contaminated version constructed and proven to violate referential transparency
- ✅ Monad laws for State monad proven
- ✅ Connection to Scala's `isolateState` documented (informal correspondence)

---

## Proofs 2, 3, 7-10: Medium Feasibility (Summary)

### Proof 2: Rc BoundMapChain (2d90323a)
- **Mechanization Potential**: 70%
- **Effort**: 4-6 weeks
- **Challenge**: Rc reference counting semantics
- **Approach**: Model Rc as abstract shared pointer with clone = O(1)

### Proof 3: Pre-allocation (9d4d619a)
- **Mechanization Potential**: 75%
- **Effort**: 3-4 weeks
- **Challenge**: Vec amortized growth analysis
- **Approach**: Use Isabelle time monad, model doubling strategy

### Proof 7: Substitution Clone Reduction (e8cdd1a7)
- **Mechanization Potential**: 60%
- **Effort**: 6-8 weeks
- **Challenge**: Rust ownership semantics (move vs borrow)
- **Approach**: Use RustBelt library, prove measurement determinism

### Proof 8: FreeMap Persistent HashMap (cb5988da)
- **Mechanization Potential**: 80%
- **Effort**: 3-5 weeks
- **Challenge**: im::HashMap HAMT structure
- **Approach**: Use Isabelle RBT_Map or Coq FMapAVL as model

### Proof 9: BoundMapChain Persistent (7df36c87)
- **Mechanization Potential**: 80%
- **Effort**: 3-5 weeks
- **Challenge**: Rc-based linked list sharing
- **Approach**: Formalize structural sharing via path copying

### Proof 10: Environment Persistent (162e763b)
- **Mechanization Potential**: 85%
- **Effort**: 3-5 weeks
- **Challenge**: De Bruijn index semantics
- **Approach**: Standard formalization (well-studied in literature)

---

## Critical Path Analysis

### Shortest Path to 5 Core Proofs

```
Month 1: Foundational Work
  └─> Week 1-2: ProcessTree, Par, State, FreeMap
  └─> Week 3-4: Fold lemmas, List properties

Month 2: Proof 1 (Par Flattening)
  └─> Week 1: Formalize definitions
  └─> Week 2: Prove equivalence
  └─> Week 3: Polish, document

Month 3: Proof 6 (Lazy sub_pars)
  └─> Week 1-2: Bitmask bijection
  └─> Week 3-4: Cartesian product

Month 4: Proof 4 (Accumulator)
  └─> Week 1-2: Complexity model
  └─> Week 3: Equivalence proof
  └─> Week 4: Bounds proofs

Month 5: Proof 11 (State Isolation)
  └─> Week 1-2: State monad
  └─> Week 3: isolateState proof
  └─> Week 4: Counterexample

Month 6: Proof 5 (Match Optimization)
  └─> Week 1-2: Double-reversal
  └─> Week 3: Polish all proofs

Total: 6 months (24 weeks) for 5 core proofs
```

### Parallelization Opportunities

If 2 people available:

**Person A:** Proofs 1, 4, 5 (Coq expert) - 3 months
**Person B:** Proofs 6, 11 (Isabelle/monad expert) - 3 months
**Overlap:** Foundational work (collaborate) - 1 month

**Total with 2 people:** 4 months (16 weeks)

---

## Risk Matrix

| Proof | Complexity | Novelty | Tool Maturity | Overall Risk |
|-------|------------|---------|---------------|--------------|
| Proof 1 | Low | Low | High | **LOW** |
| Proof 4 | Medium | Low | High | **LOW** |
| Proof 5 | Low | Low | High | **LOW** |
| Proof 6 | Medium | Medium | High | **MEDIUM** |
| Proof 11 | Medium | Low | High | **LOW** |
| Proof 7 | High | High | Medium | **HIGH** |
| Proofs 8-10 | Medium | Low | Medium | **MEDIUM** |

**Recommendation**: Start with low-risk proofs (1, 4, 5, 11) before tackling medium/high-risk.

---

## Conclusion

The roadmap demonstrates a clear path to mechanizing 5 high-value proofs in **11-17 months** with **1-2 people**. The critical path focuses on low-risk, high-impact proofs that provide the most confidence gain.

**Next Steps:**
1. Secure resources (1-2 Coq/Isabelle experts)
2. Begin foundational work (ProcessTree, Par definitions)
3. Follow critical path: Proofs 1 → 4 → 5 → 6 → 11
4. Evaluate after each proof for go/no-go decision

---

**Document Status:** ✅ Complete
**Next Document:** `coq-formalization-examples.md` - Detailed Coq code examples
