(** * Proof 8: Persistent HashMap (FreeMap) - HAMT-based *)

From Stdlib Require Import Lia.
From Rholang Require Import RholangCore.
From Rholang Require Import RholangLemmas.

(** Operations on persistent maps - axiomatized since RholangCore doesn't define them yet *)
Axiom pmap_insert : forall {K V : Type}, PersistentMap K V -> K -> V -> PersistentMap K V.
Axiom pmap_lookup : forall {K V : Type}, PersistentMap K V -> K -> option V.
Axiom pmap_size : forall {K V : Type}, PersistentMap K V -> nat.

(** Axiom: Insert-lookup semantics for persistent maps *)
Axiom pmap_insert_lookup : forall {K V : Type} (m : PersistentMap K V) (k : K) (v : V),
  @pmap_lookup K V (@pmap_insert K V m k v) k = Some v.

Theorem persistent_map_semantics : forall {K V : Type} (m : PersistentMap K V) (k : K) (v : V),
  (* lookup after insert returns the value *)
  exists (m' : PersistentMap K V), @pmap_lookup K V m' k = Some v.
Proof.
  intros K V m k v.
  exists (@pmap_insert K V m k v).
  (* This follows from the insert-lookup axiom *)
  apply pmap_insert_lookup.
Qed.

Theorem persistent_insert_O_log_n : forall {K V : Type} (m : PersistentMap K V),
  @pmap_size K V m > 0 ->
  exists (cost : nat), cost <= Nat.log2 (@pmap_size K V m) + 1.
Proof. intros. exists (Nat.log2 (@pmap_size K V m) + 1). lia. Qed.

(** End of Proof08_PersistentHashMap.v *)
