use crate::helpers;

const HELP_MSG: &str = "Usage: qobuz <COMMAND>

Commands:
  load          Load an artist's releases into the database
  check         Check for new music from all the artists in the database
  list          List all the artists in the database
  gen-playlist  Generate a playlist with all the latest releases
  check-gen     Check for new music and put all the latest releases into a playlist
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
";

#[test]
fn help() {
    assert_cmd::Command::cargo_bin("qobuz")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(HELP_MSG);
}

const LOAD_AND_LIST_1: &str = "Loading data for \'AVRALIZE\'
Loaded 7 releases
";

const LOAD_AND_LIST_2: &str = "AVRALIZE
";

#[tokio::test]
async fn list() {
    let test = helpers::Test::init().await;

    // There should be no output as we have loaded no artists.
    let mut cmd = assert_cmd::Command::cargo_bin("qobuz").unwrap();
    cmd.envs(test.vars.clone()).arg("list").assert().stdout("");

    // Load an artist.
    let mut cmd = assert_cmd::Command::cargo_bin("qobuz").unwrap();
    cmd.envs(test.vars.clone())
        .args(["load", "13925362"])
        .assert()
        .stdout(LOAD_AND_LIST_1);

    // Now there should be one artist.
    let mut cmd = assert_cmd::Command::cargo_bin("qobuz").unwrap();
    cmd.envs(test.vars.clone())
        .arg("list")
        .assert()
        .stdout(LOAD_AND_LIST_2);
}

const CHECK_1: &str = "Checking 1 artists

Found 1 new release for AVRALIZE
  • helium
";

#[tokio::test]
async fn check() {
    let test = helpers::Test::init().await;

    // There should be no output as we have loaded no artists.
    let mut cmd = assert_cmd::Command::cargo_bin("qobuz").unwrap();
    cmd.envs(test.vars.clone()).arg("list").assert().stdout("");

    // Load an artist.
    let mut cmd = assert_cmd::Command::cargo_bin("qobuz").unwrap();
    cmd.envs(test.vars.clone())
        .args(["load", "13925362"])
        .assert()
        .stdout(LOAD_AND_LIST_1);

    // Now there should be one artist.
    let mut cmd = assert_cmd::Command::cargo_bin("qobuz").unwrap();
    cmd.envs(test.vars.clone())
        .arg("list")
        .assert()
        .stdout(LOAD_AND_LIST_2);

    let mut cmd = assert_cmd::Command::cargo_bin("qobuz").unwrap();
    cmd.envs(test.vars.clone())
        .arg("check")
        .assert()
        .stdout(CHECK_1);
}

#[tokio::test]
async fn gen_playlist() {
    let test = helpers::Test::init().await;

    // Load an artist.
    let mut cmd = assert_cmd::Command::cargo_bin("qobuz").unwrap();
    cmd.envs(test.vars.clone())
        .args(["load", "13925362"])
        .assert()
        .stdout(LOAD_AND_LIST_1);

    // Generate a playlist.
    let now = chrono::Local::now().date_naive().to_string();
    let expected_stdout = format!("Created playlist: {now}\n");
    let mut cmd = assert_cmd::Command::cargo_bin("qobuz").unwrap();
    cmd.envs(test.vars.clone())
        .arg("gen-playlist")
        .assert()
        .stdout(expected_stdout);
}
