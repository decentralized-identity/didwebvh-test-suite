# dart status

Implementation: didwebvh-dart 0.1.2

## DID Creation

| Test Case | Result | Notes |
|---|---|---|
| basic-create | ✅ PASS |  |
| basic-update | ✅ PASS |  |
| deactivate | ✅ PASS |  |
| idn-alabel-create | ✅ PASS |  |
| key-rotation | ✅ PASS |  |
| multi-update | ✅ PASS |  |
| multiple-update-keys | ⚠️ SKIP | multiple-update-keys at create time not supported by didwebvh-dart API |
| portable | ✅ PASS |  |
| portable-move | ✅ PASS |  |
| pre-rotation | ✅ PASS |  |
| pre-rotation-consume | ✅ PASS |  |
| services | ✅ PASS |  |
| witness-threshold | ✅ PASS |  |
| witness-update | ✅ PASS |  |

## Negative Resolution

| Test Case | Expected Error | Result | Notes |
|---|---|---|---|
| negative-cross-did-witness-replay | invalidDid | ✅ PASS |  |
| negative-did-key-body-fragment-mismatch | invalidProof | ✅ PASS |  |
| negative-dns-label-too-long/step 1 | invalidDid | ❌ FAIL | resolver would fetch invalid DID at: https://aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.example.com/.well-known/did.jsonl |
| negative-duplicate-witness-ids | invalidParameters | ✅ PASS |  |
| negative-fragment-leaks-into-domain/step 1 | invalidDid | ✅ PASS |  |
| negative-lowercase-pct-port-ip/step 1 | invalidDid | ✅ PASS |  |
| negative-path-traversal-did/step 1 | invalidDid | ❌ FAIL | resolver would fetch invalid DID at: https://example.com/../../admin/did.jsonl |
| negative-pct-encoded-ip-host/step 1 | invalidDid | ❌ FAIL | resolver would fetch invalid DID at: https://127%2E0%2E0%2E1/.well-known/did.jsonl |
| negative-pct-encoded-ip-host/step 2 | invalidDid | ❌ FAIL | resolver would fetch invalid DID at: https://127%2e0%2e0%2e1/.well-known/did.jsonl |
| negative-pct-encoded-ip-host/step 3 | invalidDid | ❌ FAIL | resolver would fetch invalid DID at: https://169%2E254%2E169%2E254/.well-known/did.jsonl |
| negative-pct-encoded-traversal/step 1 | invalidDid | ❌ FAIL | resolver would fetch invalid DID at: https://example.com/%252E%252E/admin/did.jsonl |
| negative-pct-encoded-traversal/step 2 | invalidDid | ❌ FAIL | resolver would fetch invalid DID at: https://example.com/%252e%252e/admin/did.jsonl |
| negative-pct-encoded-traversal/step 3 | invalidDid | ❌ FAIL | resolver would fetch invalid DID at: https://example.com/a%252Fb/did.jsonl |
| negative-portable-scid-swap | invalidDid | ✅ PASS |  |
| negative-pre-rotation-omit-updatekeys | invalidParameters | ✅ PASS |  |
| negative-raw-unicode-domain/step 1 | invalidDid | ❌ FAIL | resolver would fetch invalid DID at: https://xn--jp-cd2fp15c.xn--fsq.jp/.well-known/did.jsonl |
| negative-raw-unicode-domain/step 2 | invalidDid | ❌ FAIL | resolver would fetch invalid DID at: https://xn--bcher-kva.example.com/.well-known/did.jsonl |
| negative-scid-mismatch-genesis | invalidDid | ✅ PASS |  |
| negative-unknown-method-version | invalidDid | ✅ PASS |  |
| negative-versiontime-future | invalidDid | ✅ PASS |  |
| negative-versiontime-non-monotonic | invalidDid | ✅ PASS |  |
| negative-witness-update-threshold-not-met | invalidDid | ✅ PASS |  |
| negative-wrong-cryptosuite | invalidProof | ✅ PASS |  |
| negative-zero-witness-threshold | invalidParameters | ✅ PASS |  |

## DID-to-HTTPS Transformation

| Test Case | Expected URL | Result | Notes |
|---|---|---|---|
| idn-did-to-https/step 1 | https://xn--jp-cd2fp15c.xn--fsq.jp/.well-known/did.jsonl | ✅ PASS |  |
| idn-did-to-https/step 2 | https://xn--jp-cd2fp15c.xn--fsq.jp/%E7%94%A8%E6%88%B7/did.jsonl | 🔶 DIFF | actual: https://xn--jp-cd2fp15c.xn--fsq.jp/%25E7%2594%25A8%25E6%2588%25B7/did.jsonl |
| idn-did-to-https/step 3 | https://xn--jp-cd2fp15c.xn--fsq.jp/%E7%94%A8%E6%88%B7/did.jsonl | 🔶 DIFF | actual: https://xn--jp-cd2fp15c.xn--fsq.jp/%25e7%2594%25a8%25e6%2588%25b7/did.jsonl |
| ldh-label-edge-cases/step 1 | https://x.example.com/.well-known/did.jsonl | ✅ PASS |  |
| ldh-label-edge-cases/step 2 | https://3m.example.com/.well-known/did.jsonl | ✅ PASS |  |

## Interoperability Probes (non-conformance)

| Test Case | Expected URL | Result | Notes |
|---|---|---|---|
| idn-pct-ulabel-probe/step 1 | https://xn--jp-cd2fp15c.xn--fsq.jp/.well-known/did.jsonl | 🔶 DIFF | actual: https://jp%E7%B4%8D%E8%B1%86.%E4%BE%8B.jp/.well-known/did.jsonl |
| idn-pct-ulabel-probe/step 2 | https://xn--bcher-kva.example.com/.well-known/did.jsonl | 🔶 DIFF | actual: https://b%C3%BCcher.example.com/.well-known/did.jsonl |
| idn-pct-ulabel-probe/step 3 | https://xn--bcher-kva.example.com/.well-known/did.jsonl | 🔶 DIFF | actual: https://B%C3%9CCHER.example.com/.well-known/did.jsonl |
| idn-pct-ulabel-probe/step 4 | https://xn--fa-hia.de/.well-known/did.jsonl | 🔶 DIFF | actual: https://fa%C3%9F.de/.well-known/did.jsonl |
| path-pct-unreserved-probe/step 1 | https://example.com/a~b/did.jsonl | 🔶 DIFF | actual: https://example.com/a%257Eb/did.jsonl |
| path-pct-unreserved-probe/step 2 | https://example.com/a~b/did.jsonl | 🔶 DIFF | actual: https://example.com/a%257eb/did.jsonl |
| path-pct-unreserved-probe/step 3 | https://example.com/Ab-c_d.e/did.jsonl | 🔶 DIFF | actual: https://example.com/%2541b%252Dc%255Fd%252Ee/did.jsonl |

## Cross-Resolution

| Test Case | Log Source | Result | Notes |
|---|---|---|---|
| basic-create | dart (self) | ✅ PASS |  |
| basic-create | java | ✅ PASS |  |
| basic-create | java-eecc | 🔶 DIFF | see diffs.txt |
| basic-create | python | 🔶 DIFF | see diffs.txt |
| basic-create | rust | 🔶 DIFF | see diffs.txt |
| basic-create | ts | ✅ PASS |  |
| basic-update | dart (self) | ✅ PASS |  |
| basic-update | java | ✅ PASS |  |
| basic-update | java-eecc | 🔶 DIFF | see diffs.txt |
| basic-update | python | 🔶 DIFF | see diffs.txt |
| basic-update | rust | 🔶 DIFF | see diffs.txt |
| basic-update | ts | ✅ PASS |  |
| deactivate | dart (self) | ✅ PASS |  |
| deactivate | java | 🔶 DIFF | see diffs.txt |
| deactivate | java-eecc | 🔶 DIFF | see diffs.txt |
| deactivate | python | 🔶 DIFF | see diffs.txt |
| deactivate | rust | 🔶 DIFF | see diffs.txt |
| deactivate | ts | 🔶 DIFF | see diffs.txt |
| idn-alabel-create | dart (self) | ✅ PASS |  |
| idn-alabel-create | java | ✅ PASS |  |
| idn-alabel-create | java-eecc | 🔶 DIFF | see diffs.txt |
| idn-alabel-create | python | 🔶 DIFF | see diffs.txt |
| idn-alabel-create | rust | 🔶 DIFF | see diffs.txt |
| idn-alabel-create | ts | ✅ PASS |  |
| key-rotation | dart (self) | ✅ PASS |  |
| key-rotation | java | ✅ PASS |  |
| key-rotation | java-eecc | 🔶 DIFF | see diffs.txt |
| key-rotation | python | 🔶 DIFF | see diffs.txt |
| key-rotation | rust | 🔶 DIFF | see diffs.txt |
| key-rotation | ts | ✅ PASS |  |
| multi-update | dart (self) | ✅ PASS |  |
| multi-update | java | ✅ PASS |  |
| multi-update | java-eecc | 🔶 DIFF | see diffs.txt |
| multi-update | python | 🔶 DIFF | see diffs.txt |
| multi-update | rust | 🔶 DIFF | see diffs.txt |
| multi-update | ts | ✅ PASS |  |
| multiple-update-keys | dart | ⚠️ SKIP | no did.jsonl present |
| multiple-update-keys | java | ⚠️ SKIP | no did.jsonl present |
| multiple-update-keys | java-eecc | 🔶 DIFF | see diffs.txt |
| multiple-update-keys | python | 🔶 DIFF | see diffs.txt |
| multiple-update-keys | rust | 🔶 DIFF | see diffs.txt |
| multiple-update-keys | ts | ✅ PASS |  |
| portable | dart (self) | ✅ PASS |  |
| portable | java | ✅ PASS |  |
| portable | java-eecc | 🔶 DIFF | see diffs.txt |
| portable | python | 🔶 DIFF | see diffs.txt |
| portable | rust | 🔶 DIFF | see diffs.txt |
| portable | ts | ✅ PASS |  |
| portable-move | dart (self) | ✅ PASS |  |
| portable-move | java | ✅ PASS |  |
| portable-move | java-eecc | 🔶 DIFF | see diffs.txt |
| portable-move | python | 🔶 DIFF | see diffs.txt |
| portable-move | rust | 🔶 DIFF | see diffs.txt |
| portable-move | ts | ✅ PASS |  |
| pre-rotation | dart (self) | ✅ PASS |  |
| pre-rotation | java | ✅ PASS |  |
| pre-rotation | java-eecc | 🔶 DIFF | see diffs.txt |
| pre-rotation | python | 🔶 DIFF | see diffs.txt |
| pre-rotation | rust | 🔶 DIFF | see diffs.txt |
| pre-rotation | ts | ✅ PASS |  |
| pre-rotation-consume | dart (self) | ✅ PASS |  |
| pre-rotation-consume | java | ✅ PASS |  |
| pre-rotation-consume | java-eecc | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | python | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | rust | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | ts | ✅ PASS |  |
| services | dart (self) | ✅ PASS |  |
| services | java | ✅ PASS |  |
| services | java-eecc | 🔶 DIFF | see diffs.txt |
| services | python | 🔶 DIFF | see diffs.txt |
| services | rust | 🔶 DIFF | see diffs.txt |
| services | ts | ✅ PASS |  |
| witness-threshold | dart (self) | ✅ PASS |  |
| witness-threshold | java | ✅ PASS |  |
| witness-threshold | java-eecc | 🔶 DIFF | see diffs.txt |
| witness-threshold | python | 🔶 DIFF | see diffs.txt |
| witness-threshold | rust | 🔶 DIFF | see diffs.txt |
| witness-threshold | ts | ✅ PASS |  |
| witness-update | dart (self) | ✅ PASS |  |
| witness-update | java | ✅ PASS |  |
| witness-update | java-eecc | 🔶 DIFF | see diffs.txt |
| witness-update | python | 🔶 DIFF | see diffs.txt |
| witness-update | rust | 🔶 DIFF | see diffs.txt |
| witness-update | ts | ✅ PASS |  |

---
Built from: https://github.com/decentralized-identity/didwebvh-dart @ main (2a5ab10)
