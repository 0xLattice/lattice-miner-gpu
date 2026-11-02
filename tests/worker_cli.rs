use serde_json::Value;
use std::process::Command;
fn worker(target: &str, start: u64, count: u64) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_lattice-miner-gpu"))
        .args([
            "--backend",
            "cpu",
            "--work-id",
            "opaque-work",
            "--prefix-hex",
            "deadbeef",
            "--block-hex",
            "unused",
            "--target",
            target,
            "--start-nonce",
            &start.to_string(),
            "--count",
            &count.to_string(),
        ])
        .output()
        .unwrap()
}
#[test]
fn process_contract_reports_found_and_exhausted_without_extra_stdout() {
    let hit = worker(
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        12345,
        1,
    );
    assert!(hit.status.success());
    assert!(hit.stderr.is_empty());
    let value: Value = serde_json::from_slice(&hit.stdout).unwrap();
    assert_eq!(value["workId"], "opaque-work");
    assert_eq!(value["status"], "found");
    assert_eq!(value["nonce"], 12345);
    assert_eq!(value["rangeStart"], 12345);
    assert_eq!(value["rangeCount"], 1);
    let exhausted = worker("1", 17, 0);
    assert!(exhausted.status.success());
    let value: Value = serde_json::from_slice(&exhausted.stdout).unwrap();
    assert_eq!(value["status"], "exhausted");
    assert!(value["nonce"].is_null());
    assert!(value["hash"].is_null());
    for invalid in ["", "0x", "xyz"] {
        let result = worker(invalid, 0, 1);
        assert!(!result.status.success());
        assert!(result.stdout.is_empty());
    }
}
