# negative-wrong-cryptosuite

**Bug:** #10 (cryptosuite not enforced on controller proof)
**Originating audit:** Rust (also affects Java)
**Affected at audit time:** Rust 🔴 · Python ✅ · TypeScript ✅ · Java 🔴

## Attack pattern

The spec mandates `eddsa-jcs-2022` for log-entry data-integrity proofs.
A vulnerable resolver only validates `proofPurpose == "assertionMethod"`
and accepts whatever `cryptosuite` value the proof carries. Because the
public-key bytes are decoded from the proof's `verificationMethod`
independently of the suite identifier, an attacker can pair an Ed25519
public key with a non-EdDSA canonicalisation pipeline. This is an
**algorithm-substitution surface** that grows every time the underlying
data-integrity library adds a new suite.

## Why this must be rejected

didwebvh v1.0 is explicit that log-entry proofs use `eddsa-jcs-2022`.
Anything else is undefined behaviour from the spec's perspective and
must not be silently honoured.

The spec should explicitly require:

> The `cryptosuite` field of every log-entry proof MUST be exactly
> `"eddsa-jcs-2022"`. Implementations MUST reject any other value
> outright, before attempting signature verification.

Witness proofs already get this treatment in most implementations
(`enforce_witness_proof_shape` / equivalent); controller proofs need
the same.

## Expected error

`invalidProof` — the resolver should fail at the proof-shape check
without invoking the underlying verifier.

## Implementation references

- Rust: `src/log_entry/read.rs:67-73` only checked `proof_purpose`. Fixed
  in patch `0010`.
- Java: no validator reads `getCryptosuite()` / `getProofPurpose()`
  (grep across `validate/` confirms only model classes reference the
  fields).
- TypeScript: not vulnerable — `src/assertions.ts:79-81` explicitly
  rejects any proof whose `cryptosuite !== 'eddsa-jcs-2022'`.
- Python: not vulnerable — `di_jcs_verify` (`proof.py:92-99`) requires
  the proof's `cryptosuite` and the key's multicodec to jointly match
  one entry in `DI_SUPPORTED`.

## Generator notes

Requires `corrupt` with `when: after-sign` to rewrite the
`proof.cryptosuite` field without re-signing.
