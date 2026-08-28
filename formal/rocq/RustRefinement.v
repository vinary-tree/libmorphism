(** * RustRefinement — pre-implementation contract for the Rust API

    This module fixes the representation-independent meaning of the public
    Rust vocabulary before production code is admitted.  Stable identifiers
    are abstracted by naturals; the Rust refinement uses fixed-size byte
    arrays.  A composition check is diagnostic data, while a composition
    witness and a validated morphism are proof-carrying values whose fields
    are private in Rust.
*)

From Stdlib Require Import Bool.Bool Lists.List Arith.PeanoNat.
Require Import Libmorphism.Composition.
Import ListNotations.

Definition StableId : Type := nat.
Definition DomainId : Type := StableId.

Record Signature : Type := signature {
  signature_source : DomainId;
  signature_target : DomainId
}.

Definition signature_eqb (left right : Signature) : bool :=
  Nat.eqb (signature_source left) (signature_source right) &&
  Nat.eqb (signature_target left) (signature_target right).

Lemma signature_eqb_eq : forall left right,
  signature_eqb left right = true <-> left = right.
Proof.
  intros [left_source left_target] [right_source right_target].
  unfold signature_eqb; simpl.
  rewrite andb_true_iff, !Nat.eqb_eq; split.
  - intros [Hsource Htarget]; now subst.
  - intros H; inversion H; auto.
Qed.

Record Provenance : Type := provenance {
  provenance_artifact : StableId;
  provenance_revision : nat;
  provenance_sequence : nat
}.

Inductive LawKind : Type :=
| IdentityLaw
| AssociativityLaw
| ExactDenotationLaw
| EffectSoundnessLaw.

Definition law_kind_eqb (left right : LawKind) : bool :=
  match left, right with
  | IdentityLaw, IdentityLaw
  | AssociativityLaw, AssociativityLaw
  | ExactDenotationLaw, ExactDenotationLaw
  | EffectSoundnessLaw, EffectSoundnessLaw => true
  | _, _ => false
  end.

Record LawEvidence : Type := law_evidence {
  evidence_subject : StableId;
  evidence_kind : LawKind;
  evidence_verifier : StableId;
  evidence_policy_version : nat
}.

Record MorphismDescriptor : Type := morphism_descriptor {
  descriptor_id : StableId;
  descriptor_signature : Signature;
  descriptor_effects : Effects;
  descriptor_precision : Precision;
  descriptor_completeness : Completeness;
  descriptor_provenance : Provenance
}.

Record CompositionSummary : Type := composition_summary {
  summary_signature : Signature;
  summary_effects : Effects;
  summary_precision : Precision;
  summary_completeness : Completeness;
  summary_provenance : list Provenance
}.

Inductive CompositionCheck : Type :=
| Compatible : CompositionSummary -> CompositionCheck
| EndpointMismatch : DomainId -> DomainId -> CompositionCheck.

(** The summary is a pure product of the two descriptors.  Provenance retains
    execution order: [before] is always the first element. *)
Definition compose_summary
    (after before : MorphismDescriptor) : CompositionSummary :=
  composition_summary
    (signature
      (signature_source (descriptor_signature before))
      (signature_target (descriptor_signature after)))
    (effects_union
      (descriptor_effects before)
      (descriptor_effects after))
    (precision_compose
      (descriptor_precision before)
      (descriptor_precision after))
    (completeness_compose
      (descriptor_completeness before)
      (descriptor_completeness after))
    [descriptor_provenance before; descriptor_provenance after].

(** [check_composition after before] checks [after o before]. *)
Definition check_composition
    (after before : MorphismDescriptor) : CompositionCheck :=
  let before_target := signature_target (descriptor_signature before) in
  let after_source := signature_source (descriptor_signature after) in
  if Nat.eqb before_target after_source then
    Compatible (compose_summary after before)
  else EndpointMismatch before_target after_source.

Theorem composition_check_rejects_mismatched_endpoints :
  forall after before,
    signature_target (descriptor_signature before) <>
      signature_source (descriptor_signature after) ->
    check_composition after before =
      EndpointMismatch
        (signature_target (descriptor_signature before))
        (signature_source (descriptor_signature after)).
Proof.
  intros after before Hmismatch; unfold check_composition.
  destruct (Nat.eqb
    (signature_target (descriptor_signature before))
    (signature_source (descriptor_signature after))) eqn:Heq.
  - apply Nat.eqb_eq in Heq; contradiction.
  - reflexivity.
Qed.

Theorem composition_check_reports_endpoints :
  forall after before summary,
    check_composition after before = Compatible summary ->
    summary_signature summary =
      signature
        (signature_source (descriptor_signature before))
        (signature_target (descriptor_signature after)).
Proof.
  intros after before summary Hcheck; unfold check_composition in Hcheck.
  destruct (Nat.eqb
    (signature_target (descriptor_signature before))
    (signature_source (descriptor_signature after))); try discriminate.
  now inversion Hcheck.
Qed.

Theorem composition_summary_exact_inputs :
  forall after before summary,
    check_composition after before = Compatible summary ->
    summary_precision summary = Exact ->
    descriptor_precision before = Exact /\
    descriptor_precision after = Exact.
Proof.
  intros after before summary Hcheck Hexact.
  unfold check_composition in Hcheck.
  destruct (Nat.eqb
    (signature_target (descriptor_signature before))
    (signature_source (descriptor_signature after))); try discriminate.
  inversion Hcheck; subst; simpl in Hexact.
  now apply exact_composition_has_only_exact_inputs in Hexact.
Qed.

Theorem composition_summary_complete_inputs :
  forall after before summary,
    check_composition after before = Compatible summary ->
    summary_completeness summary = Complete ->
    descriptor_completeness before = Complete /\
    descriptor_completeness after = Complete.
Proof.
  intros after before summary Hcheck Hcomplete.
  unfold check_composition in Hcheck.
  destruct (Nat.eqb
    (signature_target (descriptor_signature before))
    (signature_source (descriptor_signature after))); try discriminate.
  inversion Hcheck; subst; simpl in Hcomplete.
  now apply complete_composition_has_only_complete_inputs in Hcomplete.
Qed.

Theorem composition_summary_provenance_order :
  forall after before summary,
    check_composition after before = Compatible summary ->
    summary_provenance summary =
      [descriptor_provenance before; descriptor_provenance after].
Proof.
  intros after before summary Hcheck; unfold check_composition in Hcheck.
  destruct (Nat.eqb
    (signature_target (descriptor_signature before))
    (signature_source (descriptor_signature after))); try discriminate.
  now inversion Hcheck.
Qed.

Record CompositionWitness
    (after before : MorphismDescriptor) : Type := composition_witness {
  witnessed_summary : CompositionSummary;
  witnessed_composition :
    check_composition after before = Compatible witnessed_summary
}.

Definition validate_composition
    (after before : MorphismDescriptor) :
    option (CompositionWitness after before).
Proof.
  destruct (Nat.eqb
    (signature_target (descriptor_signature before))
    (signature_source (descriptor_signature after))) eqn:Hcompatible.
  - apply Some.
    refine (composition_witness after before (compose_summary after before) _).
    unfold check_composition; now rewrite Hcompatible.
  - exact None.
Defined.

Theorem validate_composition_is_sound :
  forall after before witness,
    validate_composition after before = Some witness ->
    check_composition after before =
      Compatible (witnessed_summary after before witness).
Proof.
  intros after before witness _.
  exact (witnessed_composition after before witness).
Qed.

Section EvidenceValidation.

Variable verifier_accepts : LawEvidence -> bool.

Definition exact_evidence_valid
    (descriptor : MorphismDescriptor)
    (evidence : LawEvidence) : bool :=
  Nat.eqb (evidence_subject evidence) (descriptor_id descriptor) &&
  law_kind_eqb (evidence_kind evidence) ExactDenotationLaw &&
  verifier_accepts evidence.

Record DescriptorCandidate : Type := descriptor_candidate {
  candidate_descriptor : MorphismDescriptor;
  candidate_claimed_signature : Signature;
  candidate_exact_evidence : option LawEvidence
}.

Definition descriptor_candidate_valid (candidate : DescriptorCandidate) : bool :=
  signature_eqb
    (candidate_claimed_signature candidate)
    (descriptor_signature (candidate_descriptor candidate)) &&
  match descriptor_precision (candidate_descriptor candidate) with
  | Exact =>
      match candidate_exact_evidence candidate with
      | Some evidence =>
          exact_evidence_valid (candidate_descriptor candidate) evidence
      | None => false
      end
  | SoundApproximation => true
  end.

Definition ValidatedMorphism :=
  { candidate : DescriptorCandidate |
    descriptor_candidate_valid candidate = true }.

Definition validate_descriptor (candidate : DescriptorCandidate) :
    option ValidatedMorphism :=
  match Bool.bool_dec (descriptor_candidate_valid candidate) true with
  | left proof => Some (exist _ candidate proof)
  | right _ => None
  end.

Theorem validate_descriptor_is_sound :
  forall candidate validated,
    validate_descriptor candidate = Some validated ->
    descriptor_candidate_valid candidate = true.
Proof.
  intros candidate validated Hvalidate; unfold validate_descriptor in Hvalidate.
  destruct (Bool.bool_dec (descriptor_candidate_valid candidate) true).
  - assumption.
  - discriminate.
Qed.

Theorem validated_exact_has_verified_bound_evidence :
  forall candidate,
    descriptor_candidate_valid candidate = true ->
    descriptor_precision (candidate_descriptor candidate) = Exact ->
    exists evidence,
      candidate_exact_evidence candidate = Some evidence /\
      exact_evidence_valid (candidate_descriptor candidate) evidence = true.
Proof.
  intros [descriptor claimed evidence] Hvalid Hexact; simpl in *.
  unfold descriptor_candidate_valid in Hvalid; simpl in Hvalid.
  apply andb_true_iff in Hvalid as [_ Hevidence].
  rewrite Hexact in Hevidence.
  destruct evidence as [evidence |]; try discriminate.
  exists evidence; auto.
Qed.

Record TypedMorphismWitness
    (source_id target_id : DomainId) : Type := typed_morphism_witness {
  typed_validated : ValidatedMorphism;
  typed_endpoints :
    descriptor_signature
      (candidate_descriptor (proj1_sig typed_validated)) =
    signature source_id target_id
}.

Theorem typed_witness_has_declared_endpoints :
  forall source_id target_id
         (witness : TypedMorphismWitness source_id target_id),
    descriptor_signature
      (candidate_descriptor
        (proj1_sig (typed_validated source_id target_id witness))) =
    signature source_id target_id.
Proof.
  intros source_id target_id witness.
  exact (typed_endpoints source_id target_id witness).
Qed.

End EvidenceValidation.

Print Assumptions composition_check_reports_endpoints.
Print Assumptions composition_summary_exact_inputs.
Print Assumptions composition_summary_provenance_order.
Print Assumptions validate_composition_is_sound.
Print Assumptions validated_exact_has_verified_bound_evidence.
Print Assumptions typed_witness_has_declared_endpoints.
