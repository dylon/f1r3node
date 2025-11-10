(** * Proof 5: Match Optimization (Double Reverse Elimination) *)

From Stdlib Require Import Lists.List.
From Stdlib Require Import Lia.
From Rholang Require Import RholangCore.
From Rholang Require Import RholangLemmas.

Theorem double_reverse_identity : forall {A : Type} (xs : list A),
  rev (rev xs) = xs.
Proof. apply rev_involutive. Qed.

Theorem match_optimization_sound : forall (cases : list (Expr * ProcessTree)),
  rev (rev cases) = cases.
Proof. apply rev_involutive. Qed.

(** End of Proof05_MatchOptimization.v *)
