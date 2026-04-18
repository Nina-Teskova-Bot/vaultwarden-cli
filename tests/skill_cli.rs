mod support;

use predicates::prelude::*;
use support::TestContext;

#[test]
fn skill_list_outputs_supported_commands() {
    let ctx = TestContext::new();

    ctx.binary()
        .arg("skill")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("unlock"))
        .stdout(predicate::str::contains("run-uri"));
}

#[test]
fn skill_describe_outputs_json_for_unlock() {
    let ctx = TestContext::new();

    ctx.binary()
        .arg("skill")
        .arg("describe")
        .arg("unlock")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"unlock\""))
        .stdout(predicate::str::contains("VAULTWARDEN_PASSWORD"));
}

#[test]
fn skill_describe_rejects_unknown_command() {
    let ctx = TestContext::new();

    ctx.binary()
        .arg("skill")
        .arg("describe")
        .arg("missing-command")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Unknown command 'missing-command'",
        ));
}

#[test]
fn command_help_json_fallback_outputs_json_for_unlock() {
    let ctx = TestContext::new();

    ctx.binary()
        .arg("unlock")
        .arg("--help")
        .arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"unlock\""))
        .stdout(predicate::str::contains("\"usage\":"));
}
