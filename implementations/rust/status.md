# rust status

Implementation: didwebvh-rs 0.5.3 @ ad614934

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
| negative-duplicate-witness-ids | invalidParameters | ✅ PASS |  |
| negative-portable-scid-swap | invalidDid | ✅ PASS |  |
| negative-pre-rotation-omit-updatekeys | invalidParameters | ✅ PASS |  |
| negative-scid-mismatch-genesis | invalidDid | ✅ PASS |  |
| negative-unknown-method-version | invalidDid | ✅ PASS |  |
| negative-versiontime-future | invalidDid | ✅ PASS |  |
| negative-versiontime-non-monotonic | invalidDid | ✅ PASS |  |
| negative-witness-update-threshold-not-met | invalidDid | ✅ PASS |  |
| negative-wrong-cryptosuite | invalidProof | ✅ PASS |  |
| negative-zero-witness-threshold | invalidParameters | ✅ PASS |  |

## Pre-network URL assertions

| Test Case | Expected | Result | Notes |
|---|---|---|---|
| idn-did-to-https/resolve-did/1 | https://xn--jp-cd2fp15c.xn--fsq.jp/.well-known/did.jsonl | ✅ PASS | fetch URL: https://xn--jp-cd2fp15c.xn--fsq.jp/.well-known/did.jsonl |
| idn-did-to-https/resolve-did/2 | https://xn--jp-cd2fp15c.xn--fsq.jp/%E7%94%A8%E6%88%B7/did.jsonl | ✅ PASS | fetch URL: https://xn--jp-cd2fp15c.xn--fsq.jp/%E7%94%A8%E6%88%B7/did.jsonl |
| idn-did-to-https/resolve-did/3 | https://xn--jp-cd2fp15c.xn--fsq.jp/%E7%94%A8%E6%88%B7/did.jsonl | 🔶 DIFF | expected https://xn--jp-cd2fp15c.xn--fsq.jp/%E7%94%A8%E6%88%B7/did.jsonl; got https://xn--jp-cd2fp15c.xn--fsq.jp/%e7%94%a8%e6%88%b7/did.jsonl |
| ldh-label-edge-cases/resolve-did/1 | https://x.example.com/.well-known/did.jsonl | ✅ PASS | fetch URL: https://x.example.com/.well-known/did.jsonl |
| ldh-label-edge-cases/resolve-did/2 | https://3m.example.com/.well-known/did.jsonl | ✅ PASS | fetch URL: https://3m.example.com/.well-known/did.jsonl |
| negative-dns-label-too-long/resolve-did/1 | invalidDid | ❌ FAIL | expected invalidDid; constructed https://aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.example.com/.well-known/did.jsonl |
| negative-fragment-leaks-into-domain/resolve-did/1 | invalidDid | ✅ PASS | rejected before network: Invalid method identifier: Invalid URL: IP addresses are not allowed, use a domain name instead: 127.0.0.1 |
| negative-lowercase-pct-port-ip/resolve-did/1 | invalidDid | ✅ PASS | rejected before network: Invalid method identifier: Invalid URL: IP addresses are not allowed, use a domain name instead: 127.0.0.1 |
| negative-path-traversal-did/resolve-did/1 | invalidDid | ✅ PASS | rejected before network: Invalid method identifier: Invalid URL: path segment ("..") is not allowed |
| negative-pct-encoded-ip-host/resolve-did/1 | invalidDid | ✅ PASS | rejected before network: Invalid method identifier: Invalid URL: IP addresses are not allowed, use a domain name instead: 127.0.0.1 |
| negative-pct-encoded-ip-host/resolve-did/2 | invalidDid | ✅ PASS | rejected before network: Invalid method identifier: Invalid URL: IP addresses are not allowed, use a domain name instead: 127.0.0.1 |
| negative-pct-encoded-ip-host/resolve-did/3 | invalidDid | ✅ PASS | rejected before network: Invalid method identifier: Invalid URL: IP addresses are not allowed, use a domain name instead: 169.254.169.254 |
| negative-pct-encoded-traversal/resolve-did/1 | invalidDid | ✅ PASS | rejected before network: Invalid method identifier: Invalid URL: path segment ("%2E%2E") is not allowed |
| negative-pct-encoded-traversal/resolve-did/2 | invalidDid | ✅ PASS | rejected before network: Invalid method identifier: Invalid URL: path segment ("%2e%2e") is not allowed |
| negative-pct-encoded-traversal/resolve-did/3 | invalidDid | ✅ PASS | rejected before network: Invalid method identifier: Invalid URL: path segment ("a%2Fb") is not allowed |
| negative-raw-unicode-domain/resolve-did/1 | invalidDid | ❌ FAIL | expected invalidDid; constructed https://xn--jp-cd2fp15c.xn--fsq.jp/.well-known/did.jsonl |
| negative-raw-unicode-domain/resolve-did/2 | invalidDid | ❌ FAIL | expected invalidDid; constructed https://xn--bcher-kva.example.com/.well-known/did.jsonl |

## URL probes (non-conformance)

| Test Case | Expected | Result | Notes |
|---|---|---|---|
| idn-pct-ulabel-probe/resolve-did/1 | https://xn--jp-cd2fp15c.xn--fsq.jp/.well-known/did.jsonl | ✅ PASS | fetch URL: https://xn--jp-cd2fp15c.xn--fsq.jp/.well-known/did.jsonl |
| idn-pct-ulabel-probe/resolve-did/2 | https://xn--bcher-kva.example.com/.well-known/did.jsonl | ✅ PASS | fetch URL: https://xn--bcher-kva.example.com/.well-known/did.jsonl |
| idn-pct-ulabel-probe/resolve-did/3 | https://xn--bcher-kva.example.com/.well-known/did.jsonl | ✅ PASS | fetch URL: https://xn--bcher-kva.example.com/.well-known/did.jsonl |
| idn-pct-ulabel-probe/resolve-did/4 | https://xn--fa-hia.de/.well-known/did.jsonl | ✅ PASS | fetch URL: https://xn--fa-hia.de/.well-known/did.jsonl |
| path-pct-unreserved-probe/resolve-did/1 | https://example.com/a~b/did.jsonl | 🔶 DIFF | expected https://example.com/a~b/did.jsonl; got https://example.com/a%7Eb/did.jsonl |
| path-pct-unreserved-probe/resolve-did/2 | https://example.com/a~b/did.jsonl | 🔶 DIFF | expected https://example.com/a~b/did.jsonl; got https://example.com/a%7eb/did.jsonl |
| path-pct-unreserved-probe/resolve-did/3 | https://example.com/Ab-c_d.e/did.jsonl | 🔶 DIFF | expected https://example.com/Ab-c_d.e/did.jsonl; got https://example.com/%41b%2Dc%5Fd%2Ee/did.jsonl |

## Cross-Resolution

| Test Case | Log Source | Result | Notes |
|---|---|---|---|
| basic-create | dart | 🔶 DIFF | see diffs.txt |
| basic-create | java | 🔶 DIFF | see diffs.txt |
| basic-create | java-eecc | 🔶 DIFF | see diffs.txt |
| basic-create | python | 🔶 DIFF | see diffs.txt |
| basic-create | rust (self) | ✅ PASS |  |
| basic-create | ts | 🔶 DIFF | see diffs.txt |
| basic-update | dart | 🔶 DIFF | see diffs.txt |
| basic-update | java | 🔶 DIFF | see diffs.txt |
| basic-update | java-eecc | 🔶 DIFF | see diffs.txt |
| basic-update | python | 🔶 DIFF | see diffs.txt |
| basic-update | rust (self) | ✅ PASS |  |
| basic-update | ts | 🔶 DIFF | see diffs.txt |
| deactivate | dart | 🔶 DIFF | see diffs.txt |
| deactivate | java | 🔶 DIFF | see diffs.txt |
| deactivate | java-eecc | 🔶 DIFF | see diffs.txt |
| deactivate | python | 🔶 DIFF | see diffs.txt |
| deactivate | rust (self) | ✅ PASS |  |
| deactivate | ts | 🔶 DIFF | see diffs.txt |
| idn-alabel-create | dart | 🔶 DIFF | see diffs.txt |
| idn-alabel-create | java | 🔶 DIFF | see diffs.txt |
| idn-alabel-create | java-eecc | 🔶 DIFF | see diffs.txt |
| idn-alabel-create | python | 🔶 DIFF | see diffs.txt |
| idn-alabel-create | rust (self) | ✅ PASS |  |
| idn-alabel-create | ts | 🔶 DIFF | see diffs.txt |
| key-rotation | dart | 🔶 DIFF | see diffs.txt |
| key-rotation | java | 🔶 DIFF | see diffs.txt |
| key-rotation | java-eecc | 🔶 DIFF | see diffs.txt |
| key-rotation | python | 🔶 DIFF | see diffs.txt |
| key-rotation | rust (self) | ✅ PASS |  |
| key-rotation | ts | 🔶 DIFF | see diffs.txt |
| multi-update | dart | 🔶 DIFF | see diffs.txt |
| multi-update | java | 🔶 DIFF | see diffs.txt |
| multi-update | java-eecc | 🔶 DIFF | see diffs.txt |
| multi-update | python | 🔶 DIFF | see diffs.txt |
| multi-update | rust (self) | ✅ PASS |  |
| multi-update | ts | 🔶 DIFF | see diffs.txt |
| multiple-update-keys | dart | ⚠️ SKIP | no did.jsonl |
| multiple-update-keys | java | ⚠️ SKIP | no did.jsonl |
| multiple-update-keys | java-eecc | 🔶 DIFF | see diffs.txt |
| multiple-update-keys | python | 🔶 DIFF | see diffs.txt |
| multiple-update-keys | rust (self) | ✅ PASS |  |
| multiple-update-keys | ts | 🔶 DIFF | see diffs.txt |
| portable | dart | 🔶 DIFF | see diffs.txt |
| portable | java | 🔶 DIFF | see diffs.txt |
| portable | java-eecc | 🔶 DIFF | see diffs.txt |
| portable | python | 🔶 DIFF | see diffs.txt |
| portable | rust (self) | ✅ PASS |  |
| portable | ts | 🔶 DIFF | see diffs.txt |
| portable-move | dart | 🔶 DIFF | see diffs.txt |
| portable-move | java | 🔶 DIFF | see diffs.txt |
| portable-move | java-eecc | 🔶 DIFF | see diffs.txt |
| portable-move | python | 🔶 DIFF | see diffs.txt |
| portable-move | rust (self) | ✅ PASS |  |
| portable-move | ts | 🔶 DIFF | see diffs.txt |
| pre-rotation | dart | 🔶 DIFF | see diffs.txt |
| pre-rotation | java | 🔶 DIFF | see diffs.txt |
| pre-rotation | java-eecc | 🔶 DIFF | see diffs.txt |
| pre-rotation | python | 🔶 DIFF | see diffs.txt |
| pre-rotation | rust (self) | ✅ PASS |  |
| pre-rotation | ts | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | dart | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | java | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | java-eecc | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | python | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | rust (self) | ✅ PASS |  |
| pre-rotation-consume | ts | 🔶 DIFF | see diffs.txt |
| services | dart | 🔶 DIFF | see diffs.txt |
| services | java | 🔶 DIFF | see diffs.txt |
| services | java-eecc | 🔶 DIFF | see diffs.txt |
| services | python | 🔶 DIFF | see diffs.txt |
| services | rust (self) | ✅ PASS |  |
| services | ts | 🔶 DIFF | see diffs.txt |
| witness-threshold | dart | 🔶 DIFF | see diffs.txt |
| witness-threshold | java | 🔶 DIFF | see diffs.txt |
| witness-threshold | java-eecc | 🔶 DIFF | see diffs.txt |
| witness-threshold | python | 🔶 DIFF | see diffs.txt |
| witness-threshold | rust (self) | ✅ PASS |  |
| witness-threshold | ts | 🔶 DIFF | see diffs.txt |
| witness-update | dart | 🔶 DIFF | see diffs.txt |
| witness-update | java | 🔶 DIFF | see diffs.txt |
| witness-update | java-eecc | 🔶 DIFF | see diffs.txt |
| witness-update | python | 🔶 DIFF | see diffs.txt |
| witness-update | rust (self) | ✅ PASS |  |
| witness-update | ts | 🔶 DIFF | see diffs.txt |

---
Built from: https://github.com/decentralized-identity/didwebvh-rs @ main (8e42c36)
