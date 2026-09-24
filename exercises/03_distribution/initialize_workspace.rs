// RUSTLINGS: REBUILD THE CHECKER | Mission 022/112 | TODO
// Initialize Workspace
// Prerequisite: reveal_solution. Target: src/init.rs::init (upstream line 26).
//
// Contract: Complete one missing initialization stage. Preserve existing-directory checks, workspace detection, catalog materialization, and the generated manifest.
//
// Workflow: read this contract, predict the failure, repair the Rust below,
// then save. Rustlings runs the actual reconstructed implementation and its
// regression probes. Earlier repairs are part of this build; future code is scaffold.
// TODO: Fill the missing implementation step.
// Keep the repair markers, main, and checker intact. Hints: `rustlings hint initialize_workspace`.
// The token wrapper transports this fragment into its original module/impl;
// it is compiled there with the real dependencies, not evaluated as a string.

#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! repair {
    () => {
// BEGIN RUSTLINGS REPAIR
pub fn init() -> Result<()> {
    let rustlings_dir = Path::new("rustlings");
    if rustlings_dir.exists() {
        bail!(RUSTLINGS_DIR_ALREADY_EXISTS_ERR);
    }

    let locate_project_output = Command::new("cargo")
        .arg("locate-project")
        .arg("-q")
        .arg("--workspace")
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .context(
            "Failed to run `cargo locate-project …`\n\
             Did you already install Rust?\n\
             Try running `cargo --version` to diagnose the problem.",
        )?;

    if !Command::new("cargo")
        .arg("clippy")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("Failed to run `cargo clippy --version`")?
        .success()
    {
        bail!(
            "Clippy, the official Rust linter, is missing.\n\
             Please install it first before initializing Rustlings."
        )
    }

    let mut stdout = io::stdout().lock();
    let mut init_git = true;

    if locate_project_output.status.success() {
        if Path::new("exercises").exists() && Path::new("solutions").exists() {
            bail!(IN_INITIALIZED_DIR_ERR);
        }

        let workspace_manifest =
            serde_json::de::from_slice::<CargoLocateProject>(&locate_project_output.stdout)
                .context(
                    "Failed to read the field `root` from the output of `cargo locate-project …`",
                )?
                .root;

        let workspace_manifest_content = fs::read_to_string(workspace_manifest)
            .with_context(|| format!("Failed to read the file {}", workspace_manifest.display()))?;
        if !workspace_manifest_content.contains("[workspace]")
            && !workspace_manifest_content.contains("workspace.")
        {
            bail!(
                "The current directory is already part of a Cargo project.\n\
                 Please initialize Rustlings in a different directory"
            );
        }

        stdout.write_all(b"This command will create the directory `rustlings/` as a member of this Cargo workspace.\n\
                           Press ENTER to continue ")?;
        press_enter_prompt(&mut stdout)?;

        // Make sure "rustlings" is added to `workspace.members` by making
        // Cargo initialize a new project.
        let status = Command::new("cargo")
            .arg("new")
            .arg("-q")
            .arg("--vcs")
            .arg("none")
            .arg("rustlings")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .status()?;
        if !status.success() {
            bail!(
                "Failed to initialize a new Cargo workspace member.\n\
                 Please initialize Rustlings in a different directory"
            );
        }

        stdout.write_all(b"The directory `rustlings` has been added to `workspace.members` in the `Cargo.toml` file of this Cargo workspace.\n")?;
        fs::remove_dir_all(rustlings_dir)
            .context("Failed to remove the temporary directory `rustlings/`")?;
        init_git = false;
    } else {
        stdout.write_all(b"This command will create the directory `rustlings/` which will contain the exercises.\n\
                           Press ENTER to continue ")?;
        press_enter_prompt(&mut stdout)?;
    }

    create_dir(rustlings_dir).context("Failed to create the `rustlings/` directory")?;
    set_current_dir(rustlings_dir)
        .context("Failed to change the current directory to `rustlings/`")?;

    let info_file = InfoFile::parse()?;
    EMBEDDED_FILES
        .init_exercises_dir(&info_file.exercises)
        .context("Failed to initialize the `rustlings/exercises` directory")?;

    create_dir("solutions").context("Failed to create the `solutions/` directory")?;
    fs::write(
        "solutions/README.md",
        include_bytes!("../solutions/README.md"),
    )
    .context("Failed to create the file rustlings/solutions/README.md")?;
    for dir in EMBEDDED_FILES.exercise_dirs {
        let mut dir_path = String::with_capacity(10 + dir.name.len());
        dir_path.push_str("solutions/");
        dir_path.push_str(dir.name);
        create_dir(&dir_path)
            .with_context(|| format!("Failed to create the directory {dir_path}"))?;
    }
    for exercise_info in &info_file.exercises {
        let solution_path = exercise_info.sol_path();
        fs::write(&solution_path, INIT_SOLUTION_FILE)
            .with_context(|| format!("Failed to create the file {solution_path}"))?;
    }


    // TODO: Restore this step using the contract above.
    // TODO: Complete the missing operation.
    let newline_ind = current_cargo_toml
        .as_bytes()
        .iter()
        .position(|c| *c == b'\n')
        .context("The embedded `Cargo.toml` is empty or contains only one line")?;
    let current_cargo_toml = current_cargo_toml
        .get(newline_ind + 1..)
        .context("The embedded `Cargo.toml` contains only one line")?;
    let updated_cargo_toml = updated_cargo_toml(&info_file.exercises, current_cargo_toml, b"")
        .context("Failed to generate `Cargo.toml`")?;
    fs::write("Cargo.toml", updated_cargo_toml)
        .context("Failed to create the file `rustlings/Cargo.toml`")?;

    fs::write("rust-analyzer.toml", RUST_ANALYZER_TOML)
        .context("Failed to create the file `rustlings/rust-analyzer.toml`")?;

    fs::write(".gitignore", GITIGNORE)
        .context("Failed to create the file `rustlings/.gitignore`")?;

    fs::write("README.md", README).context("Failed to create the file `rustlings/README.md`")?;

    create_dir(".vscode").context("Failed to create the directory `rustlings/.vscode`")?;
    fs::write(".vscode/extensions.json", VS_CODE_EXTENSIONS_JSON)
        .context("Failed to create the file `rustlings/.vscode/extensions.json`")?;

    if init_git && let Ok(dir) = current_dir() {
        let mut dir = dir.as_path();

        loop {
            if dir.join(".git").exists() || dir.join(".jj").exists() {
                break;
            }

            if let Some(parent) = dir.parent() {
                dir = parent;
            } else {
                // Ignore any Git error because Git initialization is not required.
                let _ = Command::new("git")
                    .arg("init")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
                break;
            }
        }
    }

    stdout.queue(SetForegroundColor(Color::Green))?;
    stdout.write_all("Initialization done ✓".as_bytes())?;
    stdout.queue(ResetColor)?;
    stdout.write_all(b"\n\n")?;

    stdout.queue(SetAttribute(Attribute::Bold))?;
    stdout.write_all(POST_INIT_MSG)?;
    stdout.queue(ResetColor)?;

    Ok(())
}
// END RUSTLINGS REPAIR
    };
}

#[path = "../../exercises/01_catalog/runner.rs"]
mod workshop;

fn main() {
    workshop::run(22, include_str!("initialize_workspace.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn mission_contract() {
        super::workshop::run(22, include_str!("initialize_workspace.rs"));
    }
}
