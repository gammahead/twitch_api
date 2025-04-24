use std::path::{Path, PathBuf};

use clap::Parser;
use once_cell::sync::OnceCell;
use serde_derive::Deserialize;
use xshell::{cmd, Shell};

static RUSTDOCFLAGS: &[&str] = &["--cfg", "nightly"];
static RUSTFLAGS: &[&str] = &["--cfg", "nightly"];
static TWITCH_OAUTH2_FEATURES: &str = "all mock_api";

#[derive(Debug, Parser)]
pub enum Args {
    Doc {
        /// Set the target dir, this will by default be a subdirectory inside `target` to
        /// save on compilation, as the rust flags will be changed, thus needing a new compilation
        #[clap(long, default_value = "target/extra")]
        target_dir: String,
        #[clap(last = true)]
        last: Option<String>,
    },
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let sh = Shell::new()?;

    let args = Args::parse();

    match args {
        Args::Doc { target_dir, last } => {
            let _rustdocflags =
                sh.push_env("CARGO_ENCODED_RUSTDOCFLAGS", RUSTDOCFLAGS.join("\u{1f}"));
            let _rustflags = sh.push_env("CARGO_ENCODED_RUSTFLAGS", RUSTFLAGS.join("\u{1f}"));
            if !cargo_ver(&sh)?.contains("nightly") {
                color_eyre::eyre::bail!("Not running with a nightly cargo, use `cargo +nightly`");
            }
            cmd!(
                sh,
                "cargo doc --target-dir {target_dir} --no-deps --features {TWITCH_OAUTH2_FEATURES} -Zunstable-options -Zrustdoc-scrape-examples {last...}"
            )
            .run()?;
        }
    }
    Ok(())
}

fn cargo_ver(sh: &Shell) -> Result<String, color_eyre::Report> {
    cmd!(sh, "cargo -V").read().map_err(Into::into)
}

#[track_caller]
fn pkgid() -> Result<String, color_eyre::Report> {
    let sh = xshell::Shell::new()?;
    sh.change_dir(get_cargo_workspace());
    cmd!(sh, "cargo pkgid")
        .read()
        .map(|s| s.trim().to_owned())
        .map_err(Into::into)
}

/// Returns the cargo workspace for the manifest
pub fn get_cargo_workspace() -> &'static Path {
    static WORKSPACE: OnceCell<PathBuf> = OnceCell::new();
    #[derive(Debug, Deserialize)]
    pub struct CargoMetadata {
        pub workspace_root: PathBuf,
    }
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    WORKSPACE.get_or_init(|| {
        let sh = xshell::Shell::new().unwrap();
        sh.change_dir(manifest_dir);
        cmd!(sh, "cargo metadata --format-version 1 --no-deps")
            .read()
            .map_err(color_eyre::Report::from)
            .and_then(|s| serde_json::from_str::<CargoMetadata>(&s).map_err(Into::into))
            .unwrap()
            .workspace_root
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_pkgid_hashtag() {
        let pkgid = dbg!(pkgid().unwrap());
        assert!(!pkgid.contains('@'));
        assert!(pkgid.contains("twitch_oauth2"));
    }

    #[test]
    pub fn assert_msrv() {
        let workspace = get_cargo_workspace();
        let toml = std::fs::read_to_string(workspace.join("Cargo.toml")).unwrap();
        let msrv = toml
            .split("rust-version = \"")
            .nth(1)
            .unwrap()
            .split('"')
            .next()
            .unwrap();
        dbg!(msrv);
        let read_dir = std::fs::read_dir(workspace.join(".github/workflows")).unwrap();
        for workflow in read_dir {
            let workflow = workflow.unwrap();
            let path = workflow.path();
            if path.extension() == Some(std::ffi::OsStr::new("yml")) {
                let content = std::fs::read_to_string(&path).unwrap();
                if content.contains("MSRV:") {
                    println!("check {}", path.display());
                    assert!(content.contains(&format!("MSRV: {msrv}")));
                }
            }
        }
    }
}
