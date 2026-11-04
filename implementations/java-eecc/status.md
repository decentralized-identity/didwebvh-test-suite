# java-eecc status

Implementation: java-eecc 0.3.1

## DID Creation

| Test Case | Result | Notes |
|---|---|---|
| basic-create | ✅ PASS |  |
| basic-update | ✅ PASS |  |
| deactivate | ✅ PASS |  |
| idn-alabel-create | ✅ PASS |  |
| key-rotation | ✅ PASS |  |
| multi-update | ✅ PASS |  |
| multiple-update-keys | ✅ PASS |  |
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
| negative-dns-label-too-long/step 1 | invalidDid | ✅ PASS |  |
| negative-duplicate-witness-ids | invalidParameters | ✅ PASS |  |
| negative-fragment-leaks-into-domain/step 1 | invalidDid | ✅ PASS |  |
| negative-lowercase-pct-port-ip/step 1 | invalidDid | ✅ PASS |  |
| negative-path-traversal-did/step 1 | invalidDid | ✅ PASS |  |
| negative-pct-encoded-ip-host/step 1 | invalidDid | ✅ PASS |  |
| negative-pct-encoded-ip-host/step 2 | invalidDid | ✅ PASS |  |
| negative-pct-encoded-ip-host/step 3 | invalidDid | ✅ PASS |  |
| negative-pct-encoded-traversal/step 1 | invalidDid | ✅ PASS |  |
| negative-pct-encoded-traversal/step 2 | invalidDid | ✅ PASS |  |
| negative-pct-encoded-traversal/step 3 | invalidDid | ✅ PASS |  |
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
| basic-create | dart | 🔶 DIFF | see diffs.txt |
| basic-create | java | 🔶 DIFF | see diffs.txt |
| basic-create | java-eecc (self) | ✅ PASS |  |
| basic-create | python | 🔶 DIFF | see diffs.txt |
| basic-create | rust | 🔶 DIFF | see diffs.txt |
| basic-create | ts | 🔶 DIFF | see diffs.txt |
| basic-update | dart | 🔶 DIFF | see diffs.txt |
| basic-update | java | 🔶 DIFF | see diffs.txt |
| basic-update | java-eecc (self) | ✅ PASS |  |
| basic-update | python | 🔶 DIFF | see diffs.txt |
| basic-update | rust | 🔶 DIFF | see diffs.txt |
| basic-update | ts | 🔶 DIFF | see diffs.txt |
| deactivate | dart | 🔶 DIFF | see diffs.txt |
| deactivate | java | 🔶 DIFF | see diffs.txt |
| deactivate | java-eecc (self) | ✅ PASS |  |
| deactivate | python | 🔶 DIFF | see diffs.txt |
| deactivate | rust | 🔶 DIFF | see diffs.txt |
| deactivate | ts | 🔶 DIFF | see diffs.txt |
| idn-alabel-create | dart | 🔶 DIFF | see diffs.txt |
| idn-alabel-create | java | 🔶 DIFF | see diffs.txt |
| idn-alabel-create | java-eecc (self) | ✅ PASS |  |
| idn-alabel-create | python | 🔶 DIFF | see diffs.txt |
| idn-alabel-create | rust | 🔶 DIFF | see diffs.txt |
| idn-alabel-create | ts | 🔶 DIFF | see diffs.txt |
| key-rotation | dart | 🔶 DIFF | see diffs.txt |
| key-rotation | java | 🔶 DIFF | see diffs.txt |
| key-rotation | java-eecc (self) | ✅ PASS |  |
| key-rotation | python | 🔶 DIFF | see diffs.txt |
| key-rotation | rust | 🔶 DIFF | see diffs.txt |
| key-rotation | ts | 🔶 DIFF | see diffs.txt |
| multi-update | dart | 🔶 DIFF | see diffs.txt |
| multi-update | java | 🔶 DIFF | see diffs.txt |
| multi-update | java-eecc (self) | ✅ PASS |  |
| multi-update | python | 🔶 DIFF | see diffs.txt |
| multi-update | rust | 🔶 DIFF | see diffs.txt |
| multi-update | ts | 🔶 DIFF | see diffs.txt |
| multiple-update-keys | dart | ⚠️ SKIP | no did.jsonl present |
| multiple-update-keys | java | ⚠️ SKIP | no did.jsonl present |
| multiple-update-keys | java-eecc (self) | ✅ PASS |  |
| multiple-update-keys | python | 🔶 DIFF | see diffs.txt |
| multiple-update-keys | rust | 🔶 DIFF | see diffs.txt |
| multiple-update-keys | ts | 🔶 DIFF | see diffs.txt |
| portable | dart | 🔶 DIFF | see diffs.txt |
| portable | java | 🔶 DIFF | see diffs.txt |
| portable | java-eecc (self) | ✅ PASS |  |
| portable | python | 🔶 DIFF | see diffs.txt |
| portable | rust | 🔶 DIFF | see diffs.txt |
| portable | ts | 🔶 DIFF | see diffs.txt |
| portable-move | dart | 🔶 DIFF | see diffs.txt |
| portable-move | java | 🔶 DIFF | see diffs.txt |
| portable-move | java-eecc (self) | 🔶 DIFF | see diffs.txt |
| portable-move | python | 🔶 DIFF | see diffs.txt |
| portable-move | rust | 🔶 DIFF | see diffs.txt |
| portable-move | ts | 🔶 DIFF | see diffs.txt |
| pre-rotation | dart | 🔶 DIFF | see diffs.txt |
| pre-rotation | java | 🔶 DIFF | see diffs.txt |
| pre-rotation | java-eecc (self) | ✅ PASS |  |
| pre-rotation | python | 🔶 DIFF | see diffs.txt |
| pre-rotation | rust | 🔶 DIFF | see diffs.txt |
| pre-rotation | ts | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | dart | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | java | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | java-eecc (self) | ✅ PASS |  |
| pre-rotation-consume | python | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | rust | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | ts | 🔶 DIFF | see diffs.txt |
| services | dart | 🔶 DIFF | see diffs.txt |
| services | java | 🔶 DIFF | see diffs.txt |
| services | java-eecc (self) | ✅ PASS |  |
| services | python | 🔶 DIFF | see diffs.txt |
| services | rust | 🔶 DIFF | see diffs.txt |
| services | ts | 🔶 DIFF | see diffs.txt |
| witness-threshold | dart | 🔶 DIFF | see diffs.txt |
| witness-threshold | java | 🔶 DIFF | see diffs.txt |
| witness-threshold | java-eecc (self) | ✅ PASS |  |
| witness-threshold | python | 🔶 DIFF | see diffs.txt |
| witness-threshold | rust | 🔶 DIFF | see diffs.txt |
| witness-threshold | ts | 🔶 DIFF | see diffs.txt |
| witness-update | dart | 🔶 DIFF | see diffs.txt |
| witness-update | java | 🔶 DIFF | see diffs.txt |
| witness-update | java-eecc (self) | ✅ PASS |  |
| witness-update | python | 🔶 DIFF | see diffs.txt |
| witness-update | rust | 🔶 DIFF | see diffs.txt |
| witness-update | ts | 🔶 DIFF | see diffs.txt |

---
Built from: https://github.com/european-epc-competence-center/didwebvh @ main (ed6b592)
