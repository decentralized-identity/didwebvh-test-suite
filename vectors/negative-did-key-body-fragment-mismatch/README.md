# negative-did-key-body-fragment-mismatch

**Bug:** #1 (did:key body/fragment mismatch in proof auth)
**Originating audit:** Rust
**Affected at audit time:** Rust 🔴 · Python ✅ · TypeScript ✅ · Java ✅

## Attack pattern

A `did:key` verification method has the shape `did:key:<mb>#<mb>` where
the two multibase values are normally the same. The authorisation check
in a vulnerable resolver only compares the **fragment** (after `#`)
against the authorised `updateKeys`, while the signature verification
decodes the public key bytes from the **body** (before `#`). An
attacker constructs:

```
verificationMethod = did:key:<attacker-multibase>#<authorized-multibase>
```

The fragment matches an authorised key so the authorisation check
passes. The body decodes to the attacker's key so signature verification
succeeds against a signature the attacker just made. The result is a
full forgery primitive: anyone can mint log entries for any did:webvh DID
whose `updateKeys` they know.

## Why this must be rejected

A `did:key` URL's body and fragment are intended to refer to the same
key. Treating them as independent (one for auth, one for crypto) breaks
the binding.

The spec should explicitly require:

> When a log-entry proof's `verificationMethod` is a `did:key` URL, its
> body and fragment MUST decode to the same multibase value, and
> implementations MUST validate this equality before either the
> authorisation check or the signature verification.

## Expected error

`invalidProof` — the shape of the verificationMethod is invalid.

## Implementation references

- Rust: `src/log_entry/read.rs:198-203` — `check_signing_key_authorized`
  only checked the fragment. Fixed in patch `0001`.
- Python: not vulnerable — `resolve_did_key` (`proof.py:118-138`)
  enforces `method_key != method_fragment` and raises.
- TypeScript: not vulnerable — `isKeyAuthorized`
  (`src/assertions.ts:11-21`) extracts the body before `#`.
- Java: not vulnerable — `ProofVerifier.verify` (line 41) and
  `isAuthorized` (line 64) both use the fragment.

## Generator notes

Requires `corrupt` with `when: after-sign`. Two seeded keys must be
declared so the generator can compute both multibases. The generator's
substitution `{KEY_AUTHORIZED}` / `{KEY_ATTACKER}` resolves at corrupt
time from the keys block.
