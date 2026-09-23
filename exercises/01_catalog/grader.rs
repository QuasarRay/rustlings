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
            "FAIL" => Ok(Some((false, output.into()))),
            _ => Ok(None),
        }
    }
    fn evaluate(&self, files: &Files, _long: bool) -> Result<(bool, String)> {
        let input = serialize(files, &self.identity);
        let key = digest(&input);
        if let Some(report) = self.cached(&key, &input)? {
            return Ok(report);
        }
        // One budget owns queueing, Clippy, and tests together. The course host
        // grants 600 seconds, leaving 60 seconds for startup and cleanup.
        let deadline = Instant::now() + Duration::from_secs(540);
        let _lock = self.lock(deadline)?;
        if let Some(report) = self.cached(&key, &input)? {
            return Ok(report);
        }
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
            write_changed(&build.join(path), text.as_bytes())?;
        }
        let target = self.cache.join("engine-target");
        let mut report = String::new();
        let mut passed = true;
        let commands: &[&[&str]] = &[
            &[
                "clippy",
                "--locked",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
            &["test", "--locked", "--workspace", "--", "--test-threads=1"],
        ];
        for args in commands {
            let mut cmd = Command::new("cargo");
            cmd.args(*args)
                .current_dir(&build)
                .env("CARGO_TARGET_DIR", &target)
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
            if !status.success() {
                passed = false;
                break;
            }
        }
        let dir = self.cache.join("results");
        fs::create_dir_all(&dir)?;
        // The full input is checked as well as its hash; hash collisions cannot grant a pass.
        fs::write(dir.join(format!("{key}.input")), input)?;
        fs::write(
            dir.join(format!("{key}.report")),
            format!("{}\n{report}", if passed { "PASS" } else { "FAIL" }),
        )?;
        Ok((passed, report))
    }
    fn check(&self, id: usize, role: &str, source: &str) -> Result<()> {
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
        let (pass, report) = self.evaluate(&files, false)?;
        if !pass {
            return outcome(
                1,
                format!(
                    "REJECTED: mission {:03} {}.\nEdit exercises/{}/{}.rs (restores {}), not target/workshop/engine.\n{}",
                    id, m.name, m.dir, m.name, m.file, report
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
        for m in &self.missions {
            let mut repairs = baseline.clone();
            repairs[m.id - 1] = self.read_fragment(m, "exercises")?;
            if repairs[m.id - 1] == baseline[m.id - 1] {
                return fail(format!("{} is already solved", m.name));
            }
            let (pass, _) = self.evaluate(&self.assembled(&repairs)?, true)?;
            if pass {
                return fail(format!(
                    "Injected defect in {} escaped compilation and behavioral checks",
                    m.name
                ));
            }
            println!(
                "{:03}/{} verified: starter FAIL, reference PASS — {}",
                m.id,
                self.missions.len(),
                m.name
            );
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
        "check" => {
            let id = args.get(3).ok_or("missing mission")?.parse()?;
            let role = args.get(4).ok_or("missing role")?;
            let mut source = String::new();
            std::io::stdin().read_to_string(&mut source)?;
            workshop.check(id, role, &source)
        }
        "check-file" => {
            let id = args.get(3).ok_or("missing mission")?.parse()?;
            let role = args.get(4).ok_or("missing role")?;
            let source = fs::read_to_string(args.get(5).ok_or("missing source path")?)?;
            workshop.check(id, role, &source)
        }
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
