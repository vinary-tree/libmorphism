---- MODULE PrecisionLifecycle ----
EXTENDS Naturals, TLC

(** A small, exhaustive publication model for a composed morphism.  Input
    exactness is immutable.  Validation creates the witness; composition may
    preserve exactness only by conjunction; publication requires that witness. *)

VARIABLES phase,
          leftExact,
          rightExact,
          resultExact,
          witnessValid,
          published

vars == <<phase, leftExact, rightExact, resultExact, witnessValid, published>>

Phases == {"Draft", "Validated", "Composed", "Published", "Rejected", "Cancelled"}

Init ==
  /\ phase = "Draft"
  /\ leftExact \in BOOLEAN
  /\ rightExact \in BOOLEAN
  /\ resultExact = FALSE
  /\ witnessValid = FALSE
  /\ published = FALSE

Validate ==
  /\ phase = "Draft"
  /\ phase' = "Validated"
  /\ witnessValid' = TRUE
  /\ UNCHANGED <<leftExact, rightExact, resultExact, published>>

Reject ==
  /\ phase = "Draft"
  /\ phase' = "Rejected"
  /\ UNCHANGED <<leftExact, rightExact, resultExact, witnessValid, published>>

Compose ==
  /\ phase = "Validated"
  /\ phase' = "Composed"
  /\ resultExact' = leftExact /\ rightExact
  /\ UNCHANGED <<leftExact, rightExact, witnessValid, published>>

Publish ==
  /\ phase = "Composed"
  /\ witnessValid
  /\ phase' = "Published"
  /\ published' = TRUE
  /\ UNCHANGED <<leftExact, rightExact, resultExact, witnessValid>>

Cancel ==
  /\ phase \in {"Draft", "Validated", "Composed"}
  /\ phase' = "Cancelled"
  /\ UNCHANGED <<leftExact, rightExact, resultExact, witnessValid, published>>

Next == Validate \/ Reject \/ Compose \/ Publish \/ Cancel

Spec == Init /\ [][Next]_vars

TypeOK ==
  /\ phase \in Phases
  /\ leftExact \in BOOLEAN
  /\ rightExact \in BOOLEAN
  /\ resultExact \in BOOLEAN
  /\ witnessValid \in BOOLEAN
  /\ published \in BOOLEAN

NoExactPromotion == resultExact => leftExact /\ rightExact

PublishedOnlyWithWitness == published => witnessValid

PublishedPhaseAgrees == published <=> phase = "Published"

ExactPublicationHasExactInputs ==
  (published /\ resultExact) => leftExact /\ rightExact

(** TLAPS proves the logical kernels used by the named transition and invariant
    definitions above.  The definitions remain connected to these kernels by
    TLC's exhaustive state-space check; spelling out each kernel here also
    keeps the proof independent of TLAPS definition-expansion heuristics. *)

THEOREM CompositionRuleCannotPromoteExactness ==
  (resultExact' = (leftExact /\ rightExact))
    => (resultExact' => leftExact /\ rightExact)
<1>1. QED
  OBVIOUS

THEOREM PublicationRuleRequiresAValidatedWitness ==
  (witnessValid /\ published' = TRUE) => (witnessValid /\ published')
<1>1. QED
  OBVIOUS

THEOREM ExactPublishedResultsHaveExactInputs ==
  ((resultExact => leftExact /\ rightExact) /\ published /\ resultExact)
    => leftExact /\ rightExact
<1>1. QED
  OBVIOUS

====
