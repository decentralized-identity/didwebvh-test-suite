# java status

Implementation: didwebvh-java 0.2.0-SNAPSHOT

## DID Creation

| Test Case | Result | Notes |
|---|---|---|
| basic-create | ✅ PASS |  |
| basic-update | ✅ PASS |  |
| deactivate | ✅ PASS |  |
| key-rotation | ✅ PASS |  |
| multi-update | ✅ PASS |  |
| multiple-update-keys | ⚠️ SKIP | multiple-update-keys at create time not supported by didwebvh-java API |
| portable | ✅ PASS |  |
| portable-move | ✅ PASS |  |
| pre-rotation | ✅ PASS |  |
| pre-rotation-consume | ⚠️ SKIP | resolver rejected generated log: Invalid DID log: signing key not in active updateKeys at entry 2 |
| services | ✅ PASS |  |
| witness-threshold | ✅ PASS |  |
| witness-update | ✅ PASS |  |

## Negative Resolution

| Test Case | Expected Error | Result | Notes |
|---|---|---|---|
| negative-cross-did-witness-replay | invalidDid | ✅ PASS |  |
| negative-did-key-body-fragment-mismatch | invalidProof | ✅ PASS |  |
| negative-duplicate-witness-ids | invalidParameters | ✅ PASS |  |
| negative-fragment-leaks-into-domain | invalidDid | ✅ PASS |  |
| negative-lowercase-pct-port-ip | invalidDid | ✅ PASS |  |
| negative-path-traversal-did | invalidDid | ✅ PASS |  |
| negative-pct-encoded-ip-host | invalidDid | ✅ PASS |  |
| negative-pct-encoded-traversal | invalidDid | ✅ PASS |  |
| negative-portable-scid-swap | invalidDid | ✅ PASS |  |
| negative-pre-rotation-omit-updatekeys | invalidParameters | ✅ PASS |  |
| negative-scid-mismatch-genesis | invalidDid | ✅ PASS |  |
| negative-unknown-method-version | invalidDid | ✅ PASS |  |
| negative-versiontime-future | invalidDid | ✅ PASS |  |
| negative-versiontime-non-monotonic | invalidDid | ✅ PASS |  |
| negative-wrong-cryptosuite | invalidProof | ✅ PASS |  |
| negative-zero-witness-threshold | invalidParameters | ✅ PASS |  |

## Cross-Resolution

| Test Case | Log Source | Result | Notes |
|---|---|---|---|
| basic-create | java (self) | ✅ PASS |  |
| basic-create | java-eecc | 🔶 DIFF | see diffs.txt |
| basic-create | python | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| basic-create | rust | 🔶 DIFF | see diffs.txt |
| basic-create | ts | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| basic-update | java (self) | ✅ PASS |  |
| basic-update | java-eecc | 🔶 DIFF | see diffs.txt |
| basic-update | python | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| basic-update | rust | 🔶 DIFF | see diffs.txt |
| basic-update | ts | ⚠️ XFAIL | TS COMPAT: nextKeyHashes:[] — TS serialises empty list; Java library may reject on update validation |
| deactivate | java (self) | ✅ PASS |  |
| deactivate | java-eecc | 🔶 DIFF | see diffs.txt |
| deactivate | python | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| deactivate | rust | 🔶 DIFF | see diffs.txt |
| deactivate | ts | ⚠️ XFAIL | TS COMPAT: nextKeyHashes:[] — TS serialises empty list; Java library may reject on update validation |
| key-rotation | java (self) | ✅ PASS |  |
| key-rotation | java-eecc | 🔶 DIFF | see diffs.txt |
| key-rotation | python | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| key-rotation | rust | 🔶 DIFF | see diffs.txt |
| key-rotation | ts | ⚠️ XFAIL | TS COMPAT: nextKeyHashes:[] — TS serialises empty list; Java library may reject on update validation |
| multi-update | java (self) | ✅ PASS |  |
| multi-update | java-eecc | 🔶 DIFF | see diffs.txt |
| multi-update | python | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| multi-update | rust | 🔶 DIFF | see diffs.txt |
| multi-update | ts | ⚠️ XFAIL | TS COMPAT: nextKeyHashes:[] — TS serialises empty list; Java library may reject on update validation |
| multiple-update-keys | java | ⚠️ SKIP | no did.jsonl present |
| multiple-update-keys | java-eecc | 🔶 DIFF | see diffs.txt |
| multiple-update-keys | python | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| multiple-update-keys | rust | 🔶 DIFF | see diffs.txt |
| multiple-update-keys | ts | ⚠️ XFAIL | TS COMPAT: nextKeyHashes:[] — TS serialises empty list; Java library may reject on update validation |
| portable | java (self) | ✅ PASS |  |
| portable | java-eecc | 🔶 DIFF | see diffs.txt |
| portable | python | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| portable | rust | 🔶 DIFF | see diffs.txt |
| portable | ts | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| portable-move | java (self) | ✅ PASS |  |
| portable-move | java-eecc | 🔶 DIFF | see diffs.txt |
| portable-move | python | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| portable-move | rust | 🔶 DIFF | see diffs.txt |
| portable-move | ts | ⚠️ XFAIL | TS COMPAT: nextKeyHashes:[] — TS serialises empty list; Java library may reject on update validation |
| pre-rotation | java (self) | ✅ PASS |  |
| pre-rotation | java-eecc | 🔶 DIFF | see diffs.txt |
| pre-rotation | python | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| pre-rotation | rust | 🔶 DIFF | see diffs.txt |
| pre-rotation | ts | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| pre-rotation-consume | java | ⚠️ SKIP | no did.jsonl present |
| pre-rotation-consume | java-eecc | ❌ FAIL | resolve error: Invalid DID log: signing key not in active updateKeys at entry 2 |
| pre-rotation-consume | python | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| pre-rotation-consume | rust | ❌ FAIL | resolve error: Invalid DID log: signing key not in active updateKeys at entry 2 |
| pre-rotation-consume | ts | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| services | java (self) | ✅ PASS |  |
| services | java-eecc | 🔶 DIFF | see diffs.txt |
| services | python | ❌ FAIL | LIB BUG: didwebvh-java 0.2.0 NPEs on witness:{} in parameters (Python/TS write empty witness object; library expects absent field) |
| services | rust | 🔶 DIFF | see diffs.txt |
| services | ts | ⚠️ XFAIL | TS COMPAT: nextKeyHashes:[] — TS serialises empty list; Java library may reject on update validation |
| witness-threshold | java (self) | ✅ PASS |  |
| witness-threshold | java-eecc | 🔶 DIFF | see diffs.txt |
| witness-threshold | python | 🔶 DIFF | see diffs.txt |
| witness-threshold | rust | ❌ FAIL | resolve error: Invalid witness proofs: insufficient witness proofs for entry 1-QmdDTDXMshdRwAbnN7b22BKAUy1CvQsq6y6zQqSP7reDJ7: need 1, got 0 |
| witness-threshold | ts | 🔶 DIFF | see diffs.txt |
| witness-update | java (self) | ✅ PASS |  |
| witness-update | java-eecc | 🔶 DIFF | see diffs.txt |
| witness-update | python | 🔶 DIFF | see diffs.txt |
| witness-update | rust | ❌ FAIL | resolve error: Invalid witness proofs: missing witness proof for entry 1-QmayLd7TkamnZhA8EbQCP6aUvU7YvGW1xW2iauRFC47ziP |
| witness-update | ts | ⚠️ XFAIL | TS COMPAT: nextKeyHashes:[] — TS serialises empty list; Java library may reject on update validation |
