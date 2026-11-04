# Python Compliance Harness

Compliance test harness for the [did:webvh v1.0 specification](https://identity.foundation/didwebvh/v1.0/) using the Python [did-webvh](https://pypi.org/project/did-webvh/) library.

## Library

**[did-webvh](https://pypi.org/project/did-webvh/)** — the reference Python implementation of the did:webvh method, installable from PyPI.

## Prerequisites

- Python 3.12 or later

## Running

```bash
cd implementations/python
python -m venv .venv
.venv/bin/pip install did-webvh pytest pytest-asyncio jcs==0.2.1
.venv/bin/pytest
```

## Test Files

| File | What it tests |
|---|---|
| `test_vectors.py` | Resolves every vector in `vectors/` against the Python resolver and compares against the committed `resolutionResult*.json` files; also checks `resolve-did` script steps (see below) |
| `test_generate.py` | Re-executes each `script.yaml` using the Python library (create, update, deactivate) and verifies the generated log and resolution results match the committed artifacts |
| `test_resolver_suite.py` | Runs the hand-authored error-case suite in `test-suite/` — invalid logs, missing proofs, etc. |

The `test-suite/` directory contains hand-authored logs for negative / error scenarios (`invalid-json`, `invalid-method`, `invalid-scid`, `missing-proof`, `missing-witness`) plus a happy-path log (`short`) that covers resolution by `versionId`, `versionNumber`, and conflict cases.  These are currently Python-only — see `CLAUDE.md` Future Work for the plan to make them cross-language.

## `resolve-did` script steps

`test_resolve_did_step` checks `resolve-did` steps that specify `expectFetchUrl`, or `expectError` in a scenario with no (or an empty) `ts/did.jsonl`. It calls `resolve_did(did)` with `aiohttp.ClientSession.get` patched to record the URL argument exactly as the library passed it — before aiohttp converts it to a `yarl.URL` — and to abort the request, so no network access occurs.

| Step | PASS | DIFF | FAIL | SKIP |
|---|---|---|---|---|
| `expectFetchUrl` | captured URL equals expected | a different URL was fetched | resolver rejected the DID before fetching | — |
| `expectError` | no fetch, and the expected error was returned (an exception raised before fetching counts as `invalidDid`) | no fetch, but a different error was returned | resolver fetched a URL (`invalidDid` expected) | expected error requires resolution beyond URL construction |

Malformed steps (no `did`, or both / neither expectation) are reported as FAIL. Results appear in `status.md` under **Pre-network URL assertions**; DIFF/FAIL details go to `diffs.txt`. Scenarios already covered here are omitted from **Negative Resolution**.

Set `probe: true` on a scenario or step to report its results under **URL probes (non-conformance)** instead. Probe outcomes are recorded but never fail the pytest run.

## Current Test Results (`test_vectors.py`)

| Scenario | Result | Notes |
|---|---|---|
| `basic-create` | PASS | |
| `basic-update` | XFAIL | See [TS COMPAT — nextKeyHashes](#ts-compat--nextkeyhashe-) below |
| `deactivate` | XFAIL | See nextKeyHashes note |
| `key-rotation` | XFAIL | See nextKeyHashes note |
| `multiple-update-keys` | XFAIL | See nextKeyHashes note |
| `multi-update` (×3) | XFAIL | See nextKeyHashes note |
| `portable` | PASS | |
| `portable-move` | XFAIL | See nextKeyHashes note |
| `pre-rotation` | PASS | |
| `pre-rotation-consume` | PASS | |
| `services` | XFAIL | See nextKeyHashes note |
| `witness-threshold` | PASS | |
| `witness-update` | XFAIL | See nextKeyHashes note |

## Known Issues

### TS COMPAT — `nextKeyHashes: []`

The TypeScript generator serialises `nextKeyHashes: []` for every log entry that does not use pre-rotation (it always writes the field, defaulting to an empty list).  The Python library's update validator rejects an empty list: it requires the field to be either absent or a non-empty list of strings.

This affects all multi-entry logs that were not specifically written with pre-rotation in mind.  Single-entry logs (`basic-create`, `portable`, `pre-rotation`) and logs with non-empty `nextKeyHashes` throughout (`pre-rotation-consume`) are unaffected.

**Resolution needed:** either the TS generator should omit `nextKeyHashes` when the list is empty, or the Python library should treat `[]` as equivalent to absent.

### Other TS COMPAT normalisations applied in `test_vectors.py`

Several cosmetic differences between the TS generator's output and the Python library's output are papered over by normalisation functions so they do not cause false failures.  Each is documented inline in `test_vectors.py` and is a candidate for upstream resolution:

| Issue | Description |
|---|---|
| Resolution envelope `@context` | Python always adds `"@context": "https://w3id.org/did-resolution/v1"`; TS vectors omit it |
| Service ID form | Python expands bare fragment IDs (`#files`) to absolute DID URLs; TS vectors use bare fragments |
| Service endpoint trailing slash | Python appends a trailing slash to some `serviceEndpoint` values; TS vectors omit it |
| `didResolutionMetadata` shape | Python returns `null`; TS vectors commit `{"contentType": "application/did+ld+json"}` |
| Extra `didDocumentMetadata` fields | Python emits additional fields (`deactivated`, `portable`, `scid`, `watchers`, `witness`) not present in all TS vectors |

## Structure

```
implementations/python/
  pytest.ini               # asyncio_mode = auto
  test_vectors.py          # compliance vector harness
  test_generate.py         # generation compliance harness
  test_resolver_suite.py   # error-case resolver suite
  gen_resolver_suite.py    # helper to regenerate resolver-suite.json
  test-suite/              # hand-authored invalid / edge-case logs
    resolver-suite.json    # test manifest
    logs/
      invalid-json/
      invalid-method/
      invalid-scid/
      missing-proof/
      missing-witness/
      short/
```
