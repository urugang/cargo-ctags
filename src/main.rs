extern crate serde_json;

use std::env;
use std::process;
use serde_json::Value;

fn main() {
    let mut metadata = process::Command::new("cargo");
    metadata.arg("metadata");
    let mut args = env::args().skip(2);
    let mut dash = false;

    while let Some(item) = args.next() {
        if item == "--" {
            dash = true;
            break;
        }
        metadata.arg(item);
    }

    let output = match metadata.output() {
        Ok(output) => output,
        Err(_) => {
            println!("Failed to run 'cargo metadata'");
            process::exit(1);
        }
    };
    if !output.status.success() {
        println!("Failed to run 'cargo metadata'");
        process::exit(1);
    }

    let mut ctags = process::Command::new("ctags");
    ctags.arg("-R");
    while let Some(item) = args.next() {
        if item == "--" {
            dash = true;
        }
        if !dash {
            continue;
        }
        ctags.arg(item);
    }

    if let Ok(rust_src_str) = env::var("RUST_SRC_PATH") {
        ctags.arg(rust_src_str);
    }

    let metadata = std::str::from_utf8(&output.stdout).unwrap();
    let metadata: Value = serde_json::from_str(&metadata).unwrap();
    let packages = metadata.get("packages").unwrap().as_array().unwrap();
    for p in packages {
        if let Some(manifest) = p.get("manifest_path") {
            if let Some(path) = manifest.as_str() {
                if let Some(path) = std::path::Path::new(path).parent() {
                    ctags.arg(path);
                }
            }
        };
    }
    if let Ok(output) = ctags.output() {
        if output.status.success() {
            process::exit(0);
        }
    }
    println!("failed to execute: {:?} ", ctags);
    process::exit(1);
}
