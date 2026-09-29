use std::process::Command;

/// UC-006 | A3: Preview Request
#[test]
fn uc006_dry_run_prints_typed_json_without_config() {
	let output = Command::new(env!("CARGO_BIN_EXE_km"))
		.args(["member", "add", "--mail", "a@b.ch", "--prop-number", "score=1.5", "--dry-run"])
		.output()
		.expect("run km");
	assert!(output.status.success());
	let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("JSON output");
	assert_eq!(json["mail"], "a@b.ch");
	assert_eq!(json["properties"][0]["valueNumber"], 1.5);
	assert!(output.stderr.is_empty());
}

/// UC-006 | A4: Invalid Input
#[test]
fn uc006_invalid_number_reports_error() {
	let output = Command::new(env!("CARGO_BIN_EXE_km"))
		.args(["member", "add", "--mail", "a@b.ch", "--prop-number", "score=abc", "--dry-run"])
		.output()
		.expect("run km");
	assert!(!output.status.success());
	assert!(String::from_utf8_lossy(&output.stderr).contains("Invalid number for property 'score'"));
	assert!(output.stdout.is_empty());
}

/// UC-007 | A3: Nothing to Update
#[test]
fn uc007_empty_update_reports_error() {
	let output = Command::new(env!("CARGO_BIN_EXE_km"))
		.args(["member", "update", "--mail", "a@b.ch", "--dry-run"])
		.output()
		.expect("run km");
	assert!(!output.status.success());
	assert!(String::from_utf8_lossy(&output.stderr).contains("nothing to update"));
	assert!(output.stdout.is_empty());
}
