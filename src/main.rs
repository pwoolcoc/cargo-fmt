use std::process::Command;
use std::env;
use std::path::Path;
use walker::Walker;
use docopt::Docopt;

extern crate walker;
extern crate docopt;

fn fmt(path: &Path, cwd: &Path) {
    Command::new("rustfmt")
        .arg("--write-mode=overwrite")
        .arg(path)
        .current_dir(cwd)
        .status()
        .unwrap_or_else(|e| panic!("failed to execute rustfmt: {:#?}", e));
}

const USAGE: &'static str = "
Format all Rust source files in the project with rustfmt

Usage:
    cargo fmt [options]

Options:
    -h, --help  Print this message

The current implementation formats all .rs files in
the current directory, recursively.
";

fn main() {
    let opts = Docopt::new(USAGE).unwrap();
    opts.parse().unwrap_or_else(|e| e.exit());
    let cwd = env::current_dir().unwrap();
    match Walker::new(&cwd) {
        Ok(iter) => {
            for path in iter {
                let path = path.unwrap().path();
                if let Some(ext) = path.extension() {
                    if ext == "rs" {
                        fmt(&path, &cwd);
                    }
                }
            }
        }
        Err(e) => panic!(e),
    }
}
