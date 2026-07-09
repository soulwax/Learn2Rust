//! Parses `cargo test` stdout for `test result: ...` summary lines and
//! sums pass/fail counts across all of them (a crate can print one such
//! line per test binary: unit tests, each integration test file, doctests).

/// Sums `passed`/`failed` counts across every `test result: ...` line in
/// `stdout`. Lines that don't match the pattern are ignored.
pub fn parse_test_summary(stdout: &str) -> (u32, u32) {
    let mut passed = 0;
    let mut failed = 0;

    for line in stdout.lines() {
        let Some(rest) = line.trim_start().strip_prefix("test result:") else {
            continue;
        };
        // rest looks like " ok. 3 passed; 0 failed; 0 ignored; ..."
        for part in rest.split(';') {
            let part = part.trim();
            if let Some(n) = part.strip_suffix(" passed") {
                if let Some(n) = n.trim().rsplit(' ').next() {
                    passed += n.parse::<u32>().unwrap_or(0);
                }
            } else if let Some(n) = part.strip_suffix(" failed") {
                if let Some(n) = n.trim().rsplit(' ').next() {
                    failed += n.parse::<u32>().unwrap_or(0);
                }
            }
        }
    }

    (passed, failed)
}

#[cfg(test)]
mod tests {
    use super::parse_test_summary;

    #[test]
    fn parses_single_passing_summary_line() {
        let stdout = "running 3 tests\ntest a ... ok\n\ntest result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n";

        assert_eq!(parse_test_summary(stdout), (3, 0));
    }

    #[test]
    fn parses_summary_line_with_failures() {
        let stdout = "test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n";

        assert_eq!(parse_test_summary(stdout), (2, 1));
    }

    #[test]
    fn sums_multiple_summary_lines() {
        let stdout = "\
Running unittests src\\lib.rs
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

Running tests\\cli.rs
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

Doc-tests focus_forge_status
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
";

        assert_eq!(parse_test_summary(stdout), (7, 0));
    }

    #[test]
    fn no_summary_lines_returns_zero() {
        assert_eq!(parse_test_summary("nothing relevant here\n"), (0, 0));
    }
}
