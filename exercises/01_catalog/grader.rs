//! Source-restoration verifier for the course. The Rustlings engine is unchanged.
//! rustc --edition=2024 exercises/01_catalog/grader.rs -o target/workshop-grader
//! target/workshop-grader ROOT prepare|audit|check ID ROLE|export DEST [solutions]
#![forbid(unsafe_code)]
use std::{
    collections::{BTreeMap, hash_map::DefaultHasher},
    error::Error,
    fs::{self, File, OpenOptions},
    hash::{Hash, Hasher},
    io::Read,
    path::{Component, Path, PathBuf},
    process::Command,
    thread,
    time::{Duration, Instant},
};

mod diagnostics;
mod process;

#[derive(Debug)]
struct GradingError {
    code: u8,
    message: String,
}
impl std::fmt::Display for GradingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}
impl Error for GradingError {}
fn outcome<T>(code: u8, message: impl Into<String>) -> Result<T> {
    Err(Box::new(GradingError {
        code,
        message: message.into(),
    }))
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Files = BTreeMap<String, String>;
const BUILD_ID_ENV: &str = "WORKSHOP_COMPILED_INPUT";
fn build_identity_probe(label: &str) -> String {
    format!(
        r#"
// Cargo tracks this environment dependency even when source timestamps repeat.
const _: Option<&str> = option_env!("WORKSHOP_COMPILED_INPUT");
#[cfg(test)]
#[test]
fn workshop_build_identity() {{
    let runtime = std::env::var("WORKSHOP_COMPILED_INPUT").ok();
    assert_eq!(option_env!("WORKSHOP_COMPILED_INPUT"), runtime.as_deref(), "WORKSHOP_STALE_ARTIFACT");
    println!("WORKSHOP_BUILD_ID:{label}:{{}}", runtime.as_deref().unwrap_or("manual"));
}}
"#
    )
}
const BEGIN: &str = "// BEGIN RUSTLINGS REPAIR\n";
const END: &str = "\n// END RUSTLINGS REPAIR";
#[derive(Clone)]
struct Mission {
    id: usize,
    dir: String,
    name: String,
    file: String,
    start: usize,
    end: usize,
}
struct Workshop {
    root: PathBuf,
    cache: PathBuf,
    base: Files,
    probes: Files,
    missions: Vec<Mission>,
    identity: String,
}

fn fail<T>(message: impl Into<String>) -> Result<T> {
    Err(message.into().into())
}
fn fragment(source: &str) -> Result<String> {
    let (_, rest) = source
        .split_once(BEGIN)
        .ok_or("missing BEGIN repair marker")?;
    let (body, _) = rest.split_once(END).ok_or("missing END repair marker")?;
    if body.contains(BEGIN) {
        return fail("duplicate repair marker");
    }
    Ok(body.to_owned())
}
fn digest(bytes: &[u8]) -> String {
    let mut h = DefaultHasher::new();
    bytes.hash(&mut h);
    format!("{:016x}", h.finish())
}
fn archive(bytes: &[u8]) -> Result<Files> {
    let mut rest = bytes
        .strip_prefix(b"RUSTLINGS_WORKSHOP_V1\n")
        .ok_or("unsupported snapshot format")?;
    let mut files = Files::new();
    while !rest.is_empty() {
        let n = rest
            .iter()
            .position(|b| *b == b'\n')
            .ok_or("truncated archive header")?;
        let header = str::from_utf8(&rest[..n])?;
        let (path, len) = header.split_once('\t').ok_or("bad archive header")?;
        if Path::new(path)
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
        {
            return fail("snapshot paths must be relative and contained");
        }
        let len: usize = len.parse()?;
        rest = &rest[n + 1..];
        let content = rest.get(..len).ok_or("truncated archive file")?;
        if files
            .insert(path.to_owned(), str::from_utf8(content)?.to_owned())
            .is_some()
        {
            return fail("duplicate snapshot path");
        }
        rest = rest
            .get(len..)
            .and_then(|r| r.strip_prefix(b"\n"))
            .ok_or("bad archive separator")?;
    }
    Ok(files)
}
fn write_changed(path: &Path, content: &[u8]) -> Result<()> {
    if fs::read(path).ok().as_deref() == Some(content) {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}
fn serialize(files: &Files, identity: &str) -> Vec<u8> {
    let mut bytes = identity.as_bytes().to_vec();
    for (p, c) in files {
        bytes.extend_from_slice(format!("\n{}:{}:{}\n", p.len(), c.len(), p).as_bytes());
        bytes.extend_from_slice(c.as_bytes());
    }
    bytes
}
// A generated project must contain exactly the requested inputs. In particular,
// a stale build.rs, Cargo configuration, or test must never participate in grading.
fn reconcile(directory: &Path, files: &Files) -> Result<()> {
    fn visit(root: &Path, dir: &Path, files: &Files) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let relative = path
                .strip_prefix(root)?
                .to_string_lossy()
                .replace('\\', "/");
            let kind = entry.file_type()?;
            if kind.is_dir() {
                if files.keys().any(|p| p.starts_with(&format!("{relative}/"))) {
                    visit(root, &path, files)?;
                } else {
                    fs::remove_dir_all(path)?;
                }
            } else if kind.is_symlink() || !files.contains_key(&relative) {
                fs::remove_file(path)?;
            }
        }
        Ok(())
    }
    if fs::symlink_metadata(directory).is_ok_and(|m| m.file_type().is_symlink()) {
        return fail("generated project directory must not be a symbolic link");
    }
    fs::create_dir_all(directory)?;
    visit(directory, directory, files)
}

fn environment_identity(root: &Path) -> Result<Vec<u8>> {
    let mut inputs = Files::new();
    inputs.insert(
        "platform".into(),
        format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
    );
    for (key, value) in std::env::vars_os() {
        let name = key.to_string_lossy();
        // Trace presentation never changes the checked source or prerequisites.
        // Child checks always record full traces; the display depth is separate.
        if ["RUST_BACKTRACE", "RUST_LIB_BACKTRACE"].contains(&name.as_ref()) {
            continue;
        }
        if name.starts_with("RUST")
            || name.starts_with("CARGO_")
            || [
                "PATH",
                "CC",
                "CXX",
                "AR",
                "LD",
                "CFLAGS",
                "CXXFLAGS",
                "LDFLAGS",
                "SDKROOT",
                "MACOSX_DEPLOYMENT_TARGET",
            ]
            .contains(&name.as_ref())
        {
            // Cargo changes these labels for each adapter. They do not configure
            // the restored project, and must not invalidate cumulative receipts.
            if ["CARGO_BIN_NAME", "CARGO_MAKEFLAGS"].contains(&name.as_ref())
                || name.starts_with("CARGO_PKG_")
                || name.starts_with("CARGO_MANIFEST_")
            {
                continue;
            }
            inputs.insert(format!("env:{name}"), format!("{value:?}"));
        }
    }
    let mut directories: Vec<_> = root
        .join("target/workshop/engine")
        .ancestors()
        .map(Path::to_path_buf)
        .collect();
    if let Some(home) = std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".cargo")))
        .or_else(|| std::env::var_os("USERPROFILE").map(|p| PathBuf::from(p).join(".cargo")))
    {
        directories.push(home.parent().unwrap_or(&home).to_path_buf());
        for name in ["config", "config.toml"] {
            let path = home.join(name);
            if path.is_file() {
                inputs.insert(
                    format!("config:{}", path.display()),
                    fs::read_to_string(path)?,
                );
            }
        }
    }
    for directory in directories {
        for name in ["config", "config.toml"] {
            let path = directory.join(".cargo").join(name);
            if path.is_file() {
                inputs.insert(
                    format!("config:{}", path.display()),
                    fs::read_to_string(path)?,
                );
            }
        }
    }
    Ok(serialize(&inputs, "course-environment-v1"))
}
// OS-owned lock lifetime survives neither normal exit nor interruption.
// Keeping the file itself is harmless: the lock belongs to this open handle.
struct Lock {
    _file: File,
}
impl Workshop {
    fn open(root: PathBuf) -> Result<Self> {
        let root = root.canonicalize()?;
        let support = root.join("exercises/01_catalog");
        let upstream = fs::read(support.join("upstream.txt"))?;
        let probes_raw = fs::read(support.join("probes.txt"))?;
        let map = fs::read_to_string(support.join("missions.tsv"))?;
        let mut missions = Vec::new();
        for line in map.lines() {
            let f: Vec<_> = line.split('\t').collect();
            if f.len() != 6 {
                return fail("invalid mission registry");
            }
            let m = Mission {
                id: f[0].parse()?,
                dir: f[1].into(),
                name: f[2].into(),
                file: f[3].into(),
                start: f[4].parse()?,
                end: f[5].parse()?,
            };
            if m.id != missions.len() + 1 || m.end <= m.start {
                return fail("invalid mission sequence or range");
            }
            missions.push(m);
        }
        let base = archive(&upstream)?;
        let probes = archive(&probes_raw)?;
        for m in &missions {
            base.get(&m.file)
                .and_then(|s| s.get(m.start..m.end))
                .ok_or("mission range outside its source file")?;
        }
        let mut ranges = BTreeMap::<&str, Vec<(usize, usize)>>::new();
        for m in &missions {
            ranges.entry(&m.file).or_default().push((m.start, m.end));
        }
        for r in ranges.values_mut() {
            r.sort();
            if r.windows(2).any(|w| w[0].1 > w[1].0) {
                return fail("overlapping mission repairs");
            }
        }
        let mut identity = upstream;
        identity.extend_from_slice(&probes_raw);
        identity.extend_from_slice(map.as_bytes());
        identity.extend_from_slice(&fs::read(support.join("grader.rs"))?);
        identity.extend_from_slice(&fs::read(support.join("process.rs"))?);
        identity.extend_from_slice(&fs::read(support.join("diagnostics.rs"))?);
        identity.extend_from_slice(&environment_identity(&root)?);
        for tool in ["rustc", "cargo"] {
            let out = Command::new(tool)
                .arg("--version")
                .arg("--verbose")
                .output()?;
            if !out.status.success() {
                return fail(format!("{tool} is unavailable"));
            }
            identity.extend_from_slice(&out.stdout);
        }
        let identity = digest(&identity);
        let cache = root.join("target/workshop");
        fs::create_dir_all(&cache)?;
        Ok(Self {
            root,
            cache,
            base,
            probes,
            missions,
            identity,
        })
    }
    fn read_fragment(&self, m: &Mission, role: &str) -> Result<String> {
        fragment(&fs::read_to_string(
            self.root
                .join(role)
                .join(&m.dir)
                .join(format!("{}.rs", m.name)),
        )?)
    }
    fn original(&self, m: &Mission) -> String {
        self.base[&m.file][m.start..m.end].to_owned()
    }
    fn assembled(&self, repairs: &[String]) -> Result<Files> {
        if repairs.len() > self.missions.len() {
            return fail("too many repairs");
        }
        let mut files = self.base.clone();
        let mut changes: Vec<_> = self.missions.iter().zip(repairs).collect();
        changes.sort_by(|(a, _), (b, _)| a.file.cmp(&b.file).then(b.start.cmp(&a.start)));
        for (m, body) in changes {
            files
                .get_mut(&m.file)
                .unwrap()
                .replace_range(m.start..m.end, body);
        }
        Ok(files)
    }
    fn receipt(&self, id: usize) -> PathBuf {
        self.cache.join(format!("progress/{id:03}.txt"))
    }
    fn prefix_key(&self, repairs: &[String]) -> String {
        let mut s = self.identity.clone();
        for r in repairs {
            s.push_str(&format!("\n{}:{}", r.len(), r));
        }
        s
    }
    fn prior_repairs(&self, index: usize, role: &str) -> Result<Vec<String>> {
        let mut repairs = Vec::new();
        for m in self.missions.iter().take(index) {
            repairs.push(self.read_fragment(m, role)?);
            if role == "exercises"
                && fs::read_to_string(self.receipt(m.id)).ok().as_deref()
                    != Some(&self.prefix_key(&repairs))
            {
                return outcome(
                    3,
                    format!(
                        "LOCKED: complete or recheck `{}` first. An earlier repair is missing, changed, or was verified with a different toolchain.\nRun Rustlings in mission order; no edits to solutions are needed.",
                        m.name
                    ),
                );
            }
        }
        Ok(repairs)
    }
    fn lock(&self, deadline: Instant) -> Result<Lock> {
        let path = self.cache.join("compiler.lock");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)?;
        loop {
            match file.try_lock() {
                Ok(()) => return Ok(Lock { _file: file }),
                Err(std::fs::TryLockError::WouldBlock) => {
                    if Instant::now() >= deadline {
                        return outcome(
                            4,
                            format!(
                                "Another course check owns {}. Retry when it finishes.",
                                path.display()
                            ),
                        );
                    }
                    thread::sleep(Duration::from_millis(40));
                }
                Err(std::fs::TryLockError::Error(e)) => return Err(e.into()),
            }
        }
    }
    fn cached(&self, key: &str, input: &[u8]) -> Result<Option<(bool, String)>> {
        let dir = self.cache.join("results");
        if fs::read(dir.join(format!("{key}.input"))).ok().as_deref() != Some(input) {
            return Ok(None);
        }
        let Ok(report) = fs::read_to_string(dir.join(format!("{key}.report"))) else {
            return Ok(None);
        };
        let Some((status, output)) = report.split_once('\n') else {
            return Ok(None);
        };
        match status {
            "PASS" => Ok(Some((true, output.into()))),
            // A tool/filesystem failure can surface as a failed Cargo test.
            // Retrying must recover without changing a learner's source.
            "FAIL" => Ok(None),
            _ => Ok(None),
        }
    }
    fn evaluate(&self, files: &Files, _long: bool) -> Result<(bool, String)> {
        self.evaluate_filtered(files, None)
    }
    fn evaluate_filtered(&self, files: &Files, filter: Option<&str>) -> Result<(bool, String)> {
        let identity = format!("{}\nfilter={filter:?}", self.identity);
        let input = serialize(files, &identity);
        let key = digest(&input);
        if let Some(report) = self.cached(&key, &input)? {
            return Ok(report);
        }
        // One budget owns queueing, compilation, tests and Clippy together. The course host
        // grants 600 seconds, leaving 60 seconds for startup and cleanup.
        let deadline = Instant::now() + Duration::from_secs(540);
        let _lock = self.lock(deadline)?;
        if let Some(report) = self.cached(&key, &input)? {
            return Ok(report);
        }
        // Force a fresh crate rebuild on an uncached attempt, including retries
        // after infrastructure failure; dependency artifacts remain reusable.
        let build_id = format!(
            "{key}:{}:{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos()
        );
        let build = self.cache.join("engine");
        reconcile(&build, files)?;
        for (path, text) in files {
            let mut text = text.clone();
            // The original excluded exercise packages assume there is no outer
            // Cargo workspace. Our disposable build is nested in the course.
            // Fence these test-only workspaces; never put this shim in exports.
            if path == "dev/Cargo.toml" || path == "tests/test_exercises/dev/Cargo.toml" {
                text.push_str("\n[workspace]\n");
            }
            if let Some(probes) = self.probes.get(path) {
                text.push_str("\n");
                text.push_str(probes);
            }
            if path == "src/main.rs" || path == "rustlings-macros/src/lib.rs" {
                text.push_str(&build_identity_probe(path));
            }
            write_changed(&build.join(path), text.as_bytes())?;
        }
        let target = self.cache.join("engine-target");
        let mut report = String::new();
        let mut passed = true;
        let mut test_args = vec!["test", "--locked", "--workspace"];
        if let Some(filter) = filter {
            test_args.push(filter);
        }
        test_args.extend(["--", "--test-threads=1"]);
        let mut commands: Vec<&[&str]> = vec![
            // Type errors belong to the repair. Lints about unused scaffold are
            // useful only after the implementation satisfies its contracts.
            &["check", "--locked", "--workspace", "--all-targets"],
            &[
                "test",
                "--locked",
                "--workspace",
                "workshop_build_identity",
                "--",
                "--test-threads=1",
                "--nocapture",
            ],
            &test_args,
        ];
        // This upstream branch is deliberately disabled in debug builds.
        // Check its release behavior as well; ordinary debug tests cannot do so.
        if filter.is_none() || filter == Some("app_state::") {
            commands.push(&[
                "test",
                "--locked",
                "--release",
                "--bin",
                "rustlings",
                "workshop_release_contracts",
                "--",
                "--test-threads=1",
            ]);
        }
        commands.push(&[
            "clippy",
            "--locked",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ]);
        for args in commands {
            let mut cmd = Command::new("cargo");
            cmd.args(args)
                .current_dir(&build)
                .env("CARGO_TARGET_DIR", &target)
                .env(BUILD_ID_ENV, &build_id)
                .env("RUST_BACKTRACE", "full")
                .env("RUST_LIB_BACKTRACE", "full")
                .env("CARGO_TERM_COLOR", "never");
            if std::env::var_os("CARGO_BUILD_JOBS").is_none() {
                cmd.env("CARGO_BUILD_JOBS", "2");
            }
            let budget = deadline.saturating_duration_since(Instant::now());
            let (status, output) = process::capture(&mut cmd, budget).map_err(|e| {
                Box::new(GradingError {
                    code: if e.kind() == std::io::ErrorKind::TimedOut {
                        4
                    } else {
                        2
                    },
                    message: format!("INFRA_ERROR: {e}"),
                }) as Box<dyn Error>
            })?;
            report.push_str(&format!("cargo {}\n", args.join(" ")));
            report.push_str(&String::from_utf8_lossy(&output));
            if args.contains(&"workshop_build_identity") {
                let output = String::from_utf8_lossy(&output);
                if !status.success()
                    || ["src/main.rs", "rustlings-macros/src/lib.rs"]
                        .iter()
                        .any(|path| {
                            !output.contains(&format!("WORKSHOP_BUILD_ID:{path}:{build_id}"))
                        })
                {
                    return outcome(
                        2,
                        format!(
                            "INFRA_ERROR: compiled artifacts do not match the reconstructed input; no grading result was cached.\n{report}"
                        ),
                    );
                }
            }
            if !status.success() {
                if [
                    "Permission denied (os error 13)",
                    "kind: PermissionDenied",
                    "Text file busy (os error 26)",
                    "No space left on device (os error 28)",
                ]
                .iter()
                .any(|diagnostic| report.contains(diagnostic))
                {
                    return outcome(
                        2,
                        format!(
                            "INFRA_ERROR: a process or filesystem resource prevented evaluation. No rejection was cached; retry after resolving the resource error.\n{report}"
                        ),
                    );
                }
                passed = false;
                break;
            }
        }
        let dir = self.cache.join("results");
        fs::create_dir_all(&dir)?;
        // The full input is checked as well as its hash; hash collisions cannot grant a pass.
        if passed {
            fs::write(dir.join(format!("{key}.input")), input)?;
            fs::write(dir.join(format!("{key}.report")), format!("PASS\n{report}"))?;
        }
        Ok((passed, report))
    }
    fn origins(
        &self,
        repairs: &[String],
        role: &str,
        source: &str,
        source_path: Option<&Path>,
    ) -> Result<Vec<diagnostics::Origin>> {
        let mut origins = Vec::new();
        for (m, body) in self.missions.iter().zip(repairs) {
            let mut first = 1 + self.base[&m.file][..m.start]
                .bytes()
                .filter(|b| *b == b'\n')
                .count();
            // Earlier missions can occur later in the original file. Apply only
            // replacements physically before this one when translating lines.
            for (other, replacement) in self.missions.iter().zip(repairs) {
                if other.file == m.file && other.start < m.start {
                    first += replacement.bytes().filter(|b| *b == b'\n').count();
                    first -= self.base[&m.file][other.start..other.end]
                        .bytes()
                        .filter(|b| *b == b'\n')
                        .count();
                }
            }
            let editable = format!("{role}/{}/{}.rs", m.dir, m.name);
            let current = m.id == repairs.len();
            let text = if current {
                source.to_owned()
            } else {
                fs::read_to_string(self.root.join(&editable))?
            };
            let header = text.split_once(BEGIN).ok_or("missing repair marker")?.0;
            origins.push(diagnostics::Origin {
                generated: m.file.clone(),
                first,
                last: first + body.bytes().filter(|b| *b == b'\n').count(),
                editable: if current {
                    source_path
                        .map(|p| p.display().to_string())
                        .unwrap_or(editable)
                } else {
                    editable
                },
                editable_first: header.bytes().filter(|b| *b == b'\n').count() + 2,
            });
        }
        Ok(origins)
    }
    fn check(&self, id: usize, role: &str, source: &str, source_path: Option<&Path>) -> Result<()> {
        if !["exercises", "solutions"].contains(&role) {
            return fail("invalid source role");
        }
        let m = self
            .missions
            .get(id.checked_sub(1).ok_or("invalid mission")?)
            .ok_or("invalid mission")?;
        let mut repairs = self.prior_repairs(id - 1, role)?;
        repairs.push(fragment(source)?);
        let files = self.assembled(&repairs)?;
        let result = self.evaluate(&files, false);
        let (code, report) = match result {
            Ok((true, _)) => (0, String::new()),
            Ok((false, report)) => (1, report),
            Err(error) => (
                error.downcast_ref::<GradingError>().map_or(2, |e| e.code),
                error.to_string(),
            ),
        };
        if code != 0 {
            let origins = self.origins(&repairs, role, source, source_path)?;
            let active = origins.last().ok_or("missing current repair")?;
            let contract = fs::read_to_string(self.root.join("exercises/01_catalog/hints.tsv"))?
                .lines()
                .find_map(|line| {
                    let mut fields = line.split('\t');
                    (fields.next() == Some(m.name.as_str()))
                        .then(|| fields.next().unwrap_or_default().to_owned())
                })
                .unwrap_or_default();
            let label = if code == 1 {
                "REJECTED"
            } else {
                "INFRASTRUCTURE"
            };
            let header = format!(
                "{label}: mission {id:03} {}.\nContract: {contract}\nRepair: {}:{} (restores {}:{}..{}).\n{}",
                m.name,
                active.editable,
                active.editable_first,
                active.generated,
                active.first,
                active.last,
                diagnostics::locations(&report, &origins)
            );
            let path = self.cache.join(format!("diagnostics/{id:03}-{role}.log"));
            write_changed(&path, format!("{header}{report}").as_bytes())?;
            let displayed = if code == 1 {
                diagnostics::failure_report(&report)
            } else {
                report
            };
            return outcome(
                code,
                format!(
                    "{header}{}\nFull tool report: {}\nInspect more frames: rustlings workshop trace {} 1 (then 2, 3, or full).\n",
                    diagnostics::render(&displayed, diagnostics::TraceDepth::environment()),
                    path.display(),
                    m.name
                ),
            );
        }
        if role == "exercises" {
            write_changed(&self.receipt(id), self.prefix_key(&repairs).as_bytes())?;
        }
        println!(
            "MISSION {id:03}/{} CLEARED: {} | {} cumulative repairs compiled and checked",
            self.missions.len(),
            m.name,
            id
        );
        if id == self.missions.len() {
            println!(
                "Final gate passed. Export your reconstructed Rustlings using exercises/README.md."
            );
        }
        Ok(())
    }
    fn trace(&self, name: &str, depth: &str) -> Result<()> {
        let m = self
            .missions
            .iter()
            .find(|m| m.name == name)
            .ok_or("unknown mission name")?;
        let path = self
            .cache
            .join(format!("diagnostics/{:03}-exercises.log", m.id));
        let report = fs::read_to_string(&path).map_err(|_| {
            format!("No saved learner diagnostic for {name}. Run that mission in Rustlings first.")
        })?;
        let depth = diagnostics::TraceDepth::parse(depth)?;
        println!(
            "Saved tool report for {name}; rerun the mission after source changes.\n{}\nFull report: {}",
            diagnostics::render(&report, depth),
            path.display()
        );
        Ok(())
    }
    fn audit(&self) -> Result<()> {
        let baseline: Vec<_> = self.missions.iter().map(|m| self.original(m)).collect();
        let solutions: Vec<_> = self
            .missions
            .iter()
            .map(|m| self.read_fragment(m, "solutions"))
            .collect::<Result<_>>()?;
        let reconstructed = self.assembled(&solutions)?;
        if reconstructed != self.base {
            return fail("reference solutions do not reproduce the pinned project byte for byte");
        }
        let (pass, report) = self.evaluate(&reconstructed, true)?;
        if !pass {
            return fail(format!("Reference checker failed:\n{report}"));
        }
        let mut unhelpful = Vec::new();
        let mut evidence = String::from("id\tmission\trejection_stage\n");
        for m in &self.missions {
            let mut repairs = baseline.clone();
            repairs[m.id - 1] = self.read_fragment(m, "exercises")?;
            if repairs[m.id - 1] == baseline[m.id - 1] {
                return fail(format!("{} is already solved", m.name));
            }
            let (pass, report) = self.evaluate(&self.assembled(&repairs)?, true)?;
            let stage = if pass {
                "ESCAPED"
            } else if report
                .lines()
                .rev()
                .find(|l| l.starts_with("cargo "))
                .is_some_and(|l| l.starts_with("cargo clippy "))
            {
                "LINT_ONLY"
            } else if report.contains("test result: FAILED") {
                "CONTRACT"
            } else {
                "COMPILER"
            };
            evidence.push_str(&format!("{}\t{}\t{stage}\n", m.id, m.name));
            fs::write(self.cache.join("starter-audit.tsv"), &evidence)?;
            fs::write(self.cache.join(format!("starter-{:03}.log", m.id)), &report)?;
            if pass || stage == "LINT_ONLY" {
                unhelpful.push(format!("{}: {stage}", m.name));
            }
            println!(
                "{:03}/{} starter {stage}, reference PASS — {}",
                m.id,
                self.missions.len(),
                m.name
            );
        }
        if !unhelpful.is_empty() {
            return fail(format!(
                "Starters must fail compilation or a behavioral contract, without relying on lint denial:\n{}",
                unhelpful.join("\n")
            ));
        }
        let summary = format!(
            "{} independently failing starters; {} passing reference repairs; byte-identical reference reconstruction; original and added regression tests passed.\n",
            self.missions.len(),
            self.missions.len()
        );
        fs::write(self.cache.join("audit.txt"), &summary)?;
        println!("{summary}");
        Ok(())
    }
    fn mutations(&self, first: usize, last: usize) -> Result<()> {
        if first == 0 || first > last || last > self.missions.len() {
            return fail("mutation range must be inside the ordered mission catalog");
        }
        let baseline: Vec<_> = self.missions.iter().map(|m| self.original(m)).collect();
        let (pass, report) = self.evaluate(&self.base, true)?;
        if !pass {
            return fail(format!(
                "Reference failed before mutation testing:\n{report}"
            ));
        }
        let mut escaped = Vec::new();
        let mut evidence = String::from("id\tmission\tkind\ttarget\tresult\texecuted_tests\n");
        for m in self
            .missions
            .iter()
            .filter(|m| m.id >= first && m.id <= last)
        {
            let original = &baseline[m.id - 1];
            let mut variants = Vec::new();
            let mut mutant = original.clone();
            let marker = format!("WORKSHOP_MUTATION_{}", m.id);
            if original.contains("const fn") {
                let value = mutant.rfind("true").ok_or("missing boolean literal")?;
                mutant.replace_range(value..value + 4, "false");
                variants.push(("value", mutant));
            } else {
                let opening = original.find('{').ok_or("missing function body")? + 1;
                mutant.insert_str(
                    opening,
                    &format!("\nif std::hint::black_box(true) {{ panic!(\"{marker}\"); }}\n"),
                );
                variants.push(("reachability", mutant));
            }
            // Compiling semantic faults supplement the minimum reachability check.
            // Each changes a returned value, boundary, predicate or side effect.
            let semantic = match m.id {
                1 => Some(("push_str(\"exercises/\")", "push_str(\"solutions/\")")),
                4 => Some(("push_str(\"solutions/\")", "push_str(\"exercises/\")")),
                9 => Some((".arg(\"-q\")", ".arg(\"--verbose\")")),
                17 => Some((
                    "e.kind() != io::ErrorKind::AlreadyExists",
                    "e.kind() == io::ErrorKind::AlreadyExists",
                )),
                20 => Some(("exercise_files.exercise", "exercise_files.solution")),
                24 => Some(("as u32 - self.n_done", "as u32 + self.n_done")),
                28 => Some(("self.n_done += 1", "self.n_done += 2")),
                34 => Some(("if cfg!(debug_assertions)", "if std::hint::black_box(true)")),
                41 => Some(("self.len += n", "self.len += n + 1")),
                49 => Some((
                    "selected.saturating_sub(max_scroll_padding)",
                    "selected.saturating_add(max_scroll_padding)",
                )),
                54 => Some(("max_n_rows_to_display / 4", "max_n_rows_to_display / 2")),
                67 => Some(("store(true, Relaxed)", "store(false, Relaxed)")),
                74 => Some((
                    "current_exercise_ind() != exercise_ind",
                    "current_exercise_ind() == exercise_ind",
                )),
                75 => Some(("=> true,", "=> false,")),
                81 => Some(("self.term_width != width", "self.term_width == width")),
                86 => Some(("|status| status.success()", "|status| !status.success()")),
                91 => Some(("10 * id", "16 * id")),
                97 => Some((
                    "!c.is_alphanumeric() && *c != '_'",
                    "!c.is_alphanumeric() || *c != '_'",
                )),
                98 => Some(("old_bins != new_bins", "old_bins == new_bins")),
                _ => None,
            };
            if let Some((from, to)) = semantic {
                if !original.contains(from) {
                    return fail(format!("mutation anchor changed for {}", m.name));
                }
                variants.push(("value", original.replacen(from, to, 1)));
            }
            for (kind, mutant) in variants {
                let mut repairs = baseline.clone();
                repairs[m.id - 1] = mutant;
                let module = m
                    .file
                    .strip_prefix("src/")
                    .unwrap_or_default()
                    .trim_end_matches(".rs")
                    .replace('/', "::")
                    + "::";
                let filter = if self.probes.contains_key(&m.file) && m.file != "src/main.rs" {
                    Some(module.as_str())
                } else {
                    None
                };
                let files = self.assembled(&repairs)?;
                let (mut pass, mut report) = self.evaluate_filtered(&files, filter)?;
                // A local filter is only a speed optimization. Integration tests
                // and another module may be the real caller of this function.
                if pass && filter.is_some() {
                    (pass, report) = self.evaluate(&files, true)?;
                }
                // An identity test or a failed test compilation is not evidence
                // that the behavioral fault was reached by a running test.
                let ran_tests = report.contains("test result: FAILED");
                let macro_execution =
                    m.file == "rustlings-macros/src/lib.rs" && report.contains(&marker);
                let observed_fault = kind == "value" || report.contains(&marker);
                let status = if pass {
                    "ESCAPED"
                } else if !ran_tests && !macro_execution {
                    "INVALID_MUTANT"
                } else if !observed_fault {
                    "UNOBSERVED_FAILURE"
                } else {
                    "CAUGHT"
                };
                if status != "CAUGHT" {
                    escaped.push(format!("{} {} {kind}: {status}", m.id, m.name));
                }
                let tests = report
                    .lines()
                    .filter_map(|line| {
                        line.strip_prefix("test ")
                            .and_then(|line| line.split_once(" ... "))
                            .map(|(name, _)| name)
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                let tests = if macro_execution {
                    "proc_macro_expansion"
                } else {
                    &tests
                };
                evidence.push_str(&format!(
                    "{}\t{}\t{kind}\t{}\t{status}\t{tests}\n",
                    m.id, m.name, m.file
                ));
                fs::write(
                    self.cache.join(format!("mutation-{:03}-{kind}.log", m.id)),
                    &report,
                )?;
                println!(
                    "mutation {:03}/{} {kind}: {status} — {}",
                    m.id,
                    self.missions.len(),
                    m.name
                );
            }
        }
        fs::write(self.cache.join("mutation-audit.tsv"), evidence)?;
        if !escaped.is_empty() {
            return fail(format!(
                "Behavioral evidence is incomplete:\n{}",
                escaped.join("\n")
            ));
        }
        Ok(())
    }
    fn export(&self, dest: &Path, role: &str) -> Result<()> {
        if dest.exists() {
            return fail("export destination must not exist");
        }
        let repairs = if role == "solutions" {
            self.missions
                .iter()
                .map(|m| self.read_fragment(m, "solutions"))
                .collect::<Result<Vec<_>>>()?
        } else {
            self.prior_repairs(self.missions.len(), "exercises")?
        };
        let files = self.assembled(&repairs)?;
        let (pass, report) = self.evaluate(&files, true)?;
        if !pass {
            return fail(format!("Final implementation failed:\n{report}"));
        }
        fs::create_dir_all(dest)?;
        for (p, c) in files {
            write_changed(&dest.join(p), c.as_bytes())?;
        }
        println!(
            "Reconstructed Rustlings written to {}. The export contains the original project files with your repairs; no grader or test probes are inserted.",
            dest.display()
        );
        Ok(())
    }
}
fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        return fail("usage: grader ROOT prepare|audit|check ID ROLE|export DEST [solutions]");
    }
    let workshop = Workshop::open(PathBuf::from(&args[1]))?;
    match args[2].as_str() {
        "prepare" => {
            let (pass, report) = workshop.evaluate(&workshop.base, true)?;
            if !pass {
                return fail(report);
            }
            println!(
                "Preparation complete. Dependencies and the actual checker regression suite passed. No learner progress was granted."
            );
            Ok(())
        }
        "audit" => workshop.audit(),
        "mutations" => workshop.mutations(
            args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(1),
            args.get(4)
                .map(|s| s.parse())
                .transpose()?
                .unwrap_or(workshop.missions.len()),
        ),
        "check" => {
            let id = args.get(3).ok_or("missing mission")?.parse()?;
            let role = args.get(4).ok_or("missing role")?;
            let mut source = String::new();
            std::io::stdin().read_to_string(&mut source)?;
            workshop.check(id, role, &source, None)
        }
        "check-file" => {
            let id = args.get(3).ok_or("missing mission")?.parse()?;
            let role = args.get(4).ok_or("missing role")?;
            let source = fs::read_to_string(args.get(5).ok_or("missing source path")?)?;
            workshop.check(id, role, &source, Some(Path::new(&args[5])))
        }
        "trace" => workshop.trace(
            args.get(3)
                .ok_or("usage: grader ROOT trace MISSION [DEPTH]")?,
            args.get(4).map(String::as_str).unwrap_or("1"),
        ),
        "export" => workshop.export(
            Path::new(args.get(3).ok_or("missing destination")?),
            args.get(4).map(String::as_str).unwrap_or("exercises"),
        ),
        _ => fail("unknown verifier command"),
    }
}
fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            std::process::ExitCode::from(e.downcast_ref::<GradingError>().map_or(2, |e| e.code))
        }
    }
}

#[cfg(test)]
mod verifier_tests {
    use super::*;
    struct Scratch(PathBuf);
    impl Scratch {
        fn new() -> Self {
            static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let p =
                std::env::temp_dir().join(format!("rustlings-grader-{}-{n}", std::process::id()));
            fs::create_dir(&p).unwrap();
            Self(p)
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn diagnostic_fixture(tmp: &Scratch, source: &str) -> Workshop {
        Workshop {
            root: tmp.0.clone(),
            cache: tmp.0.join("workshop"),
            base: Files::from([
                ("Cargo.toml".into(), "[package]\nname='rustlings'\nversion='0.0.0'\nedition='2024'\n[workspace]\nmembers=['rustlings-macros']\n".into()),
                ("Cargo.lock".into(), "version = 4\n[[package]]\nname = 'rustlings'\nversion = '0.0.0'\n[[package]]\nname = 'rustlings-macros'\nversion = '0.0.0'\n".into()),
                ("src/main.rs".into(), source.into()),
                ("rustlings-macros/Cargo.toml".into(), "[package]\nname='rustlings-macros'\nversion='0.0.0'\nedition='2024'\n".into()),
                ("rustlings-macros/src/lib.rs".into(), "".into()),
            ]),
            probes: Files::new(),
            missions: Vec::new(),
            identity: "diagnostic-fixture".into(),
        }
    }
    #[test]
    fn compiler_errors_and_contract_failures_precede_scaffold_lints() {
        let tmp = Scratch::new();
        fs::create_dir(tmp.0.join("workshop")).unwrap();
        let workshop = diagnostic_fixture(
            &tmp,
            "fn wants_str(_: &str) {} fn main() { wants_str(String::new()); }\n",
        );
        let (pass, report) = workshop
            .evaluate_filtered(&workshop.base, Some("contract"))
            .unwrap();
        assert!(!pass);
        assert!(report.contains("error[E0308]"), "{report}");
        assert!(report.contains("consider borrowing here"), "{report}");
        assert!(!report.contains("cargo clippy"), "{report}");

        let workshop = diagnostic_fixture(
            &tmp,
            "fn main() { let unrelated_lint = 1; }\n#[test] fn contract() { assert_eq!(2, 3, \"counter must advance once\"); }\n",
        );
        let (pass, report) = workshop
            .evaluate_filtered(&workshop.base, Some("contract"))
            .unwrap();
        assert!(!pass);
        assert!(report.contains("counter must advance once"), "{report}");
        assert!(report.contains("left: 2"), "{report}");
        assert!(report.contains("right: 3"), "{report}");
        assert!(!report.contains("cargo clippy"), "{report}");
    }
    #[test]
    fn removes_untracked_build_inputs_and_preserves_expected_files() {
        let tmp = Scratch::new();
        write_changed(&tmp.0.join("src/main.rs"), b"expected").unwrap();
        write_changed(&tmp.0.join("build.rs"), b"stale").unwrap();
        write_changed(&tmp.0.join(".cargo/config.toml"), b"stale").unwrap();
        write_changed(&tmp.0.join("src/old.rs"), b"stale").unwrap();
        let files = Files::from([("src/main.rs".into(), "expected".into())]);
        reconcile(&tmp.0, &files).unwrap();
        assert_eq!(
            fs::read_to_string(tmp.0.join("src/main.rs")).unwrap(),
            "expected"
        );
        assert!(!tmp.0.join("build.rs").exists());
        assert!(!tmp.0.join(".cargo").exists());
        assert!(!tmp.0.join("src/old.rs").exists());
    }
    #[test]
    fn cargo_config_changes_invalidate_the_environment_identity() {
        let tmp = Scratch::new();
        let before = environment_identity(&tmp.0).unwrap();
        write_changed(
            &tmp.0.join(".cargo/config.toml"),
            b"[build]\nrustflags = ['--cfg', 'changed']\n",
        )
        .unwrap();
        let after = environment_identity(&tmp.0).unwrap();
        assert_ne!(before, after);
        assert_eq!(after, environment_identity(&tmp.0).unwrap());
    }
    #[test]
    fn trace_depth_preserves_receipts_but_rustflags_change_identity() {
        const CHILD: &str = "WORKSHOP_ENVIRONMENT_TEST_ROOT";
        if let Some(root) = std::env::var_os(CHILD) {
            println!(
                "ENVIRONMENT_ID={}",
                digest(&environment_identity(Path::new(&root)).unwrap())
            );
            return;
        }
        let tmp = Scratch::new();
        let mut identities = Vec::new();
        for (depth, flags) in [
            ("0", ""),
            ("1", ""),
            ("3", ""),
            ("full", ""),
            ("3", "--cfg changed_build"),
        ] {
            let (status, output) = process::capture(
                Command::new(std::env::current_exe().unwrap())
                    .args(["--exact", "verifier_tests::trace_depth_preserves_receipts_but_rustflags_change_identity", "--nocapture"])
                    .env(CHILD, &tmp.0).env("RUST_BACKTRACE", depth)
                    .env("RUST_LIB_BACKTRACE", depth).env("RUSTFLAGS", flags),
                Duration::from_secs(10),
            ).unwrap();
            assert!(status.success(), "{}", String::from_utf8_lossy(&output));
            identities.push(
                String::from_utf8(output)
                    .unwrap()
                    .lines()
                    .find_map(|l| l.strip_prefix("ENVIRONMENT_ID="))
                    .unwrap()
                    .to_owned(),
            );
        }
        assert!(identities[..4].iter().all(|id| id == &identities[0]));
        assert_ne!(identities[0], identities[4]);
    }
    #[test]
    fn a_persisted_tool_failure_does_not_block_retrying_unchanged_source() {
        let tmp = Scratch::new();
        let workshop = Workshop {
            root: tmp.0.clone(),
            cache: tmp.0.clone(),
            base: Files::new(),
            probes: Files::new(),
            missions: Vec::new(),
            identity: String::new(),
        };
        let input = b"unchanged learner source";
        let key = digest(input);
        let results = tmp.0.join("results");
        write_changed(&results.join(format!("{key}.input")), input).unwrap();
        write_changed(
            &results.join(format!("{key}.report")),
            b"FAIL\nPermission denied (os error 13)",
        )
        .unwrap();
        assert!(workshop.cached(&key, input).unwrap().is_none());
        write_changed(
            &results.join(format!("{key}.report")),
            b"PASS\nverified after tool recovery",
        )
        .unwrap();
        assert!(workshop.cached(&key, input).unwrap().unwrap().0);
        assert!(
            workshop
                .cached(&key, b"different source")
                .unwrap()
                .is_none()
        );
    }
    #[test]
    fn cargo_rebuilds_changed_content_even_when_source_mtime_is_preserved() {
        let tmp = Scratch::new();
        write_changed(
            &tmp.0.join("Cargo.toml"),
            b"[package]\nname='workshop_freshness'\nversion='0.0.0'\nedition='2024'\n[workspace]\n",
        )
        .unwrap();
        let source = tmp.0.join("src/lib.rs");
        let mut timestamp = None;
        for (identity, value) in [("first", "one"), ("second", "two")] {
            let content = format!(
                "#[test] fn changed_value() {{ assert_eq!(\"{value}\", std::env::var(\"EXPECTED_VALUE\").unwrap()); }}\n{}",
                build_identity_probe("fixture")
            );
            write_changed(&source, content.as_bytes()).unwrap();
            if let Some(time) = timestamp {
                File::options()
                    .write(true)
                    .open(&source)
                    .unwrap()
                    .set_times(fs::FileTimes::new().set_modified(time))
                    .unwrap();
            } else {
                timestamp = Some(fs::metadata(&source).unwrap().modified().unwrap());
            }
            let (status, output) = process::capture(
                Command::new("cargo")
                    .args(["test", "--offline", "--", "--nocapture"])
                    .current_dir(&tmp.0)
                    .env("CARGO_TARGET_DIR", tmp.0.join("target"))
                    .env(BUILD_ID_ENV, identity)
                    .env("EXPECTED_VALUE", value),
                Duration::from_secs(60),
            )
            .unwrap();
            assert!(status.success(), "{}", String::from_utf8_lossy(&output));
            assert!(
                String::from_utf8_lossy(&output)
                    .contains(&format!("WORKSHOP_BUILD_ID:fixture:{identity}"))
            );
        }
    }
    #[cfg(unix)]
    #[test]
    fn does_not_follow_generated_tree_symlinks() {
        let tmp = Scratch::new();
        let outside = Scratch::new();
        write_changed(&outside.0.join("keep"), b"untouched").unwrap();
        std::os::unix::fs::symlink(&outside.0, tmp.0.join("src")).unwrap();
        reconcile(&tmp.0, &Files::from([("src/main.rs".into(), "new".into())])).unwrap();
        assert!(!tmp.0.join("src").exists());
        assert_eq!(fs::read(outside.0.join("keep")).unwrap(), b"untouched");
    }
}
