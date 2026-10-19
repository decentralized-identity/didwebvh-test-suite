# Rust Compliance Harness

Compliance test harness for the [did:webvh v1.0 specification](https://identity.foundation/didwebvh/v1.0/) using the Rust [didwebvh-rs](https://github.com/decentralized-identity/didwebvh-rs) library.

## Library

**[decentralized-identity/didwebvh-rs](https://github.com/decentralized-identity/didwebvh-rs)** — the Rust implementation of the did:webvh method, pulled directly from GitHub via Cargo.

## Prerequisites

- Rust toolchain (stable) with Cargo

## Running

```bash
cargo run --manifest-path implementations/rust/Cargo.toml --bin test-vectors
```

Output reports `PASS` / `DIFF` / `FAIL` / `SKIP` / `XFAIL`. Exit code is `0` when there are no conformance differences or failures; `1` for a conformance `DIFF` or `FAIL`.

`resolve-did` script steps with `expectFetchUrl` or URL-level `expectError: invalidDid` are also checked without network access. The harness calls `WebVHURL::parse_did_url` and `get_fetch_url("did.jsonl", HostPolicy::PublicOnly)`, the default pre-request egress check used by the resolver. A different URL is `DIFF`, a construction error when a URL was expected is `FAIL`, and errors requiring log resolution are `SKIP` at this stage. Results appear under **Pre-network URL assertions** in `status.md`.

Set `probe: true` on a scenario or step to report these checks separately under **URL probes (non-conformance)**. Probe `PASS` / `DIFF` / `FAIL` / `SKIP` counts are printed but do not affect the conformance exit code.

## Current Test Results

```text
PASS   basic-create/resolutionResult.json
XFAIL  basic-update/resolutionResult.json (TS COMPAT: nextKeyHashes:[] - TS serialises empty list; Rust library rejects)
XFAIL  deactivate/resolutionResult.json (TS COMPAT: nextKeyHashes:[] - TS serialises empty list; Rust library rejects)
XFAIL  key-rotation/resolutionResult.json (TS COMPAT: nextKeyHashes:[] - TS serialises empty list; Rust library rejects)
XFAIL  multi-update/resolutionResult.1.json (TS COMPAT: nextKeyHashes:[] - TS serialises empty list; Rust library rejects)
XFAIL  multi-update/resolutionResult.2.json (TS COMPAT: nextKeyHashes:[] - TS serialises empty list; Rust library rejects)
XFAIL  multi-update/resolutionResult.json (TS COMPAT: nextKeyHashes:[] - TS serialises empty list; Rust library rejects)
XFAIL  multiple-update-keys/resolutionResult.json (TS COMPAT: nextKeyHashes:[] - TS serialises empty list; Rust library rejects)
PASS   portable/resolutionResult.json
XFAIL  portable-move/resolutionResult.json (TS COMPAT: nextKeyHashes:[] - TS serialises empty list; Rust library rejects)
PASS   pre-rotation/resolutionResult.json
PASS   pre-rotation-consume/resolutionResult.json
XFAIL  services/resolutionResult.json (TS COMPAT: nextKeyHashes:[] - TS serialises empty list; Rust library rejects)
PASS   witness-threshold/resolutionResult.json
XFAIL  witness-update/resolutionResult.json (TS COMPAT: nextKeyHashes:[] - TS serialises empty list; Rust library rejects)

5 passed, 0 failed, 10 xfailed
```

## Known Issues

### TS COMPAT — `nextKeyHashes: []`

The TypeScript generator serialises `nextKeyHashes: []` for every log entry that does not use pre-rotation (it always writes the field, defaulting to an empty list).  The Rust library rejects an empty list when validating the next entry's pre-rotation check against the previous entry's `nextKeyHashes`.

This affects all multi-entry logs that were not specifically written with pre-rotation in mind.  Single-entry logs (`basic-create`, `portable`, `pre-rotation`) and logs with non-empty `nextKeyHashes` throughout (`pre-rotation-consume`) are unaffected.

**Resolution needed:** either the TS generator should omit `nextKeyHashes` when the list is empty, or the Rust library should treat `[]` as equivalent to absent.

### Deactivated DID `updateKeys`

The TS generator includes `updateKeys` in the deactivation log entry.  The Rust library requires `updateKeys` to be null (absent) in a deactivation entry.  This conflict would surface for the `deactivate` scenario if it were not already masked by the `nextKeyHashes: []` XFAIL above.

### Other TS COMPAT normalisations applied in `src/main.rs`

Several cosmetic differences between the TS generator's output and the Rust library's output are papered over by normalisation functions.  Each is documented inline in `src/main.rs`:

| Issue | Description |
| --- | --- |
| Resolution envelope `@context` | Rust may add `"@context": "https://w3id.org/did-resolution/v1"`; TS vectors omit it |
| Service ID form | Rust may expand bare fragment IDs (`#files`) to absolute DID URLs; TS vectors use bare fragments |
| Service endpoint trailing slash | Rust may append a trailing slash to some `serviceEndpoint` values; TS vectors omit it |
| Service ordering | Rust may return implied services in a different order; actual services are sorted by id before comparison |
| `didResolutionMetadata` shape | Rust may return `null`; TS vectors commit `{"contentType": "application/did+ld+json"}` |
| Extra `didDocumentMetadata` fields | Rust emits additional fields (`deactivated`, `portable`, `scid`, `watchers`, `witness`) not present in all TS vectors |
| `versionNumber` absent | Rust `MetaData` struct does not include `versionNumber`; harness derives it from the `versionId` prefix |

## Structure

```text
implementations/rust/
  Cargo.toml    # crate manifest; depends on didwebvh-rs from GitHub
  src/
    main.rs     # compliance vector harness
```
