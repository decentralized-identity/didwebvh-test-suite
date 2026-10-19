"""
pytest plugin: collect cross-resolution results and write
implementations/python/status.md + implementations/python/diffs.txt
at the end of the test session.
"""
from __future__ import annotations

import json
from difflib import unified_diff
from pathlib import Path

import pytest

IMPL_DIR = Path(__file__).parent
STATUS_PATH = IMPL_DIR / "status.md"
DIFFS_PATH = IMPL_DIR / "diffs.txt"

_rows: list[dict] = []
_diffs: list[dict] = []
_neg_rows: list[dict] = []
_url_rows: list[dict] = []
_probe_rows: list[dict] = []
_url_diffs: list[dict] = []

_URL_LABELS = {"pass": "✅ PASS", "diff": "🔶 DIFF", "fail": "❌ FAIL", "skip": "⚠️ SKIP"}


# ---------------------------------------------------------------------------
# Lifecycle hooks
# ---------------------------------------------------------------------------

def pytest_sessionstart(session):
    _rows.clear()
    _diffs.clear()
    _neg_rows.clear()
    _url_rows.clear()
    _probe_rows.clear()
    _url_diffs.clear()
    if STATUS_PATH.exists():
        STATUS_PATH.unlink()
    if DIFFS_PATH.exists():
        DIFFS_PATH.unlink()


@pytest.hookimpl(hookwrapper=True)
def pytest_runtest_makereport(item, call):
    outcome = yield
    report = outcome.get_result()

    if report.when != "call" or not hasattr(item, "callspec"):
        return

    scenario = item.callspec.params.get("scenario")
    impl = item.callspec.params.get("impl")
    result_file = item.callspec.params.get("result_file")

    store = getattr(item, "_status_data", {})

    # Negative resolution tests use different status_data keys — handle before
    # the impl/result_file guard which would exit early for these tests.
    if "neg_scenario" in store:
        outcome = store.get("neg_outcome", "fail")
        neg_scenario = store["neg_scenario"]
        neg_expected = store.get("neg_expected_error", "?")
        if report.passed or outcome == "pass":
            _neg_rows.append({"testCase": neg_scenario, "expectedError": neg_expected,
                               "result": "✅ PASS", "notes": ""})
        elif report.skipped or outcome == "skip":
            _neg_rows.append({"testCase": neg_scenario, "expectedError": neg_expected,
                               "result": "⚠️ SKIP", "notes": "URL-only test (no log)"})
        else:
            _neg_rows.append({"testCase": neg_scenario, "expectedError": neg_expected,
                               "result": "❌ FAIL", "notes": "resolver accepted invalid log"})
        return

    # resolve-did step assertions (expectFetchUrl / expectError) and probes.
    if "url_case" in store or item.originalname == "test_resolve_did_step":
        case = store.get("url_case") or item.name
        outcome = store.get("url_outcome")
        notes = store.get("url_notes", "")
        if outcome is None:
            # The harness itself raised before recording an outcome.
            outcome, notes = "fail", _extract_reason(report)
        row = {"testCase": case, "expected": store.get("url_expected", "?"),
               "result": _URL_LABELS[outcome], "notes": notes}
        is_probe = store.get("url_probe", False)
        (_probe_rows if is_probe else _url_rows).append(row)
        if store.get("url_diff"):
            _url_diffs.append({"testCase": case, "probe": is_probe, "diff": store["url_diff"]})
        return

    if not (scenario and impl and result_file):
        return

    if report.passed:
        _rows.append({"testCase": scenario, "logSource": impl,
                      "result": "✅ PASS", "notes": ""})

    elif report.skipped and hasattr(report, "wasxfail"):
        notes = report.wasxfail.split("\n")[0][:120]
        _rows.append({"testCase": scenario, "logSource": impl,
                      "result": "⚠️ XFAIL", "notes": notes})

    elif report.failed:
        if store.get("actual") is not None and store.get("expected") is not None:
            diff = _compute_diff(store["expected"], store["actual"])
            if diff:
                _rows.append({"testCase": scenario, "logSource": impl,
                              "result": "🔶 DIFF", "notes": "see diffs.txt"})
                _diffs.append({"testCase": scenario, "logSource": impl,
                               "filename": result_file, "diff": diff})
            else:
                _rows.append({"testCase": scenario, "logSource": impl,
                              "result": "❌ FAIL", "notes": "assertion failed (no diff)"})
        else:
            _rows.append({"testCase": scenario, "logSource": impl,
                          "result": "❌ FAIL", "notes": _extract_reason(report)})


def pytest_sessionfinish(session, exitstatus):
    if not _rows and not _neg_rows and not _url_rows and not _probe_rows:
        return

    version = _read_version()
    header = f"Implementation: did-webvh python {version}\n\n" if version else ""

    gen_results_path = IMPL_DIR / "gen_results.json"
    gen_table = ""
    if gen_results_path.exists():
        gen_rows = json.loads(gen_results_path.read_text())
        if gen_rows:
            rows_text = "\n".join(
                f"| {r['testCase']} | {r['result']} | {r['notes']} |"
                for r in gen_rows
            )
            gen_table = (
                f"## DID Creation\n\n"
                f"| Test Case | Result | Notes |\n|---|---|---|\n{rows_text}\n\n"
            )

    neg_table = ""
    if _neg_rows:
        neg_rows_sorted = sorted(_neg_rows, key=lambda r: r["testCase"])
        neg_rows_text = "\n".join(
            f"| {r['testCase']} | {r['expectedError']} | {r['result']} | {r['notes']} |"
            for r in neg_rows_sorted
        )
        neg_table = (
            f"## Negative Resolution\n\n"
            f"| Test Case | Expected Error | Result | Notes |\n|---|---|---|---|\n{neg_rows_text}\n\n"
        )

    url_table = _format_url_rows("Pre-network URL assertions", _url_rows)
    probe_table = _format_url_rows("URL probes (non-conformance)", _probe_rows)

    table_rows = "\n".join(
        f"| {r['testCase']} | {r['logSource']} | {r['result']} | {r['notes']} |"
        for r in _rows
    )
    STATUS_PATH.write_text(
        f"# python status\n\n{header}{gen_table}{neg_table}{url_table}{probe_table}"
        f"## Cross-Resolution\n\n"
        f"| Test Case | Log Source | Result | Notes |\n|---|---|---|---|\n{table_rows}\n"
    )

    if _diffs or _url_diffs:
        diff_content = "\n\n".join([
            *(f"=== {d['testCase']} / {d['logSource']} — {d['filename']} ===\n{d['diff']}"
              for d in _diffs),
            *(f"=== {d['testCase']}{' (probe)' if d['probe'] else ''} ===\n{d['diff']}"
              for d in _url_diffs),
        ]) + "\n"
        DIFFS_PATH.write_text(diff_content)


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def _url_case_sort_key(row: dict) -> tuple[str, int]:
    prefix, _, step = row["testCase"].rpartition("/")
    return (prefix, int(step)) if step.isdigit() else (row["testCase"], 0)


def _format_url_rows(title: str, rows: list[dict]) -> str:
    if not rows:
        return ""

    def cell(value: str) -> str:
        return str(value).replace("|", "\\|").replace("\n", " ")

    body = "\n".join(
        f"| {cell(r['testCase'])} | {cell(r['expected'])} | {r['result']} | {cell(r['notes'])} |"
        for r in sorted(rows, key=_url_case_sort_key)
    )
    return (
        f"## {title}\n\n"
        f"| Test Case | Expected | Result | Notes |\n|---|---|---|---|\n{body}\n\n"
    )


def _compute_diff(expected: dict, actual: dict) -> str:
    exp_lines = json.dumps(expected, indent=2, ensure_ascii=False).splitlines(keepends=True)
    act_lines = json.dumps(actual, indent=2, ensure_ascii=False).splitlines(keepends=True)
    return "".join(unified_diff(
        exp_lines, act_lines,
        fromfile="expected", tofile="actual (python resolver)",
        n=3,
    )).rstrip()


def _extract_reason(report) -> str:
    if report.longrepr:
        lines = str(report.longrepr).splitlines()
        for line in reversed(lines):
            stripped = line.strip()
            if stripped and not stripped.startswith("E   assert"):
                return stripped[:120]
        for line in reversed(lines):
            stripped = line.strip()
            if stripped:
                return stripped[:120]
    return "unknown error"


def _read_version() -> str:
    try:
        import yaml
        cfg = yaml.safe_load((IMPL_DIR / "config.yaml").read_text())
        return str(cfg.get("version", "")) if cfg else ""
    except Exception:
        return ""
