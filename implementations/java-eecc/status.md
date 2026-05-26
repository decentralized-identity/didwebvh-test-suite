# java-eecc status

Implementation: java-eecc 0.3.0

## DID Creation

| Test Case | Result | Notes |
|---|---|---|
| basic-create | ✅ PASS |  |
| basic-update | ✅ PASS |  |
| deactivate | ✅ PASS |  |
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
| negative-cross-did-witness-replay | invalidDid | ❌ FAIL | resolver accepted invalid log |
| negative-did-key-body-fragment-mismatch | invalidProof | ❌ FAIL | resolver accepted invalid log |
| negative-duplicate-witness-ids | invalidParameters | ❌ FAIL | resolver accepted invalid log |
| negative-fragment-leaks-into-domain | invalidDid | ❌ FAIL | URL parser accepted invalid DID: did:webvh:Qm0000000000000000000000000000000000000000000000:127.0.0.1#x |
| negative-lowercase-pct-port-ip | invalidDid | ❌ FAIL | URL parser accepted invalid DID: did:webvh:Qm0000000000000000000000000000000000000000000000:127.0.0.1%3a8080 |
| negative-path-traversal-did | invalidDid | ❌ FAIL | URL parser accepted invalid DID: did:webvh:Qm0000000000000000000000000000000000000000000000:example.com:..:..:admin |
| negative-pct-encoded-ip-host | invalidDid | ❌ FAIL | URL parser accepted invalid DID: did:webvh:Qm0000000000000000000000000000000000000000000000:127%2E0%2E0%2E1 |
| negative-pct-encoded-traversal | invalidDid | ❌ FAIL | URL parser accepted invalid DID: did:webvh:Qm0000000000000000000000000000000000000000000000:example.com:%2E%2E:admin |
| negative-portable-scid-swap | invalidDid | ❌ FAIL | resolver accepted invalid log |
| negative-pre-rotation-omit-updatekeys | invalidParameters | ❌ FAIL | resolver accepted invalid log |
| negative-scid-mismatch-genesis | invalidDid | ❌ FAIL | resolver accepted invalid log |
| negative-unknown-method-version | invalidDid | ❌ FAIL | resolver accepted invalid log |
| negative-versiontime-future | invalidDid | ❌ FAIL | resolver accepted invalid log |
| negative-versiontime-non-monotonic | invalidDid | ❌ FAIL | resolver accepted invalid log |
| negative-wrong-cryptosuite | invalidProof | ❌ FAIL | resolver accepted invalid log |
| negative-zero-witness-threshold | invalidParameters | ❌ FAIL | resolver accepted invalid log |

## Cross-Resolution

| Test Case | Log Source | Result | Notes |
|---|---|---|---|
| basic-create | java | 🔶 DIFF | see diffs.txt |
| basic-create | java-eecc (self) | ✅ PASS |  |
| basic-create | python | 🔶 DIFF | see diffs.txt |
| basic-create | rust | 🔶 DIFF | see diffs.txt |
| basic-create | ts | 🔶 DIFF | see diffs.txt |
| basic-update | java | 🔶 DIFF | see diffs.txt |
| basic-update | java-eecc (self) | ✅ PASS |  |
| basic-update | python | 🔶 DIFF | see diffs.txt |
| basic-update | rust | 🔶 DIFF | see diffs.txt |
| basic-update | ts | 🔶 DIFF | see diffs.txt |
| deactivate | java | 🔶 DIFF | see diffs.txt |
| deactivate | java-eecc (self) | ✅ PASS |  |
| deactivate | python | 🔶 DIFF | see diffs.txt |
| deactivate | rust | 🔶 DIFF | see diffs.txt |
| deactivate | ts | 🔶 DIFF | see diffs.txt |
| key-rotation | java | 🔶 DIFF | see diffs.txt |
| key-rotation | java-eecc (self) | ✅ PASS |  |
| key-rotation | python | 🔶 DIFF | see diffs.txt |
| key-rotation | rust | 🔶 DIFF | see diffs.txt |
| key-rotation | ts | 🔶 DIFF | see diffs.txt |
| multi-update | java | 🔶 DIFF | see diffs.txt |
| multi-update | java-eecc (self) | ✅ PASS |  |
| multi-update | python | 🔶 DIFF | see diffs.txt |
| multi-update | rust | 🔶 DIFF | see diffs.txt |
| multi-update | ts | 🔶 DIFF | see diffs.txt |
| multiple-update-keys | java | ⚠️ SKIP | no did.jsonl present |
| multiple-update-keys | java-eecc (self) | ✅ PASS |  |
| multiple-update-keys | python | 🔶 DIFF | see diffs.txt |
| multiple-update-keys | rust | 🔶 DIFF | see diffs.txt |
| multiple-update-keys | ts | 🔶 DIFF | see diffs.txt |
| portable | java | 🔶 DIFF | see diffs.txt |
| portable | java-eecc (self) | ✅ PASS |  |
| portable | python | 🔶 DIFF | see diffs.txt |
| portable | rust | 🔶 DIFF | see diffs.txt |
| portable | ts | 🔶 DIFF | see diffs.txt |
| portable-move | java | 🔶 DIFF | see diffs.txt |
| portable-move | java-eecc (self) | 🔶 DIFF | see diffs.txt |
| portable-move | python | 🔶 DIFF | see diffs.txt |
| portable-move | rust | 🔶 DIFF | see diffs.txt |
| portable-move | ts | 🔶 DIFF | see diffs.txt |
| pre-rotation | java | 🔶 DIFF | see diffs.txt |
| pre-rotation | java-eecc (self) | ✅ PASS |  |
| pre-rotation | python | 🔶 DIFF | see diffs.txt |
| pre-rotation | rust | 🔶 DIFF | see diffs.txt |
| pre-rotation | ts | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | java | ⚠️ SKIP | no did.jsonl present |
| pre-rotation-consume | java-eecc (self) | ✅ PASS |  |
| pre-rotation-consume | python | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | rust | 🔶 DIFF | see diffs.txt |
| pre-rotation-consume | ts | 🔶 DIFF | see diffs.txt |
| services | java | 🔶 DIFF | see diffs.txt |
| services | java-eecc (self) | ✅ PASS |  |
| services | python | 🔶 DIFF | see diffs.txt |
| services | rust | 🔶 DIFF | see diffs.txt |
| services | ts | 🔶 DIFF | see diffs.txt |
| witness-threshold | java | 🔶 DIFF | see diffs.txt |
| witness-threshold | java-eecc (self) | ✅ PASS |  |
| witness-threshold | python | 🔶 DIFF | see diffs.txt |
| witness-threshold | rust | ⚠️ XFAIL | TS COMPAT: library resolution error (invalidDid): No valid entries in the DID log |
| witness-threshold | ts | 🔶 DIFF | see diffs.txt |
| witness-update | java | ⚠️ XFAIL | TS COMPAT: library resolution error (invalidDid): Witness epoch [1, 2] requires threshold=2 but only 1 witness(es) provided valid proofs |
| witness-update | java-eecc (self) | ✅ PASS |  |
| witness-update | python | 🔶 DIFF | see diffs.txt |
| witness-update | rust | ⚠️ XFAIL | TS COMPAT: library resolution error (invalidDid): No valid entries in the DID log |
| witness-update | ts | ⚠️ XFAIL | TS COMPAT: library resolution error (invalidDid): Witness epoch [1, 2] requires threshold=2 but only 1 witness(es) provided valid proofs |
