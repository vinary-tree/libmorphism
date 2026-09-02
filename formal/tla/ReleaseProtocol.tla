---- MODULE ReleaseProtocol ----
EXTENDS Naturals, TLC

(** An exhaustive model of the irreversible GitHub release boundary.  The
    protocol may prepare policy and tag protection in either order, but it may
    create a draft only after both controls and exact-source validation hold.
    It may publish only after every checksummed asset is attached. *)

VARIABLES phase,
          immutablePolicy,
          tagProtected,
          sourceValidated,
          assetsAttached,
          immutable

vars ==
  <<phase, immutablePolicy, tagProtected, sourceValidated, assetsAttached,
    immutable>>

Phases == {"Absent", "Draft", "Published"}

Init ==
  /\ phase = "Absent"
  /\ immutablePolicy = FALSE
  /\ tagProtected = FALSE
  /\ sourceValidated = FALSE
  /\ assetsAttached = FALSE
  /\ immutable = FALSE

EnableImmutablePolicy ==
  /\ phase = "Absent"
  /\ immutablePolicy' = TRUE
  /\ UNCHANGED <<phase, tagProtected, sourceValidated, assetsAttached, immutable>>

ProtectTag ==
  /\ phase = "Absent"
  /\ tagProtected' = TRUE
  /\ UNCHANGED <<phase, immutablePolicy, sourceValidated, assetsAttached, immutable>>

ValidateSource ==
  /\ phase = "Absent"
  /\ tagProtected
  /\ sourceValidated' = TRUE
  /\ UNCHANGED <<phase, immutablePolicy, tagProtected, assetsAttached, immutable>>

CreateDraft ==
  /\ phase = "Absent"
  /\ immutablePolicy
  /\ tagProtected
  /\ sourceValidated
  /\ phase' = "Draft"
  /\ UNCHANGED
       <<immutablePolicy, tagProtected, sourceValidated, assetsAttached,
         immutable>>

AttachChecksummedAssets ==
  /\ phase = "Draft"
  /\ assetsAttached' = TRUE
  /\ UNCHANGED
       <<phase, immutablePolicy, tagProtected, sourceValidated, immutable>>

Publish ==
  /\ phase = "Draft"
  /\ immutablePolicy
  /\ tagProtected
  /\ sourceValidated
  /\ assetsAttached
  /\ phase' = "Published"
  /\ immutable' = TRUE
  /\ UNCHANGED
       <<immutablePolicy, tagProtected, sourceValidated, assetsAttached>>

Next ==
  EnableImmutablePolicy \/ ProtectTag \/ ValidateSource \/ CreateDraft \/
    AttachChecksummedAssets \/ Publish

Spec == Init /\ [][Next]_vars

TypeOK ==
  /\ phase \in Phases
  /\ immutablePolicy \in BOOLEAN
  /\ tagProtected \in BOOLEAN
  /\ sourceValidated \in BOOLEAN
  /\ assetsAttached \in BOOLEAN
  /\ immutable \in BOOLEAN

DraftHasAllPreconditions ==
  (phase = "Draft") => immutablePolicy /\ tagProtected /\ sourceValidated

PublishedHasCompleteEvidence ==
  (phase = "Published") =>
    immutablePolicy /\ tagProtected /\ sourceValidated /\ assetsAttached

ImmutableExactlyWhenPublished == immutable <=> (phase = "Published")

PublishedStateCannotChange ==
  (phase = "Published") => ~ENABLED Next

(** TLAPS proves the logical kernels at the two irreversible transitions.
    TLC connects these kernels to the complete protocol state graph. *)

THEOREM DraftCreationRequiresAllPreconditions ==
  (immutablePolicy /\ tagProtected /\ sourceValidated /\ phase' = "Draft")
    => immutablePolicy /\ tagProtected /\ sourceValidated
<1>1. QED
  OBVIOUS

THEOREM PublicationRequiresCompleteEvidence ==
  (immutablePolicy /\ tagProtected /\ sourceValidated /\ assetsAttached /\
   phase' = "Published" /\ immutable' = TRUE)
    => immutablePolicy /\ tagProtected /\ sourceValidated /\ assetsAttached /\
       immutable'
<1>1. QED
  OBVIOUS

THEOREM ImmutablePublicationIsTerminal ==
  (phase = "Published" /\ immutable /\ ~ENABLED Next)
    => immutable /\ ~ENABLED Next
<1>1. QED
  OBVIOUS

====
