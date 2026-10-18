//! Compliance test harness — resolution tests
//!
//! Runs each committed vector (from every implementation subdir) through the
//! Rust didwebvh-rs resolver and compares results against the committed
//! resolutionResult*.json files.  Also writes a status.md to the rust/ subdir
//! of each scenario showing cross-resolution results.
//!
//! Normalization functions labelled "TS COMPAT" compensate for known differences
//! between what the TypeScript generator writes into vectors/ and what the Rust
//! library produces.  Each is a candidate for community discussion.
//!
//! Known hard incompatibilities are reported as XFAIL rather than FAIL.
//!
//! Usage:
//!   cargo run --bin test-vectors
//!
//! Exit code 0 = all pass or xfail; 1 = at least one failure; 2 = setup error.

use std::path::Path;

use didwebvh_rs::log_entry::LogEntryMethods;
use didwebvh_rs::host_policy::HostPolicy;
use didwebvh_rs::prelude::DIDWebVHState;
use didwebvh_rs::url::WebVHURL;
use serde_json::{json, Value};
use serde_jcs;
use similar::{ChangeTag, TextDiff};

const VECTORS_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../vectors");
const IMPL_DIR: &str = env!("CARGO_MANIFEST_DIR");
const IMPL_NAME: &str = "rust";
const CONFIG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/config.yaml");

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    let vectors_root = Path::new(VECTORS_ROOT);
    let impl_dir = Path::new(IMPL_DIR);

    // Delete output files at the start of each run
    let status_path = impl_dir.join("status.md");
    let diffs_path = impl_dir.join("diffs.txt");
    let _ = std::fs::remove_file(&status_path);
    let _ = std::fs::remove_file(&diffs_path);

    let mut scenarios: Vec<_> = match std::fs::read_dir(vectors_root) {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .collect(),
        Err(e) => {
            eprintln!("error: cannot read vectors/: {e}");
            std::process::exit(2);
        }
    };
    scenarios.sort_by_key(|e| e.file_name());

    let mut pass = 0u32;
    let mut xfail = 0u32;
    let mut fail = 0u32;
    let mut diff_count = 0u32;

    // (test_case, log_source, result, notes)
    let mut all_rows: Vec<(String, String, String, String)> = Vec::new();
    // (test_case, log_source, filename, diff_body)
    let mut all_diffs: Vec<(String, String, String, String)> = Vec::new();
    // (test_case, expected_error, result, notes)
    let mut neg_rows: Vec<(String, String, String, String)> = Vec::new();
    // URL assertions are evaluated without fetching a log or making a request.
    let mut url_rows: Vec<(String, String, String, String)> = Vec::new();
    let mut probe_rows: Vec<(String, String, String, String)> = Vec::new();
    let mut probe_pass = 0u32;
    let mut probe_diff = 0u32;
    let mut probe_fail = 0u32;
    let mut probe_skip = 0u32;
    let mut url_skip = 0u32;
    let mut scripted_scenarios = std::collections::HashSet::new();
    let mut probe_scenarios = std::collections::HashSet::new();
    let mut log_scenarios = std::collections::HashSet::new();

    for scenario in &scenarios {
        let scenario_name = scenario.file_name().to_string_lossy().to_string();
        let script_path = scenario.path().join("script.yaml");
        let Ok(script_text) = std::fs::read_to_string(&script_path) else { continue };
        let script: serde_yaml::Value = match serde_yaml::from_str(&script_text) {
            Ok(script) => script,
            Err(e) => {
                eprintln!("FAIL   {scenario_name}/script.yaml (parse error: {e})");
                fail += 1;
                url_rows.push((scenario_name, "❌ FAIL".to_string(), "script.yaml".to_string(), format!("parse error: {e}")));
                continue;
            }
        };
        let Some(steps) = script["steps"].as_sequence() else { continue };
        if steps.iter().any(|step| step["op"].as_str() == Some("create")) {
            log_scenarios.insert(scenario_name.clone());
        }
        if script["probe"].as_bool() == Some(true) {
            probe_scenarios.insert(scenario_name.clone());
        }
        let has_log = scenario.path().join("ts/did.jsonl")
            .metadata().map(|m| m.len() > 0).unwrap_or(false);
        for (index, step) in steps.iter().enumerate() {
            if step["op"].as_str() != Some("resolve-did")
                || (step["expectFetchUrl"].is_null() && step["expectError"].is_null())
                || (has_log && step["expectFetchUrl"].is_null())
            {
                continue;
            }
            scripted_scenarios.insert(scenario_name.clone());
            let id = format!("{scenario_name}/resolve-did/{}", index + 1);
            let is_probe = step["probe"].as_bool().unwrap_or(false)
                || script["probe"].as_bool().unwrap_or(false);
            let expected = step["expectFetchUrl"].as_str()
                .or_else(|| step["expectError"].as_str())
                .unwrap_or("?");
            let (result, notes) = check_url_step(step);
            let result = if is_probe && result == UrlOutcome::Fail {
                UrlOutcome::Diff
            } else {
                result
            };
            let label = match result {
                UrlOutcome::Pass => "✅ PASS",
                UrlOutcome::Diff => "🔶 DIFF",
                UrlOutcome::Fail => "❌ FAIL",
                UrlOutcome::Skip => "⚠️ SKIP",
            };
            if result == UrlOutcome::Pass || result == UrlOutcome::Skip {
                println!("{}   {id} ({notes})", result.as_str());
            } else {
                eprintln!("{}   {id} ({notes})", result.as_str());
            }
            let row = (id, label.to_string(), expected.to_string(), notes);
            if result == UrlOutcome::Diff {
                all_diffs.push((
                    scenario_name.clone(),
                    format!("step {}", index + 1),
                    "expectFetchUrl".to_string(),
                    format!("expected: {expected}\nactual: {}", row.3),
                ));
            }
            if is_probe {
                probe_rows.push(row);
                match result {
                    UrlOutcome::Pass => probe_pass += 1,
                    UrlOutcome::Diff => probe_diff += 1,
                    UrlOutcome::Fail => probe_fail += 1,
                    UrlOutcome::Skip => probe_skip += 1,
                }
            } else {
                url_rows.push(row);
                match result {
                    UrlOutcome::Pass => pass += 1,
                    UrlOutcome::Diff => diff_count += 1,
                    UrlOutcome::Fail => fail += 1,
                    UrlOutcome::Skip => url_skip += 1,
                }
            }
        }
    }

    // --- Negative resolution tests ---
    let mut neg_scenarios: Vec<_> = scenarios
        .iter()
        .filter(|e| e.file_name().to_string_lossy().starts_with("negative-"))
        .collect();
    neg_scenarios.sort_by_key(|e| e.file_name());

    for scenario in &neg_scenarios {
        let scenario_dir = scenario.path();
        let scenario_name = scenario.file_name().to_string_lossy().to_string();
        if probe_scenarios.contains(&scenario_name) {
            continue;
        }
        let ts_dir = scenario_dir.join("ts");

        // Read expected error from ts/resolutionResult.json
        let expected_error = ts_dir.join("resolutionResult.json")
            .exists()
            .then(|| std::fs::read_to_string(ts_dir.join("resolutionResult.json")).ok())
            .flatten()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
            .and_then(|v| v.pointer("/didResolutionMetadata/error").and_then(|e| e.as_str()).map(|s| s.to_string()))
            .unwrap_or_else(|| "?".to_string());

        let jsonl_path = ts_dir.join("did.jsonl");
        if !jsonl_path.exists() {
            if scripted_scenarios.contains(&scenario_name) {
                continue;
            }
            neg_rows.push((scenario_name, expected_error, "⚠️ SKIP".to_string(), "ts/ not generated".to_string()));
            continue;
        }

        let log_content = match std::fs::read_to_string(&jsonl_path) {
            Ok(c) => c,
            Err(e) => {
                neg_rows.push((scenario_name, expected_error, "⚠️ SKIP".to_string(), format!("read error: {e}")));
                continue;
            }
        };

        if log_content.trim().is_empty() {
            if scripted_scenarios.contains(&scenario_name) {
                continue;
            }
            // URL-only test: parse the DID URLs from script.yaml and check that
            // the URL parser rejects each one before any network call is needed.
            let script_path = scenario_dir.join("script.yaml");
            let script_str = match std::fs::read_to_string(&script_path) {
                Ok(s) => s,
                Err(_) => {
                    neg_rows.push((scenario_name, expected_error, "⚠️ SKIP".to_string(), "cannot read script.yaml".to_string()));
                    continue;
                }
            };
            let script: serde_yaml::Value = match serde_yaml::from_str(&script_str) {
                Ok(v) => v,
                Err(_) => {
                    neg_rows.push((scenario_name, expected_error, "⚠️ SKIP".to_string(), "cannot parse script.yaml".to_string()));
                    continue;
                }
            };
            let dids: Vec<String> = script["steps"]
                .as_sequence()
                .unwrap_or(&vec![])
                .iter()
                .filter(|s| s["op"].as_str() == Some("resolve-did"))
                .filter_map(|s| s["did"].as_str().map(|d| d.to_string()))
                .collect();

            if dids.is_empty() {
                neg_rows.push((scenario_name, expected_error, "⚠️ SKIP".to_string(), "no resolve-did ops in script".to_string()));
                continue;
            }

            let mut fail_reason: Option<String> = None;
            for did in &dids {
                if WebVHURL::parse_did_url(did).is_ok() {
                    fail_reason = Some(format!("URL parser accepted invalid DID: {did}"));
                    break;
                }
            }
            match fail_reason {
                None => {
                    pass += 1;
                    neg_rows.push((scenario_name, expected_error, "✅ PASS".to_string(), String::new()));
                }
                Some(reason) => {
                    fail += 1;
                    neg_rows.push((scenario_name, expected_error, "❌ FAIL".to_string(), reason));
                }
            }
            continue;
        }

        let witness_content = {
            let wp = ts_dir.join("did-witness.json");
            if wp.exists() { std::fs::read_to_string(&wp).ok() } else { None }
        };

        let first_entry: serde_json::Value = match log_content.lines().next()
            .and_then(|l| serde_json::from_str(l).ok())
        {
            Some(v) => v,
            None => {
                neg_rows.push((scenario_name, expected_error, "⚠️ SKIP".to_string(), "cannot parse log".to_string()));
                continue;
            }
        };
        let base_did = match first_entry.pointer("/state/id").and_then(|v| v.as_str()) {
            Some(s) => s.to_string(),
            None => {
                neg_rows.push((scenario_name, expected_error, "⚠️ SKIP".to_string(), "no /state/id".to_string()));
                continue;
            }
        };

        let mut state = DIDWebVHState::default();
        match state.resolve_log_owned(&base_did, &log_content, witness_content.as_deref()).await {
            Err(_) => {
                // Resolver correctly rejected the invalid log.
                pass += 1;
                neg_rows.push((scenario_name, expected_error, "✅ PASS".to_string(), String::new()));
            }
            Ok(_) => {
                // Resolver accepted a log it should have rejected.
                fail += 1;
                neg_rows.push((scenario_name, expected_error, "❌ FAIL".to_string(),
                    "resolver accepted invalid log".to_string()));
            }
        }
    }

    for scenario in &scenarios {
        let scenario_dir = scenario.path();
        let scenario_name = scenario.file_name().to_string_lossy().to_string();

        if !scenario_dir.join("script.yaml").exists() {
            continue;
        }

        // Negative test vectors are handled separately — skip them here.
        if scenario_name.starts_with("negative-") || !log_scenarios.contains(&scenario_name) {
            continue;
        }

        let mut impl_dirs: Vec<_> = match std::fs::read_dir(&scenario_dir) {
            Ok(rd) => rd
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
                .collect(),
            Err(_) => continue,
        };
        impl_dirs.sort_by_key(|e| e.file_name());

        for impl_entry in &impl_dirs {
            let impl_path = impl_entry.path();
            let impl_name = impl_entry.file_name().to_string_lossy().to_string();

            if !impl_path.join("did.jsonl").exists() {
                all_rows.push((scenario_name.clone(), impl_name, "⚠️ SKIP".to_string(), "no did.jsonl".to_string()));
                continue;
            }

            let mut result_files: Vec<_> = std::fs::read_dir(&impl_path)
                .unwrap()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let n = e.file_name().to_string_lossy().to_string();
                    n.starts_with("resolutionResult") && n.ends_with(".json")
                })
                .collect();
            result_files.sort_by_key(|e| e.file_name());

            if result_files.is_empty() {
                all_rows.push((scenario_name.clone(), impl_name, "⚠️ SKIP".to_string(), "no resolutionResult files".to_string()));
                continue;
            }

            let mut impl_outcome = "pass";
            let mut impl_reason = String::new();

            for rf in &result_files {
                let result_file = rf.file_name().to_string_lossy().to_string();
                let test_id = format!("{scenario_name}/{impl_name}/{result_file}");
                match run_vector_test(&impl_path, &result_file).await {
                    TestOutcome::Pass => {
                        println!("PASS   {test_id}");
                        pass += 1;
                    }
                    TestOutcome::XFail(reason) => {
                        println!("XFAIL  {test_id} ({reason})");
                        xfail += 1;
                        if impl_outcome == "pass" { impl_outcome = "xfail"; impl_reason = reason; }
                    }
                    TestOutcome::Fail(reason) => {
                        eprintln!("FAIL   {test_id} ({reason})");
                        fail += 1;
                        if impl_outcome != "diff" { impl_outcome = "fail"; }
                        if impl_reason.is_empty() { impl_reason = reason; }
                    }
                    TestOutcome::Diff(msg) => {
                        eprintln!("DIFF   {test_id}");
                        for line in msg.lines().take(5) { eprintln!("       {line}"); }
                        diff_count += 1;
                        if impl_outcome == "pass" || impl_outcome == "xfail" { impl_outcome = "diff"; }
                        all_diffs.push((scenario_name.clone(), impl_name.clone(), result_file, msg));
                    }
                }
            }

            let label = if impl_name == IMPL_NAME {
                format!("{impl_name} (self)")
            } else {
                impl_name.clone()
            };

            let (result, notes) = match impl_outcome {
                "xfail" => ("⚠️ XFAIL".to_string(), impl_reason.chars().take(120).collect()),
                "fail"  => ("❌ FAIL".to_string(),  impl_reason.chars().take(120).collect()),
                "diff"  => ("🔶 DIFF".to_string(),  "see diffs.txt".to_string()),
                _       => ("✅ PASS".to_string(),  String::new()),
            };
            all_rows.push((scenario_name.clone(), label, result, notes));
        }
    }

    println!("\n{pass} passed, {diff_count} diff, {fail} failed, {xfail} xfailed, {url_skip} URL skipped");
    println!("URL probes: {probe_pass} passed, {probe_diff} diff, {probe_fail} failed, {probe_skip} skipped (excluded from conformance exit status)");

    // Write implementations/rust/status.md
    if !all_rows.is_empty() || !url_rows.is_empty() || !probe_rows.is_empty() {
        let version = read_config_version();
        let header = if version.is_empty() { String::new() } else {
            format!("Implementation: didwebvh-rs {version}\n\n")
        };

        let gen_results_path = impl_dir.join("gen_results.json");
        let gen_table = match std::fs::read_to_string(&gen_results_path)
            .ok()
            .and_then(|s| serde_json::from_str::<Vec<Value>>(&s).ok())
            .filter(|rows| !rows.is_empty())
        {
            Some(rows) => {
                let rows_text = rows.iter()
                    .map(|r| format!("| {} | {} | {} |",
                        r["testCase"].as_str().unwrap_or(""),
                        r["result"].as_str().unwrap_or(""),
                        r["notes"].as_str().unwrap_or(""),
                    ))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("## DID Creation\n\n| Test Case | Result | Notes |\n|---|---|---|\n{rows_text}\n\n")
            }
            None => String::new(),
        };

        let neg_table = if !neg_rows.is_empty() {
            let rows = neg_rows.iter()
                .map(|(tc, ee, r, n)| format!("| {tc} | {ee} | {r} | {n} |"))
                .collect::<Vec<_>>()
                .join("\n");
            format!("## Negative Resolution\n\n| Test Case | Expected Error | Result | Notes |\n|---|---|---|---|\n{rows}\n\n")
        } else {
            String::new()
        };

        let url_table = format_url_rows("Pre-network URL assertions", &url_rows);
        let probe_table = format_url_rows("URL probes (non-conformance)", &probe_rows);
        let table_rows: String = all_rows.iter()
            .map(|(tc, ls, r, n)| format!("| {tc} | {ls} | {r} | {n} |"))
            .collect::<Vec<_>>()
            .join("\n");
        let content = format!(
            "# rust status\n\n{header}{gen_table}{neg_table}{url_table}{probe_table}## Cross-Resolution\n\n| Test Case | Log Source | Result | Notes |\n|---|---|---|---|\n{table_rows}\n"
        );
        if let Err(e) = std::fs::write(&status_path, &content) {
            eprintln!("warning: could not write status.md: {e}");
        }
    }

    // Write implementations/rust/diffs.txt
    if !all_diffs.is_empty() {
        let content: String = all_diffs.iter()
            .map(|(tc, ls, f, d)| format!("=== {tc} / {ls} — {f} ===\n{d}"))
            .collect::<Vec<_>>()
            .join("\n\n") + "\n";
        if let Err(e) = std::fs::write(&diffs_path, &content) {
            eprintln!("warning: could not write diffs.txt: {e}");
        }
    }

    if fail > 0 || diff_count > 0 {
        std::process::exit(1);
    }
}

fn read_config_version() -> String {
    let cfg_path = Path::new(CONFIG_PATH);
    let Ok(text) = std::fs::read_to_string(cfg_path) else { return String::new(); };
    let version = text.lines()
        .find(|l| l.starts_with("version:"))
        .and_then(|l| l.split(':').nth(1))
        .map(|s| s.trim().trim_matches('"').to_string())
        .unwrap_or_default();
    let commit = text.lines()
        .find(|l| l.starts_with("commit:"))
        .and_then(|l| l.split(':').nth(1))
        .map(|s| s.trim().trim_matches('"').to_string())
        .unwrap_or_default();
    [version, commit].into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" @ ")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UrlOutcome {
    Pass,
    Diff,
    Fail,
    Skip,
}

impl UrlOutcome {
    fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Diff => "DIFF",
            Self::Fail => "FAIL",
            Self::Skip => "SKIP",
        }
    }
}

fn check_url_step(step: &serde_yaml::Value) -> (UrlOutcome, String) {
    let Some(did) = step["did"].as_str() else {
        return (UrlOutcome::Fail, "resolve-did step has no did".to_string());
    };
    let expected_url = step["expectFetchUrl"].as_str();
    let expected_error = step["expectError"].as_str();
    if expected_url.is_some() && expected_error.is_some() {
        return (UrlOutcome::Fail, "step specifies both expectFetchUrl and expectError".to_string());
    }
    if expected_url.is_none() && expected_error.is_none() {
        return (UrlOutcome::Fail, "expected URL or error must be a string".to_string());
    }

    // Use the resolver's pre-request egress check, without making a request.
    let actual = WebVHURL::parse_did_url(did)
        .and_then(|url| url.get_fetch_url("did.jsonl", HostPolicy::PublicOnly));
    match (expected_url, expected_error, actual) {
        (Some(expected), None, Ok(url)) if url.as_str() == expected => {
            (UrlOutcome::Pass, format!("fetch URL: {url}"))
        }
        (Some(expected), None, Ok(url)) => {
            (UrlOutcome::Diff, format!("expected {expected}; got {url}"))
        }
        (Some(_), None, Err(e)) => (UrlOutcome::Fail, format!("URL construction failed: {e}")),
        (None, Some("invalidDid"), Err(e)) => {
            (UrlOutcome::Pass, format!("rejected before network: {e}"))
        }
        (None, Some("invalidDid"), Ok(url)) => {
            (UrlOutcome::Fail, format!("expected invalidDid; constructed {url}"))
        }
        (None, Some(error), _) => {
            (UrlOutcome::Skip, format!("{error} requires resolution beyond URL construction"))
        }
        _ => unreachable!(),
    }
}

fn format_url_rows(title: &str, rows: &[(String, String, String, String)]) -> String {
    if rows.is_empty() {
        return String::new();
    }
    let rows = rows.iter()
        .map(|(id, result, expected, notes)| format!(
            "| {} | {} | {} | {} |",
            id.replace('|', "\\|"),
            expected.replace('|', "\\|"),
            result,
            notes.replace('|', "\\|").replace('\n', " "),
        ))
        .collect::<Vec<_>>()
        .join("\n");
    format!("## {title}\n\n| Test Case | Expected | Result | Notes |\n|---|---|---|---|\n{rows}\n\n")
}

#[cfg(test)]
mod url_step_tests {
    use super::*;

    fn check(yaml: &str) -> UrlOutcome {
        let step = serde_yaml::from_str(yaml).unwrap();
        check_url_step(&step).0
    }

    #[test]
    fn compares_the_resolvers_pre_network_url() {
        let did = "did:webvh:QmUy89VrfryQ254CeHZzQfmcKqByPoKNGqYykP3SeXuegQ:x.example.com";
        assert_eq!(check(&format!(
            "did: {did}\nexpectFetchUrl: https://x.example.com/.well-known/did.jsonl"
        )), UrlOutcome::Pass);
        assert_eq!(check(&format!(
            "did: {did}\nexpectFetchUrl: https://different.example.com/did.jsonl"
        )), UrlOutcome::Diff);
    }

    #[test]
    fn checks_errors_without_fetching() {
        let did = "did:webvh:QmUy89VrfryQ254CeHZzQfmcKqByPoKNGqYykP3SeXuegQ:127.0.0.1";
        assert_eq!(check(&format!("did: {did}\nexpectError: invalidDid")), UrlOutcome::Pass);
        assert_eq!(check(&format!("did: {did}\nexpectFetchUrl: https://example.com/did.jsonl")), UrlOutcome::Fail);
        assert_eq!(check("did: did:webvh:QmS:example.com\nexpectError: invalidProof"), UrlOutcome::Skip);
    }
}

// ---------------------------------------------------------------------------
// Test outcome
// ---------------------------------------------------------------------------

enum TestOutcome {
    Pass,
    XFail(String), // known incompatibility — not a spec violation
    Fail(String),  // resolver errored — no result produced
    Diff(String),  // resolver ran but output differs from expected
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// "resolutionResult.json" → None, "resolutionResult.2.json" → Some(2)
fn extract_version_number(result_file: &str) -> Option<u32> {
    let stem = result_file.trim_end_matches(".json");
    stem.split('.').nth(1).and_then(|s| s.parse().ok())
}

// ---------------------------------------------------------------------------
// Core test logic
// ---------------------------------------------------------------------------

async fn run_vector_test(impl_dir: &Path, result_file: &str) -> TestOutcome {
    let log_content = match std::fs::read_to_string(impl_dir.join("did.jsonl")) {
        Ok(c) => c,
        Err(e) => return TestOutcome::Fail(format!("read did.jsonl: {e}")),
    };


    let mut expected: Value =
        match std::fs::read_to_string(impl_dir.join(result_file)).map_err(|e| e.to_string()).and_then(|s| serde_json::from_str(&s).map_err(|e: serde_json::Error| e.to_string())) {
            Ok(v) => v,
            Err(e) => return TestOutcome::Fail(format!("read {result_file}: {e}")),
        };

    let first_line = match log_content.lines().next() {
        Some(l) => l,
        None => return TestOutcome::Fail("empty did.jsonl".to_string()),
    };
    let first_entry: Value = match serde_json::from_str(first_line) {
        Ok(v) => v,
        Err(e) => return TestOutcome::Fail(format!("parse first log entry: {e}")),
    };
    let base_did = match first_entry.pointer("/state/id").and_then(|v| v.as_str()) {
        Some(s) => s.to_string(),
        None => return TestOutcome::Fail("no /state/id in first log entry".to_string()),
    };

    let witness_content = {
        let wp = impl_dir.join("did-witness.json");
        if wp.exists() {
            match std::fs::read_to_string(&wp) {
                Ok(c) => Some(c),
                Err(e) => return TestOutcome::Fail(format!("read did-witness.json: {e}")),
            }
        } else {
            None
        }
    };

    let did = match extract_version_number(result_file) {
        Some(n) => format!("{base_did}?versionNumber={n}"),
        None => base_did.clone(),
    };

    let mut state = DIDWebVHState::default();
    let (log_entry, meta) = match state
        .resolve_log_owned(&did, &log_content, witness_content.as_deref())
        .await
    {
        Ok(r) => r,
        Err(e) => {
            let msg = format!("{e:?}");
            return TestOutcome::Fail(format!("resolve_log: {msg}"));
        }
    };

    let did_document = match log_entry.get_did_document() {
        Ok(d) => d,
        Err(e) => return TestOutcome::Fail(format!("get_did_document: {e}")),
    };

    let mut did_document_metadata = match serde_json::to_value(&meta) {
        Ok(v) => v,
        Err(e) => return TestOutcome::Fail(format!("serialize MetaData: {e}")),
    };

    if let Some(meta_obj) = did_document_metadata.as_object_mut() {
        if let Some(version_id) = meta_obj.get("versionId").and_then(|v| v.as_str()) {
            if let Some(n) = version_id.split('-').next().and_then(|s| s.parse::<u64>().ok()) {
                meta_obj.insert("versionNumber".to_string(), json!(n));
            }
        }
    }

    let mut actual = json!({
        "didDocument": did_document,
        "didDocumentMetadata": did_document_metadata,
        "didResolutionMetadata": {"contentType": "application/did+ld+json"},
    });

    let actual_jcs = match serde_jcs::to_vec(&actual) {
        Ok(b) => b,
        Err(e) => return TestOutcome::Fail(format!("JCS serialize actual: {e}")),
    };
    let expected_jcs = match serde_jcs::to_vec(&expected) {
        Ok(b) => b,
        Err(e) => return TestOutcome::Fail(format!("JCS serialize expected: {e}")),
    };
    if actual_jcs == expected_jcs {
        return TestOutcome::Pass;
    }

    let act_str = serde_json::to_string_pretty(&actual).unwrap_or_default();
    let exp_str = serde_json::to_string_pretty(&expected).unwrap_or_default();
    let diff = TextDiff::from_lines(&exp_str, &act_str);
    let mut diff_out = String::from("--- expected\n+++ actual (rust resolver)\n");
    let groups = diff.grouped_ops(3);
    for (i, group) in groups.iter().enumerate() {
        for op in group {
            for change in diff.iter_changes(op) {
                let prefix = match change.tag() {
                    ChangeTag::Delete => "-",
                    ChangeTag::Insert => "+",
                    ChangeTag::Equal  => " ",
                };
                diff_out.push_str(&format!("{prefix}{}", change.value()));
            }
        }
        if i + 1 < groups.len() {
            diff_out.push_str("...\n");
        }
    }
    TestOutcome::Diff(diff_out)
}
