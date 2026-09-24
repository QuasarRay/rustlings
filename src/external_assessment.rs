//! Optional trainer-owned assessment, run after the learner's program succeeds.
//! Editing an exercise to return success cannot bypass this check. For hostile
//! learners, run the trainer/observer outside their OS identity and container.
use anyhow::{Context, Result, ensure};
use ncp_assessment_kernel::{current_receipt, facets_complete, tier_matches, triad};
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
    process::{Command, Stdio},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use wait_timeout::ChildExt;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    protocol: String,
    program: PathBuf,
    #[serde(default)]
    args: Vec<String>,
    timeout_secs: u64,
    exercises: BTreeMap<String, Requirement>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Requirement {
    id: u16,
    tier: u8,
    required_facets: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    protocol: String,
    name: String,
    attempt: u64,
    exercise: u16,
    tier: u8,
    ops: u8,
    net: u8,
    infra: u8,
    coupled: bool,
    provenance: bool,
    observed_facets: u64,
    feedback: String,
}
fn valid(receipt: &Receipt, requirement: &Requirement, name: &str, attempt: u64) -> bool {
    receipt.protocol == "ncp-grade-v1"
        && receipt.name == name
        && current_receipt(attempt, receipt.attempt, requirement.id, receipt.exercise)
        && tier_matches(requirement.tier, receipt.tier)
        && triad(
            receipt.ops,
            receipt.net,
            receipt.infra,
            receipt.coupled,
            receipt.provenance,
            facets_complete(requirement.required_facets, receipt.observed_facets),
        )
}

pub fn check(name: &str, mut output: Option<&mut Vec<u8>>) -> Result<bool> {
    let Some(config_path) = std::env::var_os("NCP_GRADER_CONFIG") else {
        return Ok(true);
    };
    let config_path = PathBuf::from(config_path);
    ensure!(
        config_path.is_absolute(),
        "NCP_GRADER_CONFIG must be a trainer-owned absolute path"
    );
    let bytes = fs::read(config_path).context("read trainer assessment config")?;
    ensure!(bytes.len() <= 1024 * 1024, "assessment config too large");
    let config: Config = serde_json::from_slice(&bytes)?;
    ensure!(
        config.protocol == "ncp-grade-v1"
            && config.program.is_absolute()
            && (1..=600).contains(&config.timeout_secs),
        "invalid assessment protocol, executable or deadline"
    );
    let requirement = config
        .exercises
        .get(name)
        .context("exercise has no trainer assessment; refusing ungraded completion")?;
    let attempt: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_nanos()
        .try_into()?;
    let request = serde_json::json!({"protocol":"ncp-grade-v1","name":name,"exercise":requirement.id,"attempt":attempt});
    let mut stdout = tempfile::tempfile()?;
    let mut stderr = tempfile::tempfile()?;
    let mut child = Command::new(&config.program)
        .args(&config.args)
        .stdin(Stdio::piped())
        .stdout(stdout.try_clone()?)
        .stderr(stderr.try_clone()?)
        .spawn()?;
    child
        .stdin
        .take()
        .context("grader stdin")?
        .write_all(&serde_json::to_vec(&request)?)?;
    let status = match child.wait_timeout(Duration::from_secs(config.timeout_secs))? {
        Some(status) => status,
        None => {
            child.kill()?;
            child.wait()?;
            anyhow::bail!("assessment timed out; no credit awarded");
        }
    };
    ensure!(
        stdout.metadata()?.len() <= 65536 && stderr.metadata()?.len() <= 65536,
        "assessment output exceeds 64 KiB"
    );
    if !status.success() {
        stderr.seek(SeekFrom::Start(0))?;
        let mut error = String::new();
        stderr.read_to_string(&mut error)?;
        if let Some(out) = output.as_deref_mut() {
            out.extend_from_slice(b"External assessment failed; inspect the trainer report.\n");
        }
        return Ok(false);
    }
    stdout.seek(SeekFrom::Start(0))?;
    let receipt: Receipt =
        serde_json::from_reader(stdout).context("invalid assessment receipt; no credit awarded")?;
    let passed = valid(&receipt, requirement, name, attempt);
    if let Some(out) = output {
        let feedback: String = receipt
            .feedback
            .chars()
            .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
            .take(4096)
            .collect();
        out.extend_from_slice(feedback.as_bytes());
        out.push(b'\n');
        if !passed {
            out.extend_from_slice(b"Integrated assessment incomplete: all three domains, coupling, provenance, facets and exact execution tier are required.\n");
        }
    }
    Ok(passed)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn receipt() -> Receipt {
        Receipt {
            protocol: "ncp-grade-v1".into(),
            name: "e01a".into(),
            attempt: 99,
            exercise: 0,
            tier: 1,
            ops: 2,
            net: 2,
            infra: 2,
            coupled: true,
            provenance: true,
            observed_facets: 7,
            feedback: String::new(),
        }
    }
    #[test]
    fn reject_stale_partial_uncoupled_and_wrong_tier_receipts() {
        let req = Requirement {
            id: 0,
            tier: 1,
            required_facets: 7,
        };
        assert!(valid(&receipt(), &req, "e01a", 99));
        assert!(!valid(&receipt(), &req, "e01a", 100));
        for mutate in [
            |r: &mut Receipt| r.ops = 1,
            |r: &mut Receipt| r.net = 0,
            |r: &mut Receipt| r.infra = 3,
            |r: &mut Receipt| r.coupled = false,
            |r: &mut Receipt| r.provenance = false,
            |r: &mut Receipt| r.observed_facets = 3,
            |r: &mut Receipt| r.tier = 4,
            |r: &mut Receipt| r.exercise = 1,
        ] {
            let mut r = receipt();
            mutate(&mut r);
            assert!(!valid(&r, &req, "e01a", 99));
        }
    }
}
