//! Presentation of actual tool diagnostics. This module never supplies a repair.
use std::collections::BTreeSet;

/// Rust itself supports short/full traces. In this course a numeric depth
/// displays that many groups of 20 frames from a recorded full Rust backtrace.
#[derive(Clone, Copy)]
pub enum TraceDepth {
    Frames(usize),
    Full,
}
impl TraceDepth {
    pub fn parse(value: &str) -> Result<Self, String> {
        if value == "full" {
            return Ok(Self::Full);
        }
        value
            .parse::<usize>()
            .map(|n| Self::Frames(n.saturating_mul(20)))
            .map_err(|_| "Trace depth must be 0, a positive number, or full".into())
    }
    pub fn environment() -> Self {
        std::env::var("RUST_BACKTRACE")
            .ok()
            .and_then(|s| Self::parse(&s).ok())
            .unwrap_or(Self::Frames(0))
    }
}

/// Only trim numbered stack frames and their indented continuations. Compiler
/// messages, panic payloads, assertions and test summaries are left verbatim.
/// Unknown backtrace formats pass through rather than losing diagnostic text.
pub fn render(report: &str, depth: TraceDepth) -> String {
    let TraceDepth::Frames(limit) = depth else {
        return report.to_owned();
    };
    let mut output = String::new();
    let mut in_trace = false;
    let mut frames = 0;
    let mut skipping = false;
    let mut omitted = 0;
    for line in report.split_inclusive('\n') {
        let trimmed = line.trim();
        if trimmed == "stack backtrace:" || trimmed == "Stack backtrace:" {
            if omitted > 0 {
                output.push_str(&format!("    ... {omitted} frames omitted; increase the course trace depth or use full.\n"));
                omitted = 0;
            }
            in_trace = true;
            frames = 0;
            skipping = false;
        } else if in_trace {
            let numbered = trimmed.split_once(':').is_some_and(|(n, rest)| {
                !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()) && rest.starts_with(' ')
            });
            if numbered {
                frames += 1;
                skipping = frames > limit;
                if skipping {
                    omitted += 1;
                }
            } else if trimmed.is_empty() || !line.starts_with(char::is_whitespace) {
                if omitted > 0 {
                    output.push_str(&format!("    ... {omitted} frames omitted; increase the course trace depth or use full.\n"));
                    omitted = 0;
                }
                in_trace = false;
                skipping = false;
            }
        }
        if !skipping {
            output.push_str(line);
        }
    }
    if omitted > 0 {
        output.push_str(&format!(
            "    ... {omitted} frames omitted; increase the course trace depth or use full.\n"
        ));
    }
    output
}

/// Put the failed stage first. A behavioral failure uses libtest's own failure
/// section; successful-test chatter and earlier stages remain in the raw log.
pub fn failure_report(report: &str) -> String {
    let stage = report
        .rsplit_once("\ncargo ")
        .map_or_else(|| report.to_owned(), |(_, tail)| format!("cargo {tail}"));
    if stage.starts_with("cargo test ")
        && let Some((_, failures)) = stage.split_once("\nfailures:\n")
    {
        return format!(
            "{}\n\nfailures:\n{failures}",
            stage.lines().next().unwrap_or_default()
        );
    }
    stage
}

pub struct Origin {
    pub generated: String,
    pub first: usize,
    pub last: usize,
    pub editable: String,
    pub editable_first: usize,
}

/// Add editable coordinates alongside Rust's unchanged generated coordinates.
/// No suggested replacements or diagnostic explanations are manufactured.
pub fn locations(report: &str, origins: &[Origin]) -> String {
    let mut locations = BTreeSet::new();
    for line in report.lines() {
        let line = line.replace('\\', "/");
        for origin in origins {
            let needle = format!("{}:", origin.generated);
            let Some((_, tail)) = line.split_once(&needle) else {
                continue;
            };
            let mut fields = tail.split(':');
            let Some(number) = fields.next().and_then(|n| n.parse::<usize>().ok()) else {
                continue;
            };
            if !(origin.first..=origin.last).contains(&number) {
                continue;
            }
            let column = fields
                .next()
                .unwrap_or("1")
                .trim_end_matches(|c: char| !c.is_ascii_digit());
            let column = column.parse::<usize>().unwrap_or(1);
            locations.insert(format!(
                "{}:{}:{column}",
                origin.editable,
                origin.editable_first + number - origin.first
            ));
        }
    }
    locations
        .into_iter()
        .map(|path| format!("Editable location: {path}\n"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn depth_is_progressive_and_keeps_panic_and_assertion_details() {
        let mut report = "thread 'contract' panicked at src/test.rs:3:9:\nassertion failed: counter\n  left: 0\n right: 1\nstack backtrace:\n".to_owned();
        for n in 0..65 {
            report.push_str(&format!(
                "  {n:2}: 0x123 - frame_{n}\n             at src/main.rs:{}:1\n",
                n + 1
            ));
        }
        report.push_str("\nfailures:\n    contract\ntest result: FAILED\n");
        for level in 0..=4 {
            let rendered = render(&report, TraceDepth::parse(&level.to_string()).unwrap());
            assert!(rendered.contains("left: 0\n right: 1"));
            assert!(rendered.contains("test result: FAILED"));
            assert_eq!(
                rendered.matches("0x123 - frame_").count(),
                (level * 20).min(65)
            );
        }
        assert_eq!(render(&report, TraceDepth::Full), report);
        assert!(TraceDepth::parse("nonsense").is_err());
    }
    #[test]
    fn compiler_suggestions_and_unrecognized_trace_formats_are_unchanged() {
        let report = "error[E0308]: mismatched types\nhelp: consider borrowing here\n\nstack backtrace:\n    platform-specific frame\n\nerror: could not compile\n";
        assert_eq!(render(report, TraceDepth::Frames(0)), report);
    }
    #[test]
    fn failed_stage_keeps_rust_help_and_libtest_assertions_without_success_chatter() {
        let compiler = "cargo check --locked --workspace\nerror[E0308]: mismatched types\nhelp: consider borrowing here\n";
        assert_eq!(failure_report(compiler), compiler);
        let report = "cargo check --locked\nFinished\ncargo test --locked --workspace\ntest unrelated ... ok\nfailures:\n\n---- actual_contract stdout ----\nassertion failed: counter\n  left: 0\n right: 1\n\nfailures:\n    actual_contract\ntest result: FAILED\n";
        let displayed = failure_report(report);
        assert!(!displayed.contains("unrelated"));
        assert!(!displayed.contains("cargo check"));
        assert!(displayed.contains("---- actual_contract stdout ----"));
        assert!(displayed.contains("left: 0\n right: 1"));
        assert!(displayed.contains("test result: FAILED"));
    }
    #[test]
    fn maps_only_repair_lines_including_windows_and_panic_locations() {
        let origins = [Origin {
            generated: "src/info_file.rs".into(),
            first: 40,
            last: 45,
            editable: "exercises/01_catalog/catalog_paths.rs".into(),
            editable_first: 21,
        }];
        let report = " --> src/info_file.rs:42:9\nthread 't' panicked at C:\\temp\\src\\info_file.rs:45:3:\n --> src/info_file.rs:60:1\n";
        let locations = locations(report, &origins);
        assert!(locations.contains("catalog_paths.rs:23:9"));
        assert!(locations.contains("catalog_paths.rs:26:3"));
        assert!(!locations.contains(":41:1"));
    }
}
