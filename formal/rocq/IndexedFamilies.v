(** * Indexed families are fibers; arbitrary families are not fibrations *)

From Stdlib Require Import Init.Specif.

Inductive FeedIndex : Type := dictionary_feed | automaton_feed.

Definition FeedFiber (index : FeedIndex) : Type :=
  match index with
  | dictionary_feed => nat
  | automaton_feed => bool
  end.

(** The total feed is the dependent sum of all fibers. *)
Definition FeedTotal : Type := { index : FeedIndex & FeedFiber index }.

Definition feed_projection (item : FeedTotal) : FeedIndex :=
  match item with
  | existT _ index _ => index
  end.

Theorem every_feed_item_lives_in_its_projected_fiber :
  forall item : FeedTotal, FeedFiber (feed_projection item).
Proof. intros [index value]; exact value. Qed.

(** Calling a projection a fibration requires lift data.  The following
    indexed family has an inhabited target fiber and an empty source fiber,
    while the base contains an arrow from source to target. *)
Inductive Base : Type := empty_fiber | inhabited_fiber.

Definition Fiber (base : Base) : Type :=
  match base with
  | empty_fiber => Empty_set
  | inhabited_fiber => unit
  end.

Inductive BaseArrow : Base -> Base -> Type :=
  into_inhabited : BaseArrow empty_fiber inhabited_fiber.

Definition HasContravariantLifts : Type :=
  forall source target, BaseArrow source target -> Fiber target -> Fiber source.

Theorem an_indexed_family_is_not_automatically_a_fibration :
  HasContravariantLifts -> False.
Proof.
  intro lifts.
  destruct (lifts empty_fiber inhabited_fiber into_inhabited tt).
Qed.

Print Assumptions every_feed_item_lives_in_its_projected_fiber.
Print Assumptions an_indexed_family_is_not_automatically_a_fibration.
