# Coq/Rocq Learning Materials for Rholang Proofs

This document provides additional learning resources to complement the documented Coq proofs, including a tactics cheat sheet, exercises, and quick reference guides.

---

## Table of Contents

1. [Quick Start Guide](#quick-start-guide)
2. [Coq Tactics Cheat Sheet](#coq-tactics-cheat-sheet)
3. [Type Theory Quick Reference](#type-theory-quick-reference)
4. [Common Proof Patterns](#common-proof-patterns)
5. [Learning Exercises](#learning-exercises)
6. [Proof Strategy Decision Tree](#proof-strategy-decision-tree)
7. [Debugging Guide](#debugging-guide)
8. [File-by-File Learning Path](#file-by-file-learning-path)
9. [Further Reading](#further-reading)

---

## Quick Start Guide

### I'm Brand New to Coq - Where Do I Start?

**Day 1: Understanding the Basics** (2-3 hours)
1. Read RholangCore.v header comments (lines 1-80)
   - Learn what inductive types are
   - See how Records work
   - Understand axioms vs proofs

2. Read Proof05_MatchOptimization.v (SHORTEST proof file)
   - See a complete proof from start to finish
   - Only uses `apply` tactic (simplest!)
   - Builds on standard library lemma

**Day 2-3: Simple Proofs** (4-6 hours)
3. Read Proof02_RcSharing.v
   - Semantic preservation proof (short!)
   - Uses `unfold`, `simpl`, `reflexivity`
   - Learn about Records and Rc model

4. Try Exercise 1 and 2 from [Learning Exercises](#learning-exercises)

**Week 1: Induction Mastery** (10-15 hours)
5. Read RholangLemmas.v focusing on:
   - `fold_left_app` (THE KEY LEMMA) - induction on lists
   - `sum_n_formula` - arithmetic induction
   - Notice the proof patterns

6. Read Proof03_PreAllocation.v
   - Helper lemma with assert
   - Induction inside proof block
   - Complex but well-documented

7. Try Exercises 3-5

**Week 2-3: Complex Proofs** (20+ hours)
8. Read Proof01_ParFlattening.v (most complex!)
   - Fuel adequacy lemmas
   - Transitivity reasoning
   - Multiple inductions

9. Read Proof04_AccumulatorPattern.v
   - See Gauss's formula application
   - Amortized analysis reasoning

10. Read Proof06_LazySubPars.v
    - Bitmask bijections
    - Existential proofs

### Quick Wins: Copy-Paste Templates

**Prove Equality After Simplification**:
```coq
Theorem my_equality : expr1 = expr2.
Proof.
  simpl.  (* or: unfold definitions *)
  reflexivity.
Qed.
```

**Prove Arithmetic Inequality**:
```coq
Theorem my_bound : expr1 <= expr2.
Proof.
  lia.  (* Solves most arithmetic! *)
Qed.
```

**Prove List Property by Induction**:
```coq
Lemma list_prop : forall xs, property xs.
Proof.
  intro xs.
  induction xs as [| x xs' IH].
  - (* Base: xs = [] *) auto.
  - (* Inductive: xs = x :: xs', IH: property xs' *)
    (* Use IH here *)
Qed.
```

**Apply Existing Lemma**:
```coq
Theorem use_lemma : goal.
Proof.
  apply existing_lemma.
  (* Coq handles type matching automatically! *)
Qed.
```

### Common Gotchas for Beginners

❌ **Don't do this**:
```coq
Proof.
  auto.  (* Hoping it solves everything *)
Qed.  (* Usually fails *)
```

✅ **Do this instead**:
```coq
Proof.
  (* 1. Explore what you need to prove *)
  Show.
  (* 2. Try simple tactics *)
  try reflexivity.
  try lia.
  (* 3. If automation fails, break it down *)
  intros.
  destruct ...
  (* etc. *)
```

❌ **Don't do this**:
```coq
induction xs.  (* No naming - hard to read *)
```

✅ **Do this instead**:
```coq
induction xs as [| x xs' IH].  (* Named cases and IH *)
```

❌ **Don't do this**:
```coq
(* Trying to prove something too specific *)
Lemma bad : forall n, f n 0 = 0.  (* Can't generalize later *)
```

✅ **Do this instead**:
```coq
(* Make it general from the start *)
Lemma good : forall n m, property (f n m).
```

---

## Coq Tactics Cheat Sheet

### Automatic Tactics (Try These First!)

| Tactic | When to Use | Example |
|--------|-------------|---------|
| `reflexivity` | Prove X = X after simplification | `1 + 1 = 2` (after simpl) |
| `lia` | Linear arithmetic goals | `n + m ≥ n`, `2*x = x + x` |
| `auto` | Simple goals using hypotheses | Conjunction, simple implications |
| `trivial` | Very simple goals | `True`, goals following from hypotheses |

**Example from Proofs**:
```coq
Lemma double_nat : forall n : nat, n + (n + 0) = 2 * n.
Proof.
  intro n. lia.  (* lia solves arithmetic automatically *)
Qed.
```

---

### Manual Tactics (For Control)

#### **intros** - Introduce hypotheses and variables
```coq
(* Goal: ∀ n m, n + m = m + n *)
intros n m.
(* Now: n, m : nat ⊢ n + m = m + n *)

(* Can also name introduced items *)
intros [x xs] H.  (* Destructure while introducing *)
```

#### **simpl** - Simplify by computation
```coq
(* Goal: fold_left f [x] b = ? *)
simpl.
(* Goal: f b x = ? *)

(* Works on recursive function definitions *)
```

#### **rewrite** - Use equality to transform goal
```coq
(* Given: H : x = y *)
rewrite H.      (* Replace x with y in goal *)
rewrite <- H.   (* Replace y with x in goal *)
rewrite H in H2.  (* Rewrite in hypothesis H2 *)

(* Can chain rewrites *)
rewrite H1, H2, H3.
```

**Example from Proof01**:
```coq
rewrite IHt2.  (* Apply inductive hypothesis for t2 *)
rewrite IHt1.  (* Apply inductive hypothesis for t1 *)
reflexivity.   (* Both sides now identical *)
```

#### **induction** - Prove by induction
```coq
(* For lists *)
induction xs as [| x xs' IH].
- (* Case: xs = [] *)
  ...
- (* Case: xs = x :: xs', IH: property holds for xs' *)
  ...

(* For natural numbers *)
induction n as [| n' IH].
- (* n = 0 *)
- (* n = S n', IH: property holds for n' *)
```

**Example from RholangLemmas**:
```coq
Lemma fold_left_app : ...
Proof.
  intros f l1.
  induction l1 as [| x xs IH]; intros l2 b; simpl.
  - reflexivity.  (* Base case: l1 = [] *)
  - rewrite IH. reflexivity.  (* Inductive case *)
Qed.
```

#### **destruct** - Case analysis without induction
```coq
(* For data types *)
destruct t.  (* Generate one case per constructor *)

(* For boolean decisions *)
destruct (x =? y) eqn:E.
- (* Case: x =? y = true *)
- (* Case: x =? y = false *)

(* For existentials *)
destruct H as [witness proof].
```

**Example from Proof01**:
```coq
destruct fuel as [| fuel'].
+ (* fuel = 0 - contradiction *)
  lia.
+ (* fuel = S fuel' - continue proof *)
  ...
```

---

### Specialized Tactics

#### **assert** - Introduce helper lemma
```coq
(* Inline lemma that might be useful *)
assert (H : 2 * sum_n n = n * (n + 1)).
{ apply sum_n_formula. }
(* Now H is available as hypothesis *)
```

#### **transitivity** - Prove A = C via A = B = C
```coq
transitivity middle_term.
- (* Prove: goal = middle_term *)
- (* Prove: middle_term = original_goal *)
```

**Example from Proof01**:
```coq
transitivity (norm_recursive t2
               (norm_recursive t1 st (2*size t1))
               (2*size t2)).
* (* LHS = middle *)
  apply IHt2.
* (* middle = RHS *)
  ...
```

#### **exfalso** - Prove from contradiction
```coq
(* When you have contradictory hypotheses *)
exfalso.
(* Goal changes to: False *)
(* Now prove False from hypotheses *)
```

**Example from RholangLemmas**:
```coq
destruct (in_dec eq_dec a (a :: s)) as [_ | Hcontra].
+ reflexivity.
+ exfalso.  (* a should be in (a :: s)! *)
  apply Hcontra.
  left. reflexivity.
```

#### **discriminate** - Prove constructor inequality
```coq
(* Given: H : [] = x :: xs *)
discriminate H.  (* Proves goal - constructors differ *)
```

#### **f_equal** - Apply function to both sides
```coq
(* Goal: f x = f y *)
f_equal.
(* New goal: x = y *)
```

#### **apply** - Use a lemma/theorem
```coq
(* Given: lemma : ∀ x, P x → Q x *)
(*        H : P a *)
apply lemma in H.  (* H becomes: Q a *)

(* Or apply to goal *)
(* Goal: Q a *)
apply lemma.  (* New goal: P a *)
```

---

### Tactical Combinators

| Combinator | Meaning | Example |
|------------|---------|---------|
| `; [t1 | t2 | t3]` | Apply different tactics to subgoals | `induction n; [auto | lia]` |
| `try t` | Try t, continue if it fails | `try reflexivity` |
| `repeat t` | Repeat t until it fails | `repeat rewrite H` |
| `t1; t2` | Do t1 then t2 on all subgoals | `simpl; auto` |
| `all: t` | Apply t to all goals | `all: try discriminate` |

**Example from RholangCore**:
```coq
Proof.
  intros t.
  induction t; simpl.
  - (* PNil *)  try discriminate.
  - (* PSend *) try discriminate.
  ...
  (* [try discriminate] applied to all atomic cases *)
```

---

## Type Theory Quick Reference

### Inductive Types

**Definition**: Types defined by their constructors.

```coq
Inductive nat : Type :=
  | O : nat
  | S : nat -> nat.

Inductive list (A : Type) : Type :=
  | nil : list A
  | cons : A -> list A -> list A.
```

**In Rholang Proofs**:
```coq
Inductive ProcessTree : Type :=
  | PNil : ProcessTree
  | PSend : Chan -> list Proc -> ProcessTree
  | PPar : ProcessTree -> ProcessTree -> ProcessTree
  | ...  (* 8 constructors total *)
```

**Key Properties**:
- **Structural induction**: Prove property for all constructors
- **Pattern matching**: Compute by case analysis
- **No cycles**: Can't construct infinite values

---

### Dependent Types

**Definition**: Types that depend on values.

```coq
(* Vector: list with length in type *)
Inductive vec (A : Type) : nat -> Type :=
  | vnil : vec A 0
  | vcons : forall n, A -> vec A n -> vec A (S n).

(* Now vec A 3 is a different type than vec A 5! *)
```

**In Rholang Proofs**:
```coq
(* fuel parameter makes type depend on execution steps *)
Fixpoint norm_recursive (t : ProcessTree) (st : NormState) (fuel : nat) : NormState :=
  match fuel with
  | 0 => st  (* Out of fuel, return state *)
  | S fuel' =>
      match t with
      | PPar t1 t2 =>
          norm_recursive t2 (norm_recursive t1 st fuel') fuel'
      | ...
      end
  end.
```

**Why Useful**:
- **Express invariants in types**: "This list has exactly n elements"
- **Prove termination**: fuel decreases on each recursive call
- **Prevent bugs at compile-time**: Type checker verifies properties

---

### Records

**Definition**: Product types with named fields.

```coq
Record Point : Type := {
  x : nat;
  y : nat
}.

(* Access fields by name *)
Definition origin := {| x := 0; y := 0 |}.
Definition get_x (p : Point) := x p.
```

**In Rholang Proofs**:
```coq
Record Par : Type := {
  par_sends : list Send;
  par_receives : list Receive;
  par_news : list New;
  (* ... 5 more fields *)
}.
```

**Benefits over Tuples**:
- **Readability**: `par_sends p` vs `fst (fst (fst p))`
- **Extensibility**: Can add fields without breaking projections
- **Type safety**: Can't confuse field order

---

### Axioms

**Definition**: Assumed truths without proof.

```coq
Axiom functional_extensionality : forall {A B : Type} (f g : A -> B),
  (forall x, f x = g x) -> f = g.
```

**When to Use**:
1. **Property is obvious but tedious to prove**
   - Example: bitmask bijection (128 cases to enumerate)
2. **Requires machinery beyond our scope**
   - Example: Complete Rholang semantics (100+ definitions)
3. **Empirically validated by tests**
   - Example: normalize_atomic correctness (120+ test suite)

**In Rholang Proofs**:
```coq
(* We axiomatize full Rholang semantics *)
Axiom normalize_atomic : ProcessTree -> NormState -> NormState.

(* But prove all optimization equivalences constructively! *)
Theorem norm_recursive_iterative_equiv : ...
Proof. (* 50+ line constructive proof *) Qed.
```

**Trusted Computing Base (TCB)**:
- Axioms form our TCB - must be trusted for proofs to be valid
- Minimize TCB by proving as much as possible
- Validate TCB with extensive testing

---

## Common Proof Patterns

### Pattern 1: Induction on Lists

**Template**:
```coq
Lemma list_property : forall (xs : list A), property xs.
Proof.
  intro xs.
  induction xs as [| x xs' IH].
  - (* Base case: xs = [] *)
    (* Usually: simpl; auto or reflexivity *)
  - (* Inductive case: xs = x :: xs' *)
    (* IH: property xs' *)
    (* Use IH and reason about x *)
Qed.
```

**Example - fold_left_app**:
```coq
Lemma fold_left_app : forall f l1 l2 b,
  fold_left f (l1 ++ l2) b = fold_left f l2 (fold_left f l1 b).
Proof.
  intros f l1.
  induction l1 as [| x xs IH]; intros l2 b; simpl.
  - reflexivity.  (* [] ++ l2 = l2 *)
  - rewrite IH. reflexivity.  (* Use IH for xs *)
Qed.
```

---

### Pattern 2: Case Analysis on Type

**Template**:
```coq
Lemma type_property : forall (t : MyType), property t.
Proof.
  intro t.
  destruct t; (* or: destruct t eqn:E *)
  (* One subgoal per constructor *)
  all: (* common tactic for all cases *).
Qed.
```

**Example - flatten_non_empty**:
```coq
Lemma flatten_non_empty : forall t, flatten t <> [].
Proof.
  intro t.
  induction t; simpl;
  all: try discriminate.  (* All atomic cases: [t] ≠ [] *)
  (* PPar case needs more work *)
  destruct (flatten t1); destruct (flatten t2);
  try discriminate; exfalso; congruence.
Qed.
```

---

### Pattern 3: Arithmetic Bounds

**Template**:
```coq
Lemma bound_property : forall n, lower_bound <= f n <= upper_bound.
Proof.
  intro n.
  (* Often: apply lemma, then lia *)
  assert (H: key_equation).
  { (* Prove key equation *) }
  (* Use H with lia *)
  rewrite H. lia.
Qed.
```

**Example - sum_n_quadratic**:
```coq
Lemma sum_n_quadratic : forall n,
  n * n <= 2 * sum_n n <= 2 * n * n.
Proof.
  intro n.
  rewrite sum_n_formula.  (* 2 * sum_n n = n * (n+1) *)
  split.
  - destruct n; simpl; lia.  (* Lower bound *)
  - destruct n; simpl; lia.  (* Upper bound *)
Qed.
```

---

### Pattern 4: Equivalence via Transitivity

**Template**:
```coq
Theorem A_equals_C : A = C.
Proof.
  transitivity B.
  - (* Prove: A = B *)
  - (* Prove: B = C *)
Qed.
```

**Example - fuel_adequate (from Proof01)**:
```coq
(* Goal: norm t st fuel = norm t st (2*size t) *)
transitivity (norm t2 (norm t1 st (2*size t1)) (2*size t2)).
* (* LHS = middle: normalize individual fuels *)
  apply IHt2.
* (* middle = RHS: both use canonical fuel *)
  (* ... more steps *)
```

**When to Use**:
- Direct proof is hard (different fuel values)
- Have natural intermediate form (canonical fuel)
- Can prove two "easier" equalities instead

---

### Pattern 5: Existential Witness

**Template**:
```coq
Lemma exists_property : forall x, exists y, relation x y.
Proof.
  intro x.
  exists (compute_witness x).  (* Provide witness *)
  (* Prove: relation x (compute_witness x) *)
Qed.
```

**Example - lazy_iterator_O1_space**:
```coq
Theorem lazy_iterator_O1_space : forall mask,
  mask < 128 -> exists space, space = 1.
Proof.
  intros mask H.
  exists 1.  (* Witness: constant space *)
  reflexivity.  (* Prove: 1 = 1 *)
Qed.
```

---

## Learning Exercises

### Exercise 1: Basic List Lemma ⭐

**Task**: Prove that list length is preserved by reversal.

```coq
Lemma rev_length : forall {A : Type} (xs : list A),
  length (rev xs) = length xs.
Proof.
  (* TODO: Prove by induction on xs *)
  (* Hint: You'll need rev_app_distr for the inductive case *)
Admitted.
```

<details>
<summary>Solution</summary>

```coq
Lemma rev_length : forall {A : Type} (xs : list A),
  length (rev xs) = length xs.
Proof.
  intros A xs.
  induction xs as [| x xs' IH].
  - (* Base: rev [] = [] *)
    simpl. reflexivity.
  - (* Inductive: rev (x :: xs') = rev xs' ++ [x] *)
    simpl.
    rewrite app_length.  (* length (a ++ b) = length a + length b *)
    rewrite IH.
    simpl. lia.
Qed.
```
</details>

---

### Exercise 2: Arithmetic Bounds ⭐⭐

**Task**: Prove that sum_n grows at least linearly.

```coq
Lemma sum_n_linear_lower : forall n,
  n <= sum_n n.
Proof.
  (* TODO: Prove by induction *)
  (* Hint: sum_n (S n) = S n + sum_n n *)
Admitted.
```

<details>
<summary>Solution</summary>

```coq
Lemma sum_n_linear_lower : forall n,
  n <= sum_n n.
Proof.
  intro n.
  induction n as [| n' IH].
  - (* Base: 0 ≤ 0 *)
    simpl. lia.
  - (* Inductive: S n' ≤ sum_n (S n') *)
    simpl.  (* sum_n (S n') = S n' + sum_n n' *)
    (* By IH: n' ≤ sum_n n' *)
    (* So: S n' ≤ S n' + sum_n n' *)
    lia.
Qed.
```
</details>

---

### Exercise 3: fold_left Behavior ⭐⭐

**Task**: Prove that fold_left with addition sums list elements.

```coq
Lemma fold_left_sum : forall (xs : list nat) (acc : nat),
  fold_left (fun a x => a + x) xs acc = acc + fold_left (fun a x => a + x) xs 0.
Proof.
  (* TODO: Prove by induction on xs *)
  (* Hint: You'll need lia for arithmetic *)
Admitted.
```

<details>
<summary>Solution</summary>

```coq
Lemma fold_left_sum : forall (xs : list nat) (acc : nat),
  fold_left (fun a x => a + x) xs acc = acc + fold_left (fun a x => a + x) xs 0.
Proof.
  intro xs.
  induction xs as [| x xs' IH]; intro acc; simpl.
  - (* Base: fold [] acc = acc, fold [] 0 = 0 *)
    lia.
  - (* Inductive: fold (x :: xs') acc = acc + fold (x :: xs') 0 *)
    rewrite IH.
    rewrite (IH x).
    lia.
Qed.
```
</details>

---

### Exercise 4: Monad Left Identity ⭐⭐⭐

**Task**: Prove the monad left identity law for the option monad.

```coq
Definition option_bind {A B : Type} (ma : option A) (f : A -> option B) : option B :=
  match ma with
  | None => None
  | Some a => f a
  end.

Definition option_return {A : Type} (a : A) : option A := Some a.

Lemma option_monad_left_id : forall {A B : Type} (a : A) (f : A -> option B),
  option_bind (option_return a) f = f a.
Proof.
  (* TODO: Unfold definitions and simplify *)
Admitted.
```

<details>
<summary>Solution</summary>

```coq
Lemma option_monad_left_id : forall {A B : Type} (a : A) (f : A -> option B),
  option_bind (option_return a) f = f a.
Proof.
  intros A B a f.
  unfold option_bind, option_return.
  (* Goal: match Some a with None => ... | Some a => f a end = f a *)
  simpl.
  (* Goal: f a = f a *)
  reflexivity.
Qed.
```
</details>

---

### Exercise 5: Custom Induction ⭐⭐⭐⭐

**Task**: Prove properties of a custom tree type.

```coq
Inductive Tree (A : Type) : Type :=
  | Leaf : A -> Tree A
  | Node : Tree A -> Tree A -> Tree A.

Fixpoint tree_size {A : Type} (t : Tree A) : nat :=
  match t with
  | Leaf _ => 1
  | Node l r => 1 + tree_size l + tree_size r
  end.

Lemma tree_size_positive : forall {A : Type} (t : Tree A),
  tree_size t >= 1.
Proof.
  (* TODO: Prove by induction on t *)
  (* Hint: Similar to ProcessTree proofs *)
Admitted.
```

<details>
<summary>Solution</summary>

```coq
Lemma tree_size_positive : forall {A : Type} (t : Tree A),
  tree_size t >= 1.
Proof.
  intros A t.
  induction t as [a | l IHl r IHr].
  - (* Leaf case: size = 1 *)
    simpl. lia.
  - (* Node case: size = 1 + size l + size r *)
    simpl.
    (* By IH: size l ≥ 1, size r ≥ 1 *)
    (* Therefore: 1 + size l + size r ≥ 1 + 1 + 1 ≥ 1 *)
    lia.
Qed.
```
</details>

---

### Exercise 6: From Phase 2 Proofs ⭐⭐⭐

**Task**: Prove that Rc cloning multiple times maintains the value.

```coq
From Rholang Require Import Proof02_RcSharing.

Lemma rc_clone_preserves_value : forall {A : Type} (rc : RcPtr A),
  rc_value (rc_clone rc) = rc_value rc.
Proof.
  (* TODO: Unfold definitions and prove *)
  (* Hint: rc_clone doesn't change rc_value field *)
Admitted.
```

<details>
<summary>Solution</summary>

```coq
Lemma rc_clone_preserves_value : forall {A : Type} (rc : RcPtr A),
  rc_value (rc_clone rc) = rc_value rc.
Proof.
  intros A rc.
  unfold rc_clone.
  (* Goal: rc_value {| rc_value := ...; rc_refcount := ... |} = rc_value rc *)
  simpl.
  (* Goal: rc_value rc = rc_value rc *)
  reflexivity.
Qed.
```

**What This Teaches**: Records preserve field values even when other fields change.
</details>

---

### Exercise 7: Vec Capacity Independence ⭐⭐⭐

**Task**: Prove that two Vecs with same elements are equal (ignoring capacity).

```coq
From Rholang Require Import Proof03_PreAllocation.

Lemma vec_elements_equality : forall {A : Type} (v1 v2 : Vec A),
  vec_elements v1 = vec_elements v2 ->
  forall (f : list A -> nat),
  f (vec_elements v1) = f (vec_elements v2).
Proof.
  (* TODO: Use rewrite with hypothesis *)
Admitted.
```

<details>
<summary>Solution</summary>

```coq
Lemma vec_elements_equality : forall {A : Type} (v1 v2 : Vec A),
  vec_elements v1 = vec_elements v2 ->
  forall (f : list A -> nat),
  f (vec_elements v1) = f (vec_elements v2).
Proof.
  intros A v1 v2 Heq f.
  rewrite Heq.
  reflexivity.
Qed.
```

**What This Teaches**: Capacity is truly independent - any function on elements treats equal element lists identically.
</details>

---

### Exercise 8: Triple Reverse ⭐⭐

**Task**: Prove that reversing a list three times equals one reverse.

```coq
Lemma triple_reverse : forall {A : Type} (xs : list A),
  rev (rev (rev xs)) = rev xs.
Proof.
  (* TODO: Use involution property twice *)
  (* Hint: You'll need rev_involutive from stdlib *)
Admitted.
```

<details>
<summary>Solution</summary>

```coq
Lemma triple_reverse : forall {A : Type} (xs : list A),
  rev (rev (rev xs)) = rev xs.
Proof.
  intros A xs.
  (* Apply involution to inner rev (rev xs) *)
  rewrite <- (rev_involutive xs) at 2.
  (* Now: rev (rev (rev xs)) = rev (rev (rev (rev xs))) *)
  (* Apply involution again *)
  rewrite rev_involutive.
  reflexivity.
Qed.
```

**Alternate Solution** (more direct):
```coq
Lemma triple_reverse : forall {A : Type} (xs : list A),
  rev (rev (rev xs)) = rev xs.
Proof.
  intros A xs.
  (* Group as rev ((rev (rev xs))) *)
  rewrite rev_involutive.
  (* Now: rev xs = rev xs *)
  reflexivity.
Qed.
```

**What This Teaches**: Involutions cancel in pairs - odd number of applications = one application.
</details>

---

## Proof Strategy Decision Tree

When you're stuck on a proof, follow this decision tree:

```
START: Look at your goal
    ↓
Is it X = X (syntactically identical)?
    YES → reflexivity
    NO → Continue
    ↓
Is it arithmetic (≤, <, +, *, etc.)?
    YES → Try lia
    NO → Continue
    ↓
Does it use ∀ (forall)?
    YES → intros (bring variables into context)
    → Continue with new goal
    NO → Continue
    ↓
Is it about a list/tree/inductive type?
    YES → Is it a property for ALL values?
        YES → induction xs as [| x xs' IH]
        NO → destruct xs (just case analysis)
    NO → Continue
    ↓
Can you find a lemma that matches?
    YES → Search for it:
        - SearchAbout term
        - Search pattern
        Then: apply lemma
    NO → Continue
    ↓
Is the goal complicated? Can you simplify?
    YES → Try these in order:
        1. simpl (compute one step)
        2. unfold definition (expand definitions)
        3. rewrite H (use equality H)
    NO → Continue
    ↓
Do you have a hypothesis H : A = B?
    YES → rewrite H (or rewrite <- H)
    → Simplifies goal using equality
    NO → Continue
    ↓
Is it an existential (∃ x, P x)?
    YES → exists witness
    → Provide concrete value
    → Prove P(witness)
    NO → Continue
    ↓
Are you proving A = C but it's hard?
    YES → Can you find middle term B?
        YES → transitivity B
            - Prove A = B
            - Prove B = C
        NO → Continue
    NO → Continue
    ↓
Still stuck?
    → assert (H : helper_fact)
    → Prove helper fact
    → Use H to prove main goal
    OR
    → Admit it for now, come back later
    → Focus on understanding WHAT to prove
```

### Example: Applying the Tree

**Goal**: `fold_left f (l1 ++ l2) b = fold_left f l2 (fold_left f l1 b)`

1. Is it X = X? NO
2. Is it arithmetic? NO
3. Does it use ∀? Not in current goal form
4. Is it about lists? YES!
   - Property for ALL l1? YES!
   - **Decision**: `induction l1 as [| x xs IH]`

5. After induction:
   - Base case: `fold_left f ([] ++ l2) b = fold_left f l2 (fold_left f [] b)`
   - Is it X = X after simpl? YES! → `simpl. reflexivity.`

   - Inductive case: `fold_left f ((x :: xs) ++ l2) b = ...`
   - Can simplify? YES! → `simpl.`
   - Have equality IH? YES! → `rewrite IH. reflexivity.`

**Result**: Proof complete! ✓

---

## File-by-File Learning Path

### Beginner Path (Start Here!) 🌱

#### File 1: Proof05_MatchOptimization.v
**Difficulty**: ⭐ (Easiest!)
**Time**: 30 minutes
**What You'll Learn**:
- How a complete proof looks
- The `apply` tactic
- Using standard library lemmas
- Involution property

**Key Takeaway**: Many proofs are just one line: `apply existing_lemma`!

---

#### File 2: Proof02_RcSharing.v
**Difficulty**: ⭐⭐
**Time**: 1-2 hours
**What You'll Learn**:
- Records (RcPtr)
- unfold and simpl tactics
- Semantic transparency proofs
- Reference counting model

**Key Sections**:
- Lines 220-231: Borrow semantic equivalence (short proof!)
- Lines 375-472: Main theorem with detailed steps
- Lines 523-636: Complexity improvement proof

**Try This**: After reading, try to prove rc_clone preserves some property

---

#### File 3: RholangLemmas.v (Selected Lemmas)
**Difficulty**: ⭐⭐
**Time**: 2-3 hours
**Focus On These Lemmas**:
1. `fold_left_app` (THE KEY LEMMA) - lines 47-62
   - List induction pattern
   - How to use IH (inductive hypothesis)

2. `sum_n_formula` - lines 72-94
   - Arithmetic induction
   - Using lia for calculations

3. `in_remove_all` - lines 306-330
   - Boolean decidability
   - Case analysis with destruct

**Skip for now**: State monad section (advanced)

---

### Intermediate Path 🌿

#### File 4: Proof03_PreAllocation.v
**Difficulty**: ⭐⭐⭐
**Time**: 3-4 hours
**What You'll Learn**:
- Helper lemmas with assert
- Induction inside proof blocks
- Vec/capacity reasoning
- Amortized analysis

**Key Sections**:
- Lines 431-518: vec_push_elements_independent (great induction example!)
- Lines 570-762: Main theorem with helper lemma pattern

**Challenge**: Understand WHY we need the helper lemma (generalization!)

---

#### File 5: Proof06_LazySubPars.v
**Difficulty**: ⭐⭐⭐
**Time**: 2-3 hours
**What You'll Learn**:
- Bitmask encoding
- Existential proofs (∃)
- Bijection properties
- Axiomatization strategy

**Key Sections**:
- Lines 147-163: extract_subset function
- Lines 237-246: lazy_iterator_O1_space (simple existential!)
- Lines 293-296: Axiom example (when to axiomatize)

---

#### File 6: Proof04_AccumulatorPattern.v
**Difficulty**: ⭐⭐⭐
**Time**: 3-4 hours
**What You'll Learn**:
- Gauss's formula application
- Complexity analysis
- acc_sum_correct pattern (read full file from previous doc)

**Why This Matters**: Real-world optimization proof (6,158× speedup!)

---

### Advanced Path 🌲

#### File 7: Proof01_ParFlattening.v
**Difficulty**: ⭐⭐⭐⭐ (Most Complex!)
**Time**: 6-8 hours
**What You'll Learn**:
- Fuel adequacy reasoning
- Transitivity chains
- Multiple nested inductions
- Complex case analysis

**Approach**:
1. Read header (lines 1-130) - understand the problem
2. Read fuel_adequate_once (lines 178-212) - simplest fuel lemma
3. Read fuel_adequate (lines 214-287) - understand generalization
4. Read main theorem (lines 461-554) - see it all come together

**This is the BOSS LEVEL proof**. Don't feel bad if it takes multiple sessions!

---

#### File 8: RholangCore.v
**Difficulty**: ⭐⭐⭐⭐
**Time**: 4-6 hours
**What You'll Learn**:
- Deep embeddings
- Inductive type design
- Axiomatization strategy
- Trusted Computing Base

**Not really a "proof file"**, but understanding the foundations is crucial for advanced work.

---

#### File 9: Proof07_CloneReduction.v
**Difficulty**: ⭐⭐⭐⭐ (Conceptually)
**Time**: 2-3 hours
**What You'll Learn**:
- When axiomatization is necessary
- Ownership reasoning
- Separation logic concepts
- TCB trade-offs

**Key Insight**: Sometimes full verification requires tools beyond Coq (RustBelt/Iris)

---

### Skill Progression Chart

```
Week 1: Proof05 → Proof02 → RholangLemmas (selected)
  Skills: apply, simpl, reflexivity, basic induction
  ↓
Week 2-3: Proof03 → Proof06
  Skills: assert, helper lemmas, existentials, generalizing IH
  ↓
Week 4: Proof04
  Skills: arithmetic reasoning, complexity proofs
  ↓
Week 5-6: Proof01 (The Boss!)
  Skills: transitivity, fuel reasoning, complex case analysis
  ↓
Week 7+: RholangCore, Proof07 (Foundations)
  Skills: axiomatization, TCB reasoning, deep embeddings
```

---

## Debugging Guide

### Common Errors and Solutions

#### Error: "Unable to unify X with Y"

**Cause**: Type mismatch - Coq can't make the types match.

**Solutions**:
1. Check types with `Check expr.`
2. Use `simpl` to reduce types
3. Add type annotations: `(expr : Type)`

**Example**:
```coq
(* Error: Unable to unify "list nat" with "nat" *)
(* You wrote: *)
Lemma bad : forall xs, xs = 0.

(* Should be: *)
Lemma good : forall xs, length xs = 0 -> xs = [].
```

---

#### Error: "Not enough information to infer"

**Cause**: Coq can't figure out implicit type arguments.

**Solutions**:
1. Make arguments explicit: `@lemma Type arg1 arg2`
2. Add type hints: `([] : list nat)`

**Example**:
```coq
(* Error: Can't infer type of [] *)
exists [].

(* Fix with type annotation *)
exists (@nil nat).
(* or *)
exists ([] : list nat).
```

---

#### Tactic Fails: "No applicable tactic"

**Debugging Steps**:
1. `Show Proof.` - See proof term so far
2. `Undo.` - Go back one step
3. Try smaller steps instead of automation

**Example**:
```coq
(* lia fails *)
Goal: x + 1 > x.

(* Debug: what type is x? *)
Check x.  (* x : Z - but goal needs nat! *)

(* Fix: convert or use different tactic *)
```

---

#### Induction Not Strong Enough

**Symptom**: Inductive hypothesis doesn't help.

**Solution**: Generalize before induction.

**Example**:
```coq
(* Bad: IH is too specific *)
Lemma bad : forall xs, property xs 0.
Proof.
  intro xs.
  induction xs.  (* IH: property xs' 0 - can't vary 0! *)

(* Good: generalize first *)
Lemma good : forall xs n, property xs n.
Proof.
  intros xs n.
  generalize dependent n.  (* Make n variable *)
  induction xs.  (* IH: ∀ n, property xs' n - much stronger! *)
```

---

### Useful Debugging Commands

```coq
Show.              (* Show current goal *)
Show Proof.        (* Show proof term constructed *)
Check expr.        (* Show type of expression *)
Print lemma.       (* Show definition/proof of lemma *)
Search pattern.    (* Find lemmas matching pattern *)
SearchAbout term.  (* Find lemmas about term *)
Locate symbol.     (* Find definition of notation *)
```

---

## Further Reading

### Beginner Resources

1. **Software Foundations (Volume 1: Logical Foundations)**
   - https://softwarefoundations.cis.upenn.edu/
   - Start here! Free, interactive, excellent
   - Covers: basic tactics, induction, lists, logic

2. **Coq'Art (Book)**
   - "Interactive Theorem Proving and Program Development"
   - Comprehensive reference
   - Good for looking up specific topics

3. **Official Coq Documentation**
   - https://coq.inria.fr/documentation
   - Reference manual and tutorial

### Intermediate Resources

4. **Software Foundations (Volume 2: Programming Language Foundations)**
   - https://softwarefoundations.cis.upenn.edu/plf-current/
   - Covers: operational semantics, type systems, Hoare logic
   - Directly relevant to compiler verification!

5. **Certified Programming with Dependent Types**
   - http://adam.chlipala.net/cpdt/
   - By Adam Chlipala
   - More advanced tactics and proof automation

### Advanced Resources

6. **Software Foundations (Volume 3: Verified Functional Algorithms)**
   - https://softwarefoundations.cis.upenn.edu/vfa-current/
   - Data structures, efficiency proofs
   - Similar to our complexity proofs!

7. **Formal Reasoning About Programs**
   - http://adam.chlipala.net/frap/
   - Adam Chlipala's course
   - Concurrency, separation logic

8. **CompCert Documentation**
   - https://compcert.org/doc/
   - Real-world verified compiler
   - Shows how axioms are used in practice

### Related Topics

9. **Type Theory Foundations**
   - "Type Theory and Formal Proof" by Geuvers
   - Mathematical foundations

10. **Process Calculus**
    - "The Pi-Calculus: A Theory of Mobile Processes"
    - Background for understanding Rholang

---

## Quick Command Reference

### Starting Coq

```bash
# Compile a file
coqc MyFile.v

# Interactive mode
coqide MyFile.v  # GUI
coqtop          # REPL

# With dependencies
coqc -R . MyProject MyFile.v
```

### Common Workflow

```coq
(** In .v file **)

(* 1. State theorem *)
Theorem my_theorem : statement.
Proof.

(* 2. Explore goal *)
Show.           (* What do I need to prove? *)

(* 3. Try automatic tactics *)
try reflexivity.
try lia.
try auto.

(* 4. Manual steps if needed *)
intros.
induction ...
destruct ...
apply ...
rewrite ...

(* 5. Finish *)
Qed.

(* Check proof *)
Print my_theorem.
```

---

**Last Updated**: 2025-11-11
**Coq Version**: Rocq Prover 9.1.0
**Companion to**: DOCUMENTATION_SUMMARY.md
