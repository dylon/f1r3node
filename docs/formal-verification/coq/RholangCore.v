(** * Rholang Core Definitions for Optimization Proofs

    This file contains the core type definitions and functions for formalizing
    the Rholang interpreter optimization proofs.

    Based on: /var/tmp/debug/f1r3node/docs/performance/optimization-equivalence-proofs.md

    Key structures:
    - ProcessTree: Recursive process structure with Par nodes
    - Par: 7-tuple representing parallel composition components
    - State: Normalization state (Par accumulator, FreeMap, BoundMapChain)
    - Name: Channel names with sorts (unforgeable, wildcard, etc.)
*)

From Stdlib Require Import Lists.List.
From Stdlib Require Import Arith.Arith.
From Stdlib Require Import Bool.Bool.
From Stdlib Require Import Strings.String.
From Stdlib Require Import Init.Nat.
From Stdlib Require Import Lia.
Import ListNotations.

(** ** Name System *)

(** Name sorts for channel names *)
Inductive NameSort : Type :=
  | Unforgeable : nat -> NameSort    (* Cryptographically unique names *)
  | Wildcard : NameSort               (* Anonymous wildcard names *)
  | Bound : nat -> NameSort           (* De Bruijn indexed bound names *)
  | Free : string -> NameSort.        (* Free variables *)

(** Channel names *)
Record Name : Type := mkName {
  name_sort : NameSort;
  name_id : nat                       (* Unique identifier *)
}.

(** ** Variable Maps *)

(** Simplified map representation - actual implementation uses persistent HashMap *)
Definition Map (K V : Type) := list (K * V).

Definition empty_map {K V : Type} : Map K V := [].

Fixpoint map_get {K V : Type} (eq_dec : forall x y : K, {x = y} + {x <> y})
                 (m : Map K V) (k : K) : option V :=
  match m with
  | [] => None
  | (k', v') :: rest =>
      if eq_dec k k' then Some v' else map_get eq_dec rest k
  end.

Definition map_add {K V : Type} (m : Map K V) (k : K) (v : V) : Map K V :=
  (k, v) :: m.

(** Variable sort for De Bruijn indices *)
Inductive VarSort : Type :=
  | ProcessSort
  | NameVarSort
  | ExprSort.

(** FreeMap: Maps free variable names to De Bruijn indices *)
Definition FreeMap := Map Name VarSort.

(** BoundMap: Maps bound variable indices to sorts *)
Definition BoundMap := Map nat VarSort.

(** BoundMapChain: Stack of bound variable scopes *)
Definition BoundMapChain := list BoundMap.

(** ** Process Expressions *)

(** Simplified expression type *)
Inductive Expr : Type :=
  | EVar : Name -> Expr
  | EGround : nat -> Expr              (* Ground values (numbers, bools, etc.) *)
  | EList : list Expr -> Expr
  | EAdd : Expr -> Expr -> Expr.

(** ** Process Tree (Inductive type with mutual recursion) *)

Inductive ProcessTree : Type :=
  | PNil : ProcessTree                              (* Empty process *)
  | PSend : Name -> list Expr -> bool -> ProcessTree  (* Send: chan, data, persistent *)
  | PReceive : list Name -> list Expr -> ProcessTree -> bool -> ProcessTree
      (* Receive: chans, patterns, body, persistent *)
  | PNew : nat -> ProcessTree -> ProcessTree        (* New: count, body *)
  | PMatch : Expr -> list (Expr * ProcessTree) -> ProcessTree
      (* Match: target, cases *)
  | PBundle : ProcessTree -> bool -> ProcessTree
      (* Bundle: body, read_write flag *)
  | PExpr : Expr -> ProcessTree                     (* Expression as process *)
  | PPar : ProcessTree -> ProcessTree -> ProcessTree. (* Parallel composition *)

(** Helper record types for structured access *)
Record Send : Type := mkSend {
  send_chan : Name;
  send_data : list Expr;
  send_persistent : bool
}.

Record Receive : Type := mkReceive {
  receive_chans : list Name;
  receive_patterns : list Expr;
  receive_body : ProcessTree;
  receive_persistent : bool
}.

Record New : Type := mkNew {
  new_count : nat;
  new_body : ProcessTree
}.

Record Match : Type := mkMatch {
  match_target : Expr;
  match_cases : list (Expr * ProcessTree)
}.

Record Bundle : Type := mkBundle {
  bundle_body : ProcessTree;
  bundle_read_write : bool
}.

(** ** Par Structure (7-tuple) *)

(** The accumulated Par structure during normalization.

    Components (using mathematical notation from the proof document):
    - 𝕊 (sends): Vec<Send>
    - ℝ (receives): Vec<Receive>
    - 𝕹 (news): Vec<New>
    - 𝔼 (exprs): Vec<Expr>
    - 𝕄 (matches): Vec<Match>
    - 𝕌 (unforgeables/bundles): Vec<Bundle>
    - 𝔹 (bundles): Vec<Bundle> (kept separate in actual impl)
*)
Record Par : Type := mkPar {
  par_sends : list Send;
  par_receives : list Receive;
  par_news : list New;
  par_exprs : list Expr;
  par_matches : list Match;
  par_bundles : list Bundle;
  par_connective_used : bool;           (* Logical connective flag *)
  par_locally_free : nat                (* Bitset of locally free variables *)
}.

(** Empty Par *)
Definition empty_par : Par := {|
  par_sends := [];
  par_receives := [];
  par_news := [];
  par_exprs := [];
  par_matches := [];
  par_bundles := [];
  par_connective_used := false;
  par_locally_free := 0
|}.

(** ** Normalization State *)

(** State passed through normalization process.

    Corresponds to σ = (par, ℱ, 𝓑) in the proof document:
    - par: Accumulated Par structure
    - ℱ (free_map): Free variable map
    - 𝓑 (bound_map_chain): Bound variable scope chain
*)
Record NormState : Type := mkNormState {
  state_par : Par;
  state_free_map : FreeMap;
  state_bound_map_chain : BoundMapChain
}.

(** Initial state *)
Definition init_state : NormState := {|
  state_par := empty_par;
  state_free_map := empty_map;
  state_bound_map_chain := []
|}.

(** ** Tree Operations *)

(** Check if a process is atomic (not a Par node) *)
Definition is_atomic (p : ProcessTree) : bool :=
  match p with
  | PPar _ _ => false
  | _ => true
  end.

(** Tree size (number of nodes) *)
Fixpoint tree_size (t : ProcessTree) : nat :=
  match t with
  | PNil => 1
  | PSend _ _ _ => 1
  | PReceive _ _ body _ => 1 + tree_size body
  | PNew _ body => 1 + tree_size body
  | PMatch _ cases => 1 + fold_left (fun acc '(_, body) => acc + tree_size body) cases 0
  | PBundle body _ => 1 + tree_size body
  | PExpr _ => 1
  | PPar t1 t2 => 1 + tree_size t1 + tree_size t2
  end.

(** Tree depth (maximum path length from root to leaf) *)
Fixpoint tree_depth (t : ProcessTree) : nat :=
  match t with
  | PNil => 0
  | PSend _ _ _ => 0
  | PReceive _ _ body _ => 1 + tree_depth body
  | PNew _ body => 1 + tree_depth body
  | PMatch _ cases => 1 + fold_left (fun acc '(_, body) => Nat.max acc (tree_depth body)) cases 0
  | PBundle body _ => 1 + tree_depth body
  | PExpr _ => 0
  | PPar t1 t2 => 1 + Nat.max (tree_depth t1) (tree_depth t2)
  end.

(** ** Normalization Functions *)

(** Placeholder for atomic process normalization.

    This function normalizes a single atomic (non-Par) process by:
    1. Processing the process structure
    2. Updating the Par accumulator
    3. Updating free/bound variable maps
    4. Returning the new state

    NOTE: Full implementation requires significant additional machinery
    (pattern matching, substitution, name resolution, etc.)
*)
Axiom normalize_atomic : ProcessTree -> NormState -> NormState.

(** Axiom: normalize_atomic only accepts atomic processes *)
Axiom normalize_atomic_requires_atomic : forall p st,
  is_atomic p = true ->
  exists st', normalize_atomic p st = st'.

(** ** Flattening Functions *)

(** Flatten a process tree into a list of atomic processes (DFS left-to-right).

    This is the core function for iterative Par normalization (Proof 1).
    Converts nested Par(Par(...), Par(...)) structure into flat list of atomic processes.
*)
Fixpoint flatten (t : ProcessTree) : list ProcessTree :=
  match t with
  | PPar t1 t2 => flatten t1 ++ flatten t2
  | _ => [t]  (* Atomic process *)
  end.

(** Flatten using explicit stack (closer to actual Rust implementation).

    Uses an explicit stack (worklist) to avoid recursive calls.
    This is the implementation used in commit f5219577 to eliminate stack overflow.
*)
Fixpoint flatten_stack_aux (stack : list ProcessTree) (acc : list ProcessTree)
                           (fuel : nat) : list ProcessTree :=
  match fuel with
  | 0 => acc  (* Fuel exhausted - should never happen with proper fuel *)
  | S fuel' =>
      match stack with
      | [] => List.rev acc  (* Stack empty - done *)
      | current :: rest_stack =>
          match current with
          | PPar t1 t2 =>
              (* Push right then left (so left is processed first) *)
              flatten_stack_aux (t2 :: t1 :: rest_stack) acc fuel'
          | _ =>
              (* Atomic process - add to accumulator *)
              flatten_stack_aux rest_stack (current :: acc) fuel'
          end
      end
  end.

Definition flatten_stack (t : ProcessTree) : list ProcessTree :=
  (* Fuel = 2 * tree_size to guarantee termination *)
  flatten_stack_aux [t] [] (2 * tree_size t).

(** ** Semantic Evaluation *)

(** Recursive normalization (original implementation).

    Corresponds to ⟦T⟧ᵣ(σ₀) in the proof document.
    Processes Par nodes recursively (left child, then right child).
*)
Fixpoint norm_recursive (t : ProcessTree) (st : NormState) (fuel : nat) : NormState :=
  match fuel with
  | 0 => st  (* Fuel exhausted *)
  | S fuel' =>
      match t with
      | PPar t1 t2 =>
          let st1 := norm_recursive t1 st fuel' in
          norm_recursive t2 st1 fuel'
      | _ => normalize_atomic t st
      end
  end.

(** Iterative normalization (optimized implementation).

    Corresponds to ⟦T⟧ᵢ(σ₀) in the proof document.
    Flattens tree first, then folds over atomic processes.
*)
Definition norm_iterative (t : ProcessTree) (st : NormState) : NormState :=
  fold_left (fun state proc => normalize_atomic proc state) (flatten t) st.

(** ** Utility Functions *)

(** Prepend elements to Par component *)
Definition prepend_sends (p : Par) (sends : list Send) : Par :=
  {| par_sends := sends ++ par_sends p;
     par_receives := par_receives p;
     par_news := par_news p;
     par_exprs := par_exprs p;
     par_matches := par_matches p;
     par_bundles := par_bundles p;
     par_connective_used := par_connective_used p;
     par_locally_free := par_locally_free p |}.

Definition prepend_receives (p : Par) (receives : list Receive) : Par :=
  {| par_sends := par_sends p;
     par_receives := receives ++ par_receives p;
     par_news := par_news p;
     par_exprs := par_exprs p;
     par_matches := par_matches p;
     par_bundles := par_bundles p;
     par_connective_used := par_connective_used p;
     par_locally_free := par_locally_free p |}.

Definition prepend_exprs (p : Par) (exprs : list Expr) : Par :=
  {| par_sends := par_sends p;
     par_receives := par_receives p;
     par_news := par_news p;
     par_exprs := exprs ++ par_exprs p;
     par_matches := par_matches p;
     par_bundles := par_bundles p;
     par_connective_used := par_connective_used p;
     par_locally_free := par_locally_free p |}.

(** ** Decidable Equality *)

(** Placeholder for decidable equality on ProcessTree.

    NOTE: Full implementation requires decidability for all component types
    (Name, Expr, Send, Receive, etc.)
*)
Axiom ProcessTree_eq_dec : forall (t1 t2 : ProcessTree), {t1 = t2} + {t1 <> t2}.
Axiom Par_eq_dec : forall (p1 p2 : Par), {p1 = p2} + {p1 <> p2}.
Axiom NormState_eq_dec : forall (s1 s2 : NormState), {s1 = s2} + {s1 <> s2}.

(** ** Basic Lemmas *)

(** Atomic processes have size 1 (or slightly more for nested structures) *)
Lemma atomic_size_bound : forall p,
  is_atomic p = true ->
  tree_size p >= 1.
Proof.
  intros p H.
  destruct p; simpl in *; try lia; discriminate H.
Qed.

(** Par nodes have size >= 3 (1 for Par node + at least 1 for each child) *)
Lemma par_size_bound : forall t1 t2,
  tree_size (PPar t1 t2) >= 3.
Proof.
  intros.
  simpl.
  (* Each subtree has size >= 1 *)
  assert (tree_size t1 >= 1) by (destruct t1; simpl; lia).
  assert (tree_size t2 >= 1) by (destruct t2; simpl; lia).
  lia.
Qed.

(** Flatten produces non-empty list for non-empty tree *)
Lemma flatten_non_empty : forall t,
  flatten t <> [].
Proof.
  intros t.
  induction t; simpl; try discriminate.
  (* PPar case *)
  destruct (flatten t1) eqn:Ht1; destruct (flatten t2) eqn:Ht2.
  - (* Both empty - contradiction *)
    exfalso.
    specialize (IHt1 eq_refl).
    assumption.
  - (* t1 empty, t2 non-empty *)
    simpl. discriminate.
  - (* t1 non-empty, t2 empty *)
    rewrite app_nil_r. discriminate.
  - (* Both non-empty *)
    discriminate.
Qed.

(** Flattening atomic process returns singleton *)
Lemma flatten_atomic_singleton : forall p,
  is_atomic p = true ->
  flatten p = [p].
Proof.
  intros p H.
  destruct p; simpl in *; try reflexivity; discriminate H.
Qed.

(** Flattening preserves tree size (number of atomic processes) *)
Lemma flatten_length_bound : forall t,
  List.length (flatten t) <= tree_size t.
Proof.
  intro t.
  induction t; simpl; auto with arith.
  (* PPar case *)
  rewrite List.length_app.
  lia.
Qed.

(** End of RholangCore.v *)
