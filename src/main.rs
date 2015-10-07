use std::process::Command;
use std::env;
use std::path::Path;
use walker::Walker;

extern crate walker;

fn fmt(path: &Path, cwd: &Path) {
    Command::new("rustfmt")
        .arg("--write-mode=overwrite")
        .arg(path)
        .current_dir(cwd)
        .status()
        .unwrap_or_else(|e| panic!("failed to execute rustfmt: {:#?}", e));
}

fn main() {
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
