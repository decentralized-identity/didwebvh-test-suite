# Negative test vectors

This file documents the `negative-*` scenarios in this directory. Unlike the
happy-path vectors, these scenarios describe **logs and DID URLs that a
conforming resolver MUST reject**.

Every negative vector here is derived from a real cross-implementation
security bug found in an audit of the Rust, Python, TypeScript, and Java
implementations. Each vector folder contains:

- `script.yaml` — DSL script with `negative: true` at top level
- `README.md` — attack pattern, audit bug ID, per-implementation status,
  expected error code

## Index

| Scenario | Audit bug | Vulnerable impls at audit time |
|---|---|---|
| [`negative-duplicate-witness-ids`](./negative-duplicate-witness-ids/) | #3 | Rust, Python, Java |
| [`negative-zero-witness-threshold`](./negative-zero-witness-threshold/) | R2 | Rust, TS (partial) |
| [`negative-scid-mismatch-genesis`](./negative-scid-mismatch-genesis/) | #14 | Rust, TS, Java |
| [`negative-portable-scid-swap`](./negative-portable-scid-swap/) | #15 | Rust, TS, Java |
| [`negative-pre-rotation-omit-updatekeys`](./negative-pre-rotation-omit-updatekeys/) | #9 | Rust, Java |
| [`negative-wrong-cryptosuite`](./negative-wrong-cryptosuite/) | #10 | Rust, Java |
| [`negative-did-key-body-fragment-mismatch`](./negative-did-key-body-fragment-mismatch/) | #1 | Rust |
| [`negative-path-traversal-did`](./negative-path-traversal-did/) | #4 | Rust, TS, Java |
| [`negative-pct-encoded-ip-host`](./negative-pct-encoded-ip-host/) | #12 | Rust, TS, Java |
| [`negative-pct-encoded-traversal`](./negative-pct-encoded-traversal/) | #13 | Rust, TS |
| [`negative-lowercase-pct-port-ip`](./negative-lowercase-pct-port-ip/) | #5 | Rust, TS |
| [`negative-fragment-leaks-into-domain`](./negative-fragment-leaks-into-domain/) | #6 | Rust |
| [`negative-versiontime-non-monotonic`](./negative-versiontime-non-monotonic/) | P1 | Python, TS |
| [`negative-versiontime-future`](./negative-versiontime-future/) | P1 / J8 | Python, TS, Java (partial) |
| [`negative-unknown-method-version`](./negative-unknown-method-version/) | R6 | Rust, Python, TS, Java (all four) |
| [`negative-cross-did-witness-replay`](./negative-cross-did-witness-replay/) | #11 | Rust, TS, Java |

Bug IDs `#1` – `#15` come from the original 15-patch Rust audit. Prefixes
indicate the audit each ID was originated in: `R` = Rust additional,
`P` = Python, `T` = TypeScript, `J` = Java. The full audit matrix is
external to this repo (see the security advisory that ships these
vectors).

## Required DSL extensions

These vectors use three DSL features that are not yet implemented in the
generator harnesses. CLAUDE.md (under "Future Work") already proposes
two of them; the third is added here.

### 1. `corrupt` op  *(already proposed in CLAUDE.md)*

Applies a mutation to a log entry, either:

- `when: before-sign` — generator mutates the log-entry content, then
  signs, producing an entry whose proof is *valid* over semantically
  broken content. Used for SCID mismatch, pre-rotation bypass, timestamp
  manipulation, unknown-method downgrade.
- `when: after-sign` — generator mutates the *signed* entry. Used for
  proof-integrity tests (wrong cryptosuite, body/fragment mismatch).

Mutations used by these vectors:

| `mutation` | `field` | Description |
|---|---|---|
| `replace-state-id-scid` | — | Replace the SCID segment of `state.id` |
| `replace-version-time` | — | Set `versionTime` to a literal value |
| `replace-parameter` | name | Replace a value in `parameters` |
| `drop-parameter` | name | Delete a parameter |
| `replace-proof-field` | name | Replace a field in `proof` (after-sign) |

### 2. `migrate` op  *(already proposed in CLAUDE.md)*

Issues a portable-DID move that the generator executes without invoking
its library's normal portability check. Used by
`negative-portable-scid-swap`.

### 3. `resolve-did` op  *(new — proposed here)*

Resolves a literal DID URL string rather than the DID produced by a
prior `create`. Used by all parser-level vectors where the attack is in
the DID URL itself (path traversal, percent-encoded IP, percent-encoded
`..`, lowercase `%3a`, fragment leak).

```yaml
- op: resolve-did
  did: "did:webvh:Qm0000...0000:127%2E0%2E0%2E1"
  expectError: invalidDid
```

No `keys:` or `create` step is required for these vectors — the SCID in
the DID is a placeholder that the resolver should reject before any
network fetch is attempted.

### Multi-DID support (used only by `negative-cross-did-witness-replay`)

The cross-DID replay vector additionally needs:

- A `did_label:` parameter on every op to disambiguate which DID the op
  applies to (default `A` if absent, preserving backward compatibility
  with all existing happy-path scripts).
- A new `replay-witness-proof` op that copies a witness proof from one
  DID's witness file to another's.
- An optional `did:` field on the `resolve` op naming which DID to
  resolve.

Multi-DID support could ship as a separate, smaller PR before this one
if preferred — only one vector depends on it.

## Resolver contract for negative vectors

Per CLAUDE.md:

> **Compliance implementations are only required to run resolution on
> negative vectors** — they verify that their resolver returns the
> expected error, nothing more.

The harness sees `negative: true` on the script, calls `resolveDID()`
(or `resolveDIDLiteral()` for `resolve-did` scenarios), and asserts that
the returned `didResolutionMetadata.error` equals the value in
`expectError`. Generation complexity (the `corrupt` mutations etc.)
stays in one place — the generator harness.

## Generator implementation order

These vectors ship as YAML scripts without committed artifacts because
the generator extensions above are not yet implemented in any
implementation harness. Suggested implementation order:

1. **`resolve-did`** — trivial to add (no signing, no log construction).
   Unlocks 5 vectors immediately (path traversal, pct-encoded IP,
   pct-encoded `..`, lowercase `%3a`, fragment leak).
2. **`corrupt` before-sign** — straightforward extension to the
   existing entry-build pipeline. Unlocks 7 more vectors.
3. **`corrupt` after-sign** — separate mutation pass post-signing.
   Unlocks 2 more vectors.
4. **`migrate`** — already discussed in CLAUDE.md. Unlocks 1 vector.
5. **Multi-DID support + `replay-witness-proof`** — most invasive;
   unlocks the cross-DID replay vector.

Once these are wired up in any single implementation harness, that
harness can generate and commit the per-implementation artifacts the
same way as the existing happy-path vectors. Each implementation
generator can be enhanced independently.
