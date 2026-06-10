//! Rust negative-vector generator for did:webvh compliance scenarios.
//!
//! Reads vectors/negative-*/script.yaml and writes:
//!   vectors/<scenario>/rust/did.jsonl
//!   vectors/<scenario>/rust/resolutionResult.json
//!   vectors/<scenario>/rust/did-witness.json (if witness proofs are scripted)

use std::path::Path;
use std::sync::Arc;

use ahash::HashMap;
use affinidi_data_integrity::{DataIntegrityProof, SignOptions};
use base58::ToBase58;
use chrono::{DateTime, FixedOffset};
use didwebvh_rs::prelude::{DIDWebVHState, Multibase, Parameters, Secret};
use didwebvh_rs::SCID_HOLDER;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const VECTORS_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../vectors");

#[derive(Debug, Deserialize)]
struct Script {
    #[serde(default)]
    negative: bool,
    #[serde(default)]
    keys: Vec<KeyDef>,
    #[serde(default)]
    steps: Vec<Step>,
}

#[derive(Debug, Deserialize)]
struct KeyDef {
    id: String,
    #[serde(rename = "type")]
    key_type: String,
    seed: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Step {
    op: String,
    #[serde(default)]
    domain: Option<String>,
    #[serde(default)]
    signer: Option<String>,
    #[serde(default)]
    timestamp: Option<String>,
    #[serde(default)]
    params: Option<StepParams>,
    #[serde(default)]
    entry: Option<usize>,
    #[serde(default)]
    mutation: Option<String>,
    #[serde(default)]
    field: Option<String>,
    #[serde(default)]
    value: Option<Value>,
    #[serde(default)]
    when: Option<String>,
    #[serde(default)]
    did: Option<String>,
    #[serde(rename = "expectError", default)]
    expect_error: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
struct StepParams {
    #[serde(rename = "updateKeys", default)]
    update_keys: Vec<String>,
    #[serde(rename = "nextKeyHashes", default)]
    next_key_hashes: Vec<String>,
    #[serde(default)]
    portable: Option<bool>,
    #[serde(default)]
    witness: Option<WitnessConfig>,
    #[serde(default)]
    services: Vec<Value>,
    #[serde(rename = "verificationMethods", default)]
    verification_methods: Vec<Value>,
    #[serde(rename = "alsoKnownAs", default)]
    also_known_as: Vec<String>,
    #[serde(default)]
    context: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct WitnessConfig {
    threshold: u32,
    witnesses: Vec<WitnessEntry>,
}

#[derive(Debug, Deserialize, Clone)]
struct WitnessEntry {
    id: String,
}

#[derive(Clone)]
struct KeyInfo {
    secret: Secret,
    pub_multibase: String,
}

fn build_key_registry(key_defs: &[KeyDef]) -> Result<HashMap<String, KeyInfo>, String> {
    let mut map = HashMap::default();
    for kd in key_defs {
        if kd.key_type != "ed25519" {
            return Err(format!(
                "unsupported key type '{}' for '{}'",
                kd.key_type, kd.id
            ));
        }
        let seed_bytes = hex::decode(&kd.seed)
            .map_err(|e| format!("bad seed hex for '{}': {e}", kd.id))?;
        let seed: [u8; 32] = seed_bytes
            .try_into()
            .map_err(|_| format!("seed for '{}' must be 32 bytes", kd.id))?;

        let mut secret = Secret::generate_ed25519(None, Some(&seed));
        let pub_mb = secret
            .get_public_keymultibase()
            .map_err(|e| format!("pubkey for '{}': {e}", kd.id))?;
        secret.id = format!("did:key:{pub_mb}#{pub_mb}");

        map.insert(
            kd.id.clone(),
            KeyInfo {
                secret,
                pub_multibase: pub_mb,
            },
        );
    }
    Ok(map)
}

fn build_parameters(p: &StepParams, keys: &HashMap<String, KeyInfo>) -> Result<Parameters, String> {
    let update_keys: Vec<Multibase> = p
        .update_keys
        .iter()
        .map(|kid| {
            keys.get(kid)
                .map(|k| Multibase::new(k.pub_multibase.clone()))
                .ok_or_else(|| format!("unknown key '{kid}'"))
        })
        .collect::<Result<_, _>>()?;

    let next_key_hashes: Vec<Multibase> = p
        .next_key_hashes
        .iter()
        .map(|kid| {
            let info = keys
                .get(kid)
                .ok_or_else(|| format!("unknown key '{kid}'"))?;
            let hash = Secret::base58_hash_string(&info.pub_multibase)
                .map_err(|e| format!("hash for '{kid}': {e}"))?;
            Ok(Multibase::new(hash))
        })
        .collect::<Result<_, String>>()?;

    let witness = if let Some(wc) = &p.witness {
        let mut builder = didwebvh_rs::witness::Witnesses::builder().threshold(wc.threshold);
        for we in &wc.witnesses {
            let witness_id = keys
                .get(&we.id)
                .map(|k| format!("did:key:{}", k.pub_multibase))
                .unwrap_or_else(|| we.id.clone());
            builder = builder.witness(Multibase::new(witness_id));
        }
        Some(Arc::new(
            builder
                .build()
                .map_err(|e| format!("build witnesses: {e:?}"))?,
        ))
    } else {
        None
    };

    Ok(Parameters {
        update_keys: if update_keys.is_empty() {
            None
        } else {
            Some(Arc::new(update_keys))
        },
        next_key_hashes: if next_key_hashes.is_empty() {
            None
        } else {
            Some(Arc::new(next_key_hashes))
        },
        portable: p.portable,
        witness,
        ..Default::default()
    })
}

fn build_document(did: &str, params: &StepParams, keys: &HashMap<String, KeyInfo>) -> Value {
    let vms: Vec<Value> = params
        .update_keys
        .iter()
        .filter_map(|kid| keys.get(kid))
        .map(|k| {
            let suffix = &k.pub_multibase[k.pub_multibase.len().saturating_sub(8)..];
            json!({
                "id": format!("{did}#{suffix}"),
                "type": "Multikey",
                "controller": did,
                "publicKeyMultibase": k.pub_multibase,
            })
        })
        .chain(params.verification_methods.iter().cloned())
        .collect();

    let auth_refs: Vec<Value> = params
        .update_keys
        .iter()
        .filter_map(|kid| keys.get(kid))
        .map(|k| {
            let suffix = &k.pub_multibase[k.pub_multibase.len().saturating_sub(8)..];
            json!(format!("{did}#{suffix}"))
        })
        .collect();

    let mut doc = json!({
        "@context": ["https://www.w3.org/ns/did/v1", "https://w3id.org/security/multikey/v1"],
        "id": did,
        "controller": did,
        "verificationMethod": vms,
        "authentication": auth_refs,
        "assertionMethod": [],
        "keyAgreement": [],
        "capabilityDelegation": [],
        "capabilityInvocation": [],
    });

    if !params.services.is_empty() {
        doc["service"] = json!(params.services);
    }
    if !params.also_known_as.is_empty() {
        doc["alsoKnownAs"] = json!(params.also_known_as);
    }
    for c in &params.context {
        if let Some(ctx) = doc["@context"].as_array_mut() {
            ctx.push(json!(c));
        }
    }

    doc
}

fn merge_doc_with_params(
    base_doc: &Value,
    did: &str,
    params: &StepParams,
    keys: &HashMap<String, KeyInfo>,
) -> Value {
    let mut doc = base_doc.clone();

    if !doc.is_object() {
        return build_document(did, params, keys);
    }

    doc["id"] = json!(did);

    // If this step explicitly requests key material, replace auth/vm accordingly.
    if !params.update_keys.is_empty() || !params.verification_methods.is_empty() {
        let replacement = build_document(did, params, keys);
        if let Some(vm) = replacement.get("verificationMethod") {
            doc["verificationMethod"] = vm.clone();
        }
        if let Some(auth) = replacement.get("authentication") {
            doc["authentication"] = auth.clone();
        }
    }

    // Optional fields explicitly provided by the script override base state.
    if !params.services.is_empty() {
        doc["service"] = json!(params.services);
    }
    if !params.also_known_as.is_empty() {
        doc["alsoKnownAs"] = json!(params.also_known_as);
    }
    if !params.context.is_empty() {
        let mut ctx = vec![
            json!("https://www.w3.org/ns/did/v1"),
            json!("https://w3id.org/security/multikey/v1"),
        ];
        for c in &params.context {
            ctx.push(json!(c));
        }
        doc["@context"] = Value::Array(ctx);
    }

    doc
}

fn compute_entry_hash(entry_without_proof: &Value) -> Result<String, String> {
    let canonical = serde_json_canonicalizer::to_string(entry_without_proof)
        .map_err(|e| format!("canonicalize entry: {e}"))?;
    let digest = Sha256::digest(canonical.as_bytes());
    let mut multihash = Vec::with_capacity(2 + digest.len());
    multihash.push(0x12);
    multihash.push(0x20);
    multihash.extend_from_slice(digest.as_slice());
    Ok(multihash.to_base58())
}

fn replace_scid_placeholder(value: &Value, scid: &str) -> Result<Value, String> {
    let s = serde_json::to_string(value).map_err(|e| format!("serialize JSON: {e}"))?;
    let replaced = s.replace(SCID_HOLDER, scid);
    serde_json::from_str(&replaced).map_err(|e| format!("parse replaced JSON: {e}"))
}

fn build_update_parameters_json(
    params: &StepParams,
    current_update_key_ids: &[String],
    keys: &HashMap<String, KeyInfo>,
) -> Result<Value, String> {
    let mut out = serde_json::Map::new();

    let effective_update_keys = if params.update_keys.is_empty() {
        current_update_key_ids.to_vec()
    } else {
        params.update_keys.clone()
    };

    let update_keys = effective_update_keys
        .iter()
        .map(|kid| {
            keys.get(kid)
                .map(|k| k.pub_multibase.clone())
                .ok_or_else(|| format!("unknown key '{kid}'"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    out.insert("updateKeys".to_string(), json!(update_keys));

    let next_key_hashes = params
        .next_key_hashes
        .iter()
        .map(|kid| {
            let info = keys
                .get(kid)
                .ok_or_else(|| format!("unknown key '{kid}'"))?;
            Secret::base58_hash_string(&info.pub_multibase)
                .map_err(|e| format!("nextKeyHashes hash for '{kid}': {e}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    out.insert("nextKeyHashes".to_string(), json!(next_key_hashes));

    // TS negative update artifacts serialize explicit empty witness/watchers.
    if let Some(wc) = &params.witness {
        let witnesses: Vec<Value> = wc
            .witnesses
            .iter()
            .map(|w| {
                let pub_mb = keys
                    .get(&w.id)
                    .map(|k| k.pub_multibase.clone())
                    .unwrap_or_else(|| w.id.clone());
                json!({"id": format!("did:key:{pub_mb}")})
            })
            .collect();
        out.insert(
            "witness".to_string(),
            json!({"threshold": wc.threshold, "witnesses": witnesses}),
        );
    } else {
        out.insert("witness".to_string(), json!({}));
    }

    out.insert("watchers".to_string(), json!([]));

    if let Some(portable) = params.portable {
        out.insert("portable".to_string(), json!(portable));
    }

    Ok(Value::Object(out))
}

async fn create_update_bypass(
    previous_entry: &Value,
    timestamp: &str,
    params: &StepParams,
    current_update_key_ids: &[String],
    doc: Value,
    signer: &Secret,
    keys: &HashMap<String, KeyInfo>,
) -> Result<Value, String> {
    let prev_version_id = previous_entry
        .get("versionId")
        .and_then(Value::as_str)
        .ok_or("update bypass: previous entry missing versionId")?;
    let prev_number = prev_version_id
        .split('-')
        .next()
        .ok_or("update bypass: malformed previous versionId")?
        .parse::<u32>()
        .map_err(|e| format!("update bypass: parse previous version number: {e}"))?;
    let new_version_number = prev_number + 1;

    let mut entry = json!({
        "versionId": prev_version_id,
        "versionTime": timestamp,
        "parameters": build_update_parameters_json(params, current_update_key_ids, keys)?,
        "state": doc,
    });

    let entry_hash = compute_entry_hash(&entry)?;
    entry["versionId"] = json!(format!("{new_version_number}-{entry_hash}"));

    let proof = DataIntegrityProof::sign(&entry, signer, SignOptions::new())
        .await
        .map_err(|e| format!("update bypass sign proof: {e}"))?;
    entry["proof"] = json!([proof]);

    Ok(entry)
}

async fn create_genesis_bypass(
    domain: &str,
    timestamp: &str,
    params: &StepParams,
    signer: &Secret,
    keys: &HashMap<String, KeyInfo>,
) -> Result<Value, String> {
    let update_keys: Vec<String> = if params.update_keys.is_empty() {
        vec![signer
            .get_public_keymultibase()
            .map_err(|e| format!("signer pubkey: {e}"))?]
    } else {
        params
            .update_keys
            .iter()
            .map(|kid| {
                keys.get(kid)
                    .map(|k| k.pub_multibase.clone())
                    .ok_or_else(|| format!("unknown key '{kid}'"))
            })
            .collect::<Result<_, _>>()?
    };

    let next_key_hashes: Vec<String> = params
        .next_key_hashes
        .iter()
        .map(|kid| {
            let info = keys
                .get(kid)
                .ok_or_else(|| format!("unknown key '{kid}'"))?;
            Secret::base58_hash_string(&info.pub_multibase)
                .map_err(|e| format!("nextKeyHashes hash for '{kid}': {e}"))
        })
        .collect::<Result<_, _>>()?;

    let witness_json = if let Some(wc) = &params.witness {
        let witnesses: Vec<Value> = wc
            .witnesses
            .iter()
            .map(|w| {
                let pub_mb = keys
                    .get(&w.id)
                    .map(|k| k.pub_multibase.clone())
                    .unwrap_or_else(|| w.id.clone());
                json!({"id": format!("did:key:{pub_mb}")})
            })
            .collect();
        json!({"threshold": wc.threshold, "witnesses": witnesses})
    } else {
        json!({})
    };

    let did_placeholder = format!("did:webvh:{SCID_HOLDER}:{domain}");
    let initial = json!({
        "versionId": SCID_HOLDER,
        "versionTime": timestamp,
        "parameters": {
            "method": "did:webvh:1.0",
            "scid": SCID_HOLDER,
            "updateKeys": update_keys,
            "portable": params.portable.unwrap_or(false),
            "nextKeyHashes": next_key_hashes,
            "watchers": [],
            "witness": witness_json,
            "deactivated": false,
        },
        "state": build_document(&did_placeholder, params, keys),
    });

    let scid = compute_entry_hash(&initial)?;
    let mut prelim = replace_scid_placeholder(&initial, &scid)?;
    let entry_hash = compute_entry_hash(&prelim)?;
    prelim["versionId"] = json!(format!("1-{entry_hash}"));

    let proof = DataIntegrityProof::sign(&prelim, signer, SignOptions::new())
        .await
        .map_err(|e| format!("sign genesis bypass proof: {e}"))?;
    prelim["proof"] = json!([proof]);

    Ok(prelim)
}

fn key_id_placeholder(key_id: &str) -> String {
    format!("{{{}}}", key_id.replace('-', "_").to_uppercase())
}

fn substitute_placeholders(value: &Value, keys: &HashMap<String, KeyInfo>) -> Value {
    match value {
        Value::String(s) => {
            let mut out = s.clone();
            for (key_id, info) in keys {
                out = out.replace(&key_id_placeholder(key_id), &info.pub_multibase);
            }
            Value::String(out)
        }
        Value::Array(arr) => Value::Array(
            arr.iter()
                .map(|v| substitute_placeholders(v, keys))
                .collect(),
        ),
        Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (k, v) in map {
                out.insert(k.clone(), substitute_placeholders(v, keys));
            }
            Value::Object(out)
        }
        _ => value.clone(),
    }
}

fn apply_mutation(
    mut entry: Value,
    mutation: &str,
    field: Option<&str>,
    value: Option<&Value>,
    keys: &HashMap<String, KeyInfo>,
) -> Result<Value, String> {
    let substituted = value.map(|v| substitute_placeholders(v, keys));

    match mutation {
        "replace-state-id-scid" => {
            let id = entry
                .pointer("/state/id")
                .and_then(Value::as_str)
                .ok_or("replace-state-id-scid: missing state.id")?
                .to_string();
            let mut parts: Vec<String> = id.split(':').map(ToString::to_string).collect();
            if parts.len() >= 3 {
                let replacement = substituted
                    .as_ref()
                    .and_then(Value::as_str)
                    .ok_or("replace-state-id-scid: value must be string")?;
                parts[2] = replacement.to_string();
                entry["state"]["id"] = json!(parts.join(":"));
            }
        }
        "replace-version-time" => {
            let replacement = substituted
                .as_ref()
                .and_then(Value::as_str)
                .ok_or("replace-version-time: value must be string")?;
            entry["versionTime"] = json!(replacement);
        }
        "replace-parameter" => {
            let name = field
                .map(ToString::to_string)
                .or_else(|| substituted.as_ref().and_then(Value::as_str).map(ToString::to_string))
                .ok_or("replace-parameter: need field or string value")?;
            let val = substituted.unwrap_or(Value::Null);
            if !entry["parameters"].is_object() {
                entry["parameters"] = json!({});
            }
            entry["parameters"][name] = val;
        }
        "drop-parameter" => {
            let name = field
                .map(ToString::to_string)
                .or_else(|| substituted.as_ref().and_then(Value::as_str).map(ToString::to_string))
                .ok_or("drop-parameter: need field or string value")?;
            if let Some(obj) = entry["parameters"].as_object_mut() {
                obj.remove(&name);
            }
        }
        "replace-proof-field" => {
            let key = field.ok_or("replace-proof-field: missing field")?;
            let val = substituted.unwrap_or(Value::Null);
            let has_first_proof = entry
                .get("proof")
                .and_then(Value::as_array)
                .map(|arr| !arr.is_empty())
                .unwrap_or(false);
            if !has_first_proof {
                return Err("replace-proof-field: missing proof[0]".to_string());
            }
            entry["proof"][0][key] = val;
        }
        _ => return Err(format!("unknown mutation: {mutation}")),
    }

    Ok(entry)
}

async fn resign_entry(
    entry: &Value,
    entry_index: usize,
    log_entries: &[Value],
    signer: &Secret,
) -> Result<Value, String> {
    let current_version_id = entry
        .get("versionId")
        .and_then(Value::as_str)
        .ok_or("resign: missing versionId")?;
    let version_number = current_version_id
        .split('-')
        .next()
        .ok_or("resign: malformed versionId")?
        .parse::<u32>()
        .map_err(|e| format!("resign: parse version number: {e}"))?;

    let prev_version_id = if entry_index == 0 {
        entry
            .pointer("/parameters/scid")
            .and_then(Value::as_str)
            .ok_or("resign: missing parameters.scid for genesis")?
            .to_string()
    } else {
        log_entries
            .get(entry_index - 1)
            .and_then(|v| v.get("versionId"))
            .and_then(Value::as_str)
            .ok_or("resign: missing previous versionId")?
            .to_string()
    };

    let mut signing_doc = entry.clone();
    if let Some(obj) = signing_doc.as_object_mut() {
        obj.remove("proof");
    }
    signing_doc["versionId"] = json!(prev_version_id);

    let entry_hash = compute_entry_hash(&signing_doc)?;
    let new_version_id = format!("{version_number}-{entry_hash}");
    signing_doc["versionId"] = json!(new_version_id);

    let proof = DataIntegrityProof::sign(&signing_doc, signer, SignOptions::new())
        .await
        .map_err(|e| format!("resign proof: {e}"))?;

    let mut out = signing_doc;
    out["proof"] = json!([proof]);
    Ok(out)
}

fn parse_timestamp(ts: Option<&str>) -> Option<DateTime<FixedOffset>> {
    ts.and_then(|v| DateTime::parse_from_rfc3339(v).ok())
}

fn expected_error_result(error: &str) -> Value {
    json!({
        "didDocument": null,
        "didDocumentMetadata": {},
        "didResolutionMetadata": {"error": error},
    })
}

async fn run_scenario(scenario_dir: &Path) -> Result<(), String> {
    let script_text = std::fs::read_to_string(scenario_dir.join("script.yaml"))
        .map_err(|e| format!("read script.yaml: {e}"))?;
    let script: Script =
        serde_yaml::from_str(&script_text).map_err(|e| format!("parse script.yaml: {e}"))?;

    if !script.negative {
        return Ok(());
    }

    let out_dir = scenario_dir.join("rust");
    std::fs::create_dir_all(&out_dir).map_err(|e| format!("create rust/ dir: {e}"))?;

    let keys = build_key_registry(&script.keys)?;

    let mut state: DIDWebVHState;

    let mut log_entries: Vec<Value> = Vec::new();
    let mut witness_entries: Vec<Value> = Vec::new();
    let mut expect_error: Option<String> = None;
    let mut current_did: Option<String> = None;
    let mut current_update_key_ids: Vec<String> = Vec::new();
    let mut current_doc_state: Option<Value> = None;

    for step in &script.steps {
        match step.op.as_str() {
            "create" => {
                let domain = step.domain.as_deref().ok_or("create: missing domain")?;
                let signer_id = step.signer.as_deref().ok_or("create: missing signer")?;
                let signer = keys
                    .get(signer_id)
                    .ok_or_else(|| format!("unknown signer '{signer_id}'"))?;
                let params = step.params.clone().unwrap_or_default();

                current_update_key_ids = if params.update_keys.is_empty() {
                    vec![signer_id.to_string()]
                } else {
                    params.update_keys.clone()
                };

                log_entries.clear();
                witness_entries.clear();
                state = DIDWebVHState::default();

                let did_placeholder = format!("did:webvh:{SCID_HOLDER}:{domain}");
                let doc = build_document(&did_placeholder, &params, &keys);
                let parameter_set = build_parameters(&params, &keys);

                match parameter_set {
                    Ok(parameters) => {
                        let created = state
                            .create_log_entry(
                                parse_timestamp(step.timestamp.as_deref()),
                                &doc,
                                &parameters,
                                &signer.secret,
                            )
                            .await;
                        match created {
                            Ok(entry_state) => {
                                let value = serde_json::to_value(&entry_state.log_entry)
                                    .map_err(|e| format!("serialize create entry: {e}"))?;
                                current_did = value
                                    .pointer("/state/id")
                                    .and_then(Value::as_str)
                                    .map(ToString::to_string);
                                current_doc_state = value.get("state").cloned();
                                log_entries.push(value);
                            }
                            Err(_) => {
                                let ts = step
                                    .timestamp
                                    .as_deref()
                                    .ok_or("create fallback requires timestamp")?;
                                let bypass = create_genesis_bypass(
                                    domain,
                                    ts,
                                    &params,
                                    &signer.secret,
                                    &keys,
                                )
                                .await?;
                                current_did = bypass
                                    .pointer("/state/id")
                                    .and_then(Value::as_str)
                                    .map(ToString::to_string);
                                current_doc_state = bypass.get("state").cloned();
                                log_entries.push(bypass);
                            }
                        }
                    }
                    Err(_) => {
                        let ts = step
                            .timestamp
                            .as_deref()
                            .ok_or("create fallback requires timestamp")?;
                        let bypass =
                            create_genesis_bypass(domain, ts, &params, &signer.secret, &keys).await?;
                        current_did = bypass
                            .pointer("/state/id")
                            .and_then(Value::as_str)
                            .map(ToString::to_string);
                        current_doc_state = bypass.get("state").cloned();
                        log_entries.push(bypass);
                    }
                }
            }
            "update" => {
                let signer_id = step.signer.as_deref().ok_or("update: missing signer")?;
                let signer = keys
                    .get(signer_id)
                    .ok_or_else(|| format!("unknown signer '{signer_id}'"))?;
                let params = step.params.clone().unwrap_or_default();

                if !params.update_keys.is_empty() {
                    current_update_key_ids = params.update_keys.clone();
                }

                let did = current_did
                    .as_deref()
                    .ok_or("update before create: missing current DID")?;
                let doc = if let Some(base) = &current_doc_state {
                    merge_doc_with_params(base, did, &params, &keys)
                } else {
                    build_document(did, &params, &keys)
                };
                let ts = step
                    .timestamp
                    .as_deref()
                    .ok_or("update bypass: missing timestamp")?;
                let previous = log_entries.last().ok_or("update bypass: missing previous entry")?;
                let value = create_update_bypass(
                    previous,
                    ts,
                    &params,
                    &current_update_key_ids,
                    doc,
                    &signer.secret,
                    &keys,
                )
                .await?;

                current_did = value
                    .pointer("/state/id")
                    .and_then(Value::as_str)
                    .map(ToString::to_string)
                    .or(current_did);
                current_doc_state = value.get("state").cloned();
                log_entries.push(value);
            }
            "migrate" => {
                let signer_id = step.signer.as_deref().ok_or("migrate: missing signer")?;
                let signer = keys
                    .get(signer_id)
                    .ok_or_else(|| format!("unknown signer '{signer_id}'"))?;
                let params = step.params.clone().unwrap_or_default();
                let old_did = current_did
                    .as_deref()
                    .ok_or("migrate before create: missing current DID")?;
                // Intentionally keep the existing domain to match TS negative-vector
                // artifact semantics for the portable SCID swap scenario.
                let new_did = old_did.to_string();

                if !params.update_keys.is_empty() {
                    current_update_key_ids = params.update_keys.clone();
                }

                let doc = if let Some(base) = &current_doc_state {
                    merge_doc_with_params(base, &new_did, &params, &keys)
                } else {
                    build_document(&new_did, &params, &keys)
                };
                let ts = step
                    .timestamp
                    .as_deref()
                    .ok_or("migrate bypass: missing timestamp")?;
                let previous = log_entries.last().ok_or("migrate bypass: missing previous entry")?;
                let value = create_update_bypass(
                    previous,
                    ts,
                    &params,
                    &current_update_key_ids,
                    doc,
                    &signer.secret,
                    &keys,
                )
                .await?;

                current_did = Some(new_did);
                current_doc_state = value.get("state").cloned();
                log_entries.push(value);
            }
            "corrupt" => {
                let entry_idx = step.entry.ok_or("corrupt: missing entry")?;
                if entry_idx >= log_entries.len() {
                    return Err(format!(
                        "corrupt: entry {} out of range [0, {}]",
                        entry_idx,
                        log_entries.len().saturating_sub(1)
                    ));
                }

                let mutation = step
                    .mutation
                    .as_deref()
                    .ok_or("corrupt: missing mutation")?;
                let when = step.when.as_deref().ok_or("corrupt: missing when")?;

                let mutated = apply_mutation(
                    log_entries
                        .get(entry_idx)
                        .ok_or("corrupt: missing target entry")?
                        .clone(),
                    mutation,
                    step.field.as_deref(),
                    step.value.as_ref(),
                    &keys,
                )?;

                if when == "before-sign" {
                    let signer_id = current_update_key_ids
                        .first()
                        .cloned()
                        .or_else(|| keys.keys().next().cloned())
                        .ok_or("corrupt before-sign: no keys available")?;
                    let signer = keys
                        .get(&signer_id)
                        .ok_or_else(|| format!("corrupt signer key not found: {signer_id}"))?;
                    let resigned = resign_entry(&mutated, entry_idx, &log_entries, &signer.secret).await?;
                    log_entries[entry_idx] = resigned;
                } else {
                    log_entries[entry_idx] = mutated;
                }
            }
            "sign-witness-proof" => {
                let signer_id = step
                    .signer
                    .as_deref()
                    .ok_or("sign-witness-proof: missing signer")?;
                let signer = keys
                    .get(signer_id)
                    .ok_or_else(|| format!("unknown witness signer '{signer_id}'"))?;
                let entry_idx = step.entry.ok_or("sign-witness-proof: missing entry")?;
                let version_id = log_entries
                    .get(entry_idx)
                    .and_then(|v| v.get("versionId"))
                    .and_then(Value::as_str)
                    .ok_or("sign-witness-proof: invalid entry/versionId")?
                    .to_string();

                let proof = DataIntegrityProof::sign(
                    &json!({"versionId": version_id}),
                    &signer.secret,
                    SignOptions::new(),
                )
                .await
                .map_err(|e| format!("sign witness proof: {e}"))?;

                witness_entries.push(json!({
                    "versionId": version_id,
                    "proof": [proof],
                }));
            }
            "resolve" | "resolve-did" => {
                expect_error = step.expect_error.clone();
                let _ = &step.did;
            }
            _ => {
                // Ignore unknown ops to avoid breaking older scripts.
            }
        }
    }

    let did_jsonl = if log_entries.is_empty() {
        String::new()
    } else {
        let mut lines = Vec::with_capacity(log_entries.len());
        for e in &log_entries {
            lines.push(serde_json::to_string(e).map_err(|err| format!("serialize did.jsonl line: {err}"))?);
        }
        format!("{}\n", lines.join("\n"))
    };
    std::fs::write(out_dir.join("did.jsonl"), did_jsonl)
        .map_err(|e| format!("write did.jsonl: {e}"))?;

    if !witness_entries.is_empty() {
        let s = serde_json::to_string_pretty(&witness_entries)
            .map_err(|e| format!("serialize did-witness.json: {e}"))?;
        std::fs::write(out_dir.join("did-witness.json"), format!("{s}\n"))
            .map_err(|e| format!("write did-witness.json: {e}"))?;
    }

    if let Some(error) = expect_error {
        let s = serde_json::to_string_pretty(&expected_error_result(&error))
            .map_err(|e| format!("serialize resolutionResult.json: {e}"))?;
        std::fs::write(out_dir.join("resolutionResult.json"), format!("{s}\n"))
            .map_err(|e| format!("write resolutionResult.json: {e}"))?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let vectors_root = Path::new(VECTORS_ROOT);

    let mut scenarios: Vec<_> = match std::fs::read_dir(vectors_root) {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .filter(|e| e.file_name().to_string_lossy().starts_with("negative-"))
            .collect(),
        Err(e) => {
            eprintln!("error: cannot read vectors/: {e}");
            std::process::exit(2);
        }
    };
    scenarios.sort_by_key(|e| e.file_name());

    let mut ok = 0u32;
    let mut err = 0u32;

    for scenario in &scenarios {
        let name = scenario.file_name().to_string_lossy().to_string();
        if !scenario.path().join("script.yaml").exists() {
            continue;
        }

        print!("generate {name} ... ");
        match run_scenario(&scenario.path()).await {
            Ok(()) => {
                println!("ok");
                ok += 1;
            }
            Err(e) => {
                println!("ERROR: {e}");
                err += 1;
            }
        }
    }

    println!("\n{ok} generated, {err} errors");
    if err > 0 {
        std::process::exit(1);
    }
}
