(** * Law-bearing algebra and lawful structure-preserving maps *)

From Stdlib Require Import Bool.Bool Arith.PeanoNat.

Record Monoid (Carrier : Type) : Type := monoid {
  monoid_op : Carrier -> Carrier -> Carrier;
  monoid_unit : Carrier;
  monoid_assoc : forall a b c,
    monoid_op (monoid_op a b) c = monoid_op a (monoid_op b c);
  monoid_left_identity : forall a, monoid_op monoid_unit a = a;
  monoid_right_identity : forall a, monoid_op a monoid_unit = a
}.

Definition nat_add_monoid : Monoid nat.
Proof.
  refine (monoid nat Nat.add 0 _ _ _).
  - intros; symmetry; apply Nat.add_assoc.
  - exact Nat.add_0_l.
  - exact Nat.add_0_r.
Defined.

Record JoinSemilattice (Carrier : Type) : Type := join_semilattice {
  join : Carrier -> Carrier -> Carrier;
  join_assoc : forall a b c, join (join a b) c = join a (join b c);
  join_comm : forall a b, join a b = join b a;
  join_idempotent : forall a, join a a = a
}.

Record MeetSemilattice (Carrier : Type) : Type := meet_semilattice {
  meet : Carrier -> Carrier -> Carrier;
  meet_assoc : forall a b c, meet (meet a b) c = meet a (meet b c);
  meet_comm : forall a b, meet a b = meet b a;
  meet_idempotent : forall a, meet a a = a
}.

Record Lattice (Carrier : Type) : Type := lattice {
  lattice_join : JoinSemilattice Carrier;
  lattice_meet : MeetSemilattice Carrier;
  absorb_join_meet : forall a b,
    join Carrier lattice_join a (meet Carrier lattice_meet a b) = a;
  absorb_meet_join : forall a b,
    meet Carrier lattice_meet a (join Carrier lattice_join a b) = a
}.

Definition bool_join_semilattice : JoinSemilattice bool.
Proof.
  refine (join_semilattice bool orb _ _ _).
  - intros [] [] []; reflexivity.
  - intros [] []; reflexivity.
  - intros []; reflexivity.
Defined.

Definition bool_meet_semilattice : MeetSemilattice bool.
Proof.
  refine (meet_semilattice bool andb _ _ _).
  - intros [] [] []; reflexivity.
  - intros [] []; reflexivity.
  - intros []; reflexivity.
Defined.

Definition bool_lattice : Lattice bool.
Proof.
  refine (lattice bool bool_join_semilattice bool_meet_semilattice _ _);
    intros [] []; reflexivity.
Defined.

Record Semiring (Carrier : Type) : Type := semiring {
  plus : Carrier -> Carrier -> Carrier;
  times : Carrier -> Carrier -> Carrier;
  zero : Carrier;
  one : Carrier;
  plus_assoc : forall a b c, plus (plus a b) c = plus a (plus b c);
  plus_comm : forall a b, plus a b = plus b a;
  plus_zero_left : forall a, plus zero a = a;
  times_assoc : forall a b c, times (times a b) c = times a (times b c);
  times_one_left : forall a, times one a = a;
  times_one_right : forall a, times a one = a;
  times_plus_left : forall a b c,
    times a (plus b c) = plus (times a b) (times a c);
  times_plus_right : forall a b c,
    times (plus a b) c = plus (times a c) (times b c);
  zero_times_left : forall a, times zero a = zero;
  zero_times_right : forall a, times a zero = zero
}.

Record IdempotentSemiring (Carrier : Type) : Type := idempotent_semiring {
  semiring_part : Semiring Carrier;
  plus_idempotent : forall a,
    plus Carrier semiring_part a a = a
}.

Definition natural_order {Carrier : Type} (R : Semiring Carrier)
    (left right : Carrier) : Prop :=
  plus Carrier R left right = right.

Record SemiringHomomorphism
    (Source Target : Type)
    (R : Semiring Source)
    (S : Semiring Target) : Type := semiring_homomorphism {
  hom_map : Source -> Target;
  hom_zero : hom_map (zero Source R) = zero Target S;
  hom_one : hom_map (one Source R) = one Target S;
  hom_plus : forall a b,
    hom_map (plus Source R a b) =
    plus Target S (hom_map a) (hom_map b);
  hom_times : forall a b,
    hom_map (times Source R a b) =
    times Target S (hom_map a) (hom_map b)
}.

Theorem semiring_hom_preserves_natural_order :
  forall Source Target (R : Semiring Source) (S : Semiring Target)
         (H : SemiringHomomorphism Source Target R S) a b,
  natural_order R a b ->
  natural_order S (hom_map Source Target R S H a)
                  (hom_map Source Target R S H b).
Proof.
  intros Source Target R S H a b Horder.
  unfold natural_order in *.
  rewrite <- (hom_plus Source Target R S H a b), Horder.
  reflexivity.
Qed.

Definition Injective {Source Target : Type} (map : Source -> Target) : Prop :=
  forall left right, map left = map right -> left = right.

Theorem injective_semiring_hom_reflects_natural_order :
  forall Source Target (R : Semiring Source) (S : Semiring Target)
         (H : SemiringHomomorphism Source Target R S),
  Injective (hom_map Source Target R S H) ->
  forall a b,
    natural_order S (hom_map Source Target R S H a)
                    (hom_map Source Target R S H b) ->
    natural_order R a b.
Proof.
  intros Source Target R S H Hinjective a b Horder.
  unfold natural_order in *; apply Hinjective.
  rewrite (hom_plus Source Target R S H a b).
  exact Horder.
Qed.

Theorem idempotent_semiring_addition_is_a_join_semilattice :
  forall Carrier (R : IdempotentSemiring Carrier),
  JoinSemilattice Carrier.
Proof.
  intros Carrier [S Hidem].
  refine (join_semilattice Carrier (plus Carrier S) _ _ _).
  - apply plus_assoc.
  - apply plus_comm.
  - exact Hidem.
Defined.

Print Assumptions semiring_hom_preserves_natural_order.
Print Assumptions injective_semiring_hom_reflects_natural_order.
Print Assumptions idempotent_semiring_addition_is_a_join_semilattice.
