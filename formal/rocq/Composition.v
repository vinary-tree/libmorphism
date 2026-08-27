(** * Typed composition, declared effects, precision, and validation

    This is the implementation-independent contract for libmorphism.  A
    morphism records typed endpoints, an observable trace, declared effects,
    result precision, and result completeness.  Composition is partial only
    at the endpoint boundary.  No axiom, admission, or classical principle is
    required. *)

From Stdlib Require Import Bool.Bool Lists.List Arith.PeanoNat.
Import ListNotations.

Inductive Object : Type := object : nat -> Object.

Definition object_eqb (left right : Object) : bool :=
  match left, right with
  | object left_id, object right_id => Nat.eqb left_id right_id
  end.

Lemma object_eqb_refl : forall value, object_eqb value value = true.
Proof. intros [identifier]; simpl; apply Nat.eqb_refl. Qed.

Lemma object_eqb_eq : forall left right,
  object_eqb left right = true <-> left = right.
Proof.
  intros [left_id] [right_id]; simpl; rewrite Nat.eqb_eq; split; intro H.
  - now f_equal.
  - now inversion H.
Qed.

Inductive Primitive : Type := primitive : nat -> Primitive.

Record Effects : Type := effects {
  reads_state : bool;
  writes_state : bool;
  allocates : bool;
  emits_evidence : bool
}.

Definition no_effects : Effects := effects false false false false.

Definition effects_union (left right : Effects) : Effects :=
  effects
    (orb (reads_state left) (reads_state right))
    (orb (writes_state left) (writes_state right))
    (orb (allocates left) (allocates right))
    (orb (emits_evidence left) (emits_evidence right)).

Lemma effects_union_left_identity : forall value,
  effects_union no_effects value = value.
Proof. intros []; reflexivity. Qed.

Lemma effects_union_right_identity : forall value,
  effects_union value no_effects = value.
Proof.
  intros [reads writes allocates evidence].
  unfold effects_union, no_effects; simpl.
  rewrite (orb_false_r reads), (orb_false_r writes),
    (orb_false_r allocates), (orb_false_r evidence).
  reflexivity.
Qed.

Lemma effects_union_assoc : forall first second third,
  effects_union (effects_union first second) third =
  effects_union first (effects_union second third).
Proof.
  intros [] [] []; unfold effects_union; simpl.
  now repeat rewrite orb_assoc.
Qed.

(** [precision_le x y] means that [y] is allowed to be no more precise than
    [x].  Exact information may degrade to a sound approximation; an
    approximation may never promote itself to exact. *)
Inductive Precision : Type := Exact | SoundApproximation.

Definition precision_le (left right : Precision) : bool :=
  match left, right with
  | Exact, _ => true
  | SoundApproximation, SoundApproximation => true
  | SoundApproximation, Exact => false
  end.

Definition precision_compose (left right : Precision) : Precision :=
  match left, right with
  | Exact, Exact => Exact
  | _, _ => SoundApproximation
  end.

Lemma precision_compose_left_identity : forall value,
  precision_compose Exact value = value.
Proof. now intros []. Qed.

Lemma precision_compose_right_identity : forall value,
  precision_compose value Exact = value.
Proof. now intros []. Qed.

Lemma precision_compose_assoc : forall first second third,
  precision_compose (precision_compose first second) third =
  precision_compose first (precision_compose second third).
Proof. now intros [] [] []. Qed.

Theorem precision_compose_monotone : forall first first' second second',
  precision_le first first' = true ->
  precision_le second second' = true ->
  precision_le (precision_compose first second)
               (precision_compose first' second') = true.
Proof. now intros [] [] [] [] Hfirst Hsecond. Qed.

Theorem exact_composition_has_only_exact_inputs : forall left right,
  precision_compose left right = Exact -> left = Exact /\ right = Exact.
Proof. intros [] [] H; inversion H; auto. Qed.

Inductive Completeness : Type := Complete | Incomplete.

Definition completeness_compose (left right : Completeness) : Completeness :=
  match left, right with
  | Complete, Complete => Complete
  | _, _ => Incomplete
  end.

Lemma completeness_compose_left_identity : forall value,
  completeness_compose Complete value = value.
Proof. now intros []. Qed.

Lemma completeness_compose_right_identity : forall value,
  completeness_compose value Complete = value.
Proof. now intros []. Qed.

Lemma completeness_compose_assoc : forall first second third,
  completeness_compose (completeness_compose first second) third =
  completeness_compose first (completeness_compose second third).
Proof. now intros [] [] []. Qed.

Theorem complete_composition_has_only_complete_inputs : forall left right,
  completeness_compose left right = Complete ->
  left = Complete /\ right = Complete.
Proof. intros [] [] H; inversion H; auto. Qed.

Record Morphism : Type := morphism {
  source : Object;
  target : Object;
  trace : list Primitive;
  declared_effects : Effects;
  result_precision : Precision;
  result_completeness : Completeness
}.

Definition identity (at_object : Object) : Morphism :=
  morphism at_object at_object [] no_effects Exact Complete.

(** [compose after before] denotes [after o before]. *)
Definition compose (after before : Morphism) : option Morphism :=
  if object_eqb (target before) (source after) then
    Some (morphism
      (source before)
      (target after)
      (trace before ++ trace after)
      (effects_union (declared_effects before) (declared_effects after))
      (precision_compose (result_precision before) (result_precision after))
      (completeness_compose
        (result_completeness before)
        (result_completeness after)))
  else None.

Theorem compose_reports_typed_endpoints : forall after before composed,
  compose after before = Some composed ->
  target before = source after /\
  source composed = source before /\
  target composed = target after.
Proof.
  intros [asource atarget atrace ae ap ac]
         [bsource btarget btrace be bp bc] composed H.
  unfold compose in H; simpl in H.
  destruct (object_eqb btarget asource) eqn:Htyped; try discriminate.
  inversion H; subst; repeat split; auto.
  now apply object_eqb_eq.
Qed.

Theorem compose_rejects_mismatched_endpoints : forall after before,
  target before <> source after -> compose after before = None.
Proof.
  intros after before Hmismatch; unfold compose.
  destruct (object_eqb (target before) (source after)) eqn:Htyped; auto.
  apply object_eqb_eq in Htyped; contradiction.
Qed.

Theorem compose_right_identity : forall arrow,
  compose (identity (target arrow)) arrow = Some arrow.
Proof.
  intros [s t path e p c]; unfold compose, identity; simpl.
  rewrite object_eqb_refl, app_nil_r, effects_union_right_identity,
    precision_compose_right_identity, completeness_compose_right_identity.
  reflexivity.
Qed.

Theorem compose_left_identity : forall arrow,
  compose arrow (identity (source arrow)) = Some arrow.
Proof.
  intros [s t path e p c]; unfold compose, identity; simpl.
  rewrite object_eqb_refl, effects_union_left_identity.
  now destruct p, c.
Qed.

Definition compose_left_grouped (third second first : Morphism) : option Morphism :=
  match compose second first with
  | Some second_after_first => compose third second_after_first
  | None => None
  end.

Definition compose_right_grouped (third second first : Morphism) : option Morphism :=
  match compose third second with
  | Some third_after_second => compose third_after_second first
  | None => None
  end.

Theorem compose_associative : forall third second first,
  compose_left_grouped third second first =
  compose_right_grouped third second first.
Proof.
  intros [hs ht hp he hprec hcomp]
         [gs gt gp ge gprec gcomp]
         [fs ft fp fe fprec fcomp].
  unfold compose_left_grouped, compose_right_grouped, compose; simpl.
  destruct (object_eqb ft gs) eqn:Hfg;
  destruct (object_eqb gt hs) eqn:Hgh; simpl;
  rewrite ?Hfg, ?Hgh; try reflexivity.
  rewrite app_assoc, effects_union_assoc, precision_compose_assoc,
    completeness_compose_assoc.
  reflexivity.
Qed.

(** Validation turns untrusted claims into an opaque proof-carrying value. *)
Record Candidate : Type := candidate {
  candidate_arrow : Morphism;
  claimed_source : Object;
  claimed_target : Object;
  independent_exact_confirmation : bool
}.

Definition candidate_valid (value : Candidate) : bool :=
  andb
    (object_eqb (claimed_source value) (source (candidate_arrow value)))
    (andb
      (object_eqb (claimed_target value) (target (candidate_arrow value)))
      (match result_precision (candidate_arrow value) with
       | Exact => independent_exact_confirmation value
       | SoundApproximation => true
       end)).

Definition ValidatedCandidate := { value : Candidate | candidate_valid value = true }.

Definition validate (value : Candidate) : option ValidatedCandidate :=
  match Bool.bool_dec (candidate_valid value) true with
  | left proof => Some (exist _ value proof)
  | right _ => None
  end.

Theorem validate_is_sound : forall value validated,
  validate value = Some validated -> candidate_valid value = true.
Proof.
  intros value validated H; unfold validate in H.
  destruct (Bool.bool_dec (candidate_valid value) true) as [Hvalid | Hinvalid].
  - exact Hvalid.
  - discriminate H.
Qed.

Theorem validate_is_complete : forall value,
  candidate_valid value = true -> exists validated, validate value = Some validated.
Proof.
  intros value Hvalid; unfold validate.
  destruct (Bool.bool_dec (candidate_valid value) true) as [Hproof | Hinvalid].
  - eexists; reflexivity.
  - contradiction.
Qed.

Theorem validated_exact_requires_independent_confirmation : forall value,
  candidate_valid value = true ->
  result_precision (candidate_arrow value) = Exact ->
  independent_exact_confirmation value = true.
Proof.
  intros [arrow claimed_s claimed_t confirmation] Hvalid Hexact.
  unfold candidate_valid in Hvalid; simpl in Hvalid, Hexact.
  rewrite Hexact in Hvalid; simpl in Hvalid.
  apply andb_true_iff in Hvalid.
  destruct Hvalid as [_ Hrest].
  apply andb_true_iff in Hrest.
  destruct Hrest as [_ Hconfirmation].
  exact Hconfirmation.
Qed.

Print Assumptions compose_associative.
Print Assumptions exact_composition_has_only_exact_inputs.
Print Assumptions validate_is_sound.
