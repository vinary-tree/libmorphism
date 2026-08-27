(** * Constructive countermodels for rejected semantic conflations *)

From Stdlib Require Import Lists.List Arith.PeanoNat Bool.Bool.
Import ListNotations.

(** Tropical-style path extension is addition; the meet paired with min-choice
    is max.  They differ on concrete inputs. *)
Example semiring_times_is_not_lattice_meet :
  Nat.add 1 2 <> Nat.max 1 2.
Proof. discriminate. Qed.

(** An edit-cost monoid is not a join-semilattice: addition is not
    idempotent. *)
Example edit_cost_combine_is_not_idempotent : Nat.add 1 1 <> 1.
Proof. discriminate. Qed.

Fixpoint nat_member (needle : nat) (values : list nat) : bool :=
  match values with
  | [] => false
  | head :: tail => Nat.eqb needle head || nat_member needle tail
  end.

Fixpoint append_novel (left right : list nat) : list nat :=
  match right with
  | [] => left
  | head :: tail =>
      append_novel
        (if nat_member head left then left else left ++ [head])
        tail
  end.

Definition left_biased_join := append_novel.

(** Left-biased vector union is not commutative as a vector value.  It may be
    lawful only after an explicit content quotient. *)
Example left_biased_vector_join_is_not_commutative :
  left_biased_join [0; 1] [1; 0] <>
  left_biased_join [1; 0] [0; 1].
Proof. discriminate. Qed.

(** A structure-preserving map must be injective before it may claim order
    reflection.  This constant map collapses a source non-order into a target
    equality. *)
Definition bool_natural_order (left right : bool) : Prop :=
  orb left right = right.

Definition collapse_bool (_ : bool) : bool := false.

Example noninjective_map_does_not_reflect_order :
  bool_natural_order (collapse_bool true) (collapse_bool false) /\
  ~ bool_natural_order true false.
Proof.
  split.
  - reflexivity.
  - unfold bool_natural_order; simpl; intro H; discriminate.
Qed.

Print Assumptions semiring_times_is_not_lattice_meet.
Print Assumptions edit_cost_combine_is_not_idempotent.
Print Assumptions left_biased_vector_join_is_not_commutative.
Print Assumptions noninjective_map_does_not_reflect_order.
