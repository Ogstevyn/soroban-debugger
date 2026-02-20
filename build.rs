use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Get git commit hash
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Get rustc version
    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Get build date
    let build_date = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs())
        .unwrap_or(0);

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version);
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);

    // Re-run if git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
}
use clap::CommandFactory;
use std::fs;
use std::path::Path;

// Mock crate root modules that src/cli/args.rs depends on
#[allow(dead_code)]
mod config {
    pub struct Config {
        pub debug: DebugConfig,
        pub output: OutputConfig,
    }
    pub struct DebugConfig {
        pub breakpoints: Vec<String>,
        pub verbosity: Option<u8>,
    }
    pub struct OutputConfig {
        pub format: Option<String>,
        pub show_events: Option<bool>,
    }
}

#[allow(dead_code)]
#[path = "src/cli/args.rs"]
mod args;

use args::Cli;

fn main() -> std::io::Result<()> {
    // Generate man page in the man/man1 directory
    let man_dir = Path::new("man").join("man1");
    fs::create_dir_all(&man_dir)?;

    let cmd = Cli::command();
    render_recursive(&cmd, &man_dir, "")?;

    println!("cargo:rerun-if-changed=src/cli/args.rs");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}

fn render_recursive(cmd: &clap::Command, out_dir: &Path, prefix: &str) -> std::io::Result<()> {
    let name = if prefix.is_empty() {
        cmd.get_name().to_string()
    } else {
        format!("{}-{}", prefix, cmd.get_name())
    };

    let cmd = cmd.clone().name(name.clone());
    let man = clap_mangen::Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    fs::write(out_dir.join(format!("{}.1", name)), buffer)?;

    for sub in cmd.get_subcommands() {
        if !sub.is_hide_set() {
            render_recursive(sub, out_dir, &name)?;
        }
    }

    Ok(())
}
