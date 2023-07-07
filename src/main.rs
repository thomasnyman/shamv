/*
 * Copyright 2023 Thomas Nyman <thomas.nyman@iki.fi>
 * SPDX-License-Identifier: Apache-2.0 or MIT
 */
extern crate getopts;
extern crate digest;
extern crate sha2;
extern crate hex;

use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path,PathBuf};
use getopts::Options;
use digest::DynDigest;

enum Mode {
    Rename,
    DryRun,
}

#[derive(Copy, Clone)]
enum Status {
    Success = 0,
    InsufficientArgs,
    UnsupportedAlg,
    FileNotFound,
    DigestError,
}

fn print_usage() {
    print!(r#"
Usage: {0} [OPTION...] FILE[...]
The {0} utility renames the file named by the FILE operand to a destination path that is formed
from the SHA-2 hash of the content of the file.

If the FILE operand includes a filename extension that consists of one or more suffixes, each
separated by a dot (.) character, the destination path is formed by the concatenation of the SHA-2
hash and the filename extension suffixes of the original FILE name.

Mandatory arguments to long options are mandatory for short options too.
 -a, --algorithm      The SHA-2 algorithm to use: sha224, sha256 (default), or sha512.
 -n, --dry-run        Display the current and new filenames but do not perform the rename.
 -h, --help           Print this help and exit.
 -V, --version        Print the version of the program and exit."#,
    env!("CARGO_BIN_NAME"));
}

fn print_version() {
    println!("{} version {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
    std::process::exit(0);
}

fn get_digest_alg(s: &str) -> Option<Box<dyn DynDigest>>{
    match s {
              "" => Some(Box::new(sha2::Sha256::default())),  // Default digest algorithm
        "sha224" => Some(Box::new(sha2::Sha224::default())),
        "sha256" => Some(Box::new(sha2::Sha256::default())),
        "sha512" => Some(Box::new(sha2::Sha512::default())),
               _ => None,  // Unsupported digest
    }
}

fn calculate_digest(alg: &mut dyn DynDigest, path: &Path) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    alg.update(&buffer);
    let d = alg.finalize_reset();
    Ok(hex::encode(d))
}

fn main() {
    // Read the command-line arguments
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("a", "algorithm", "The SHA-2 algorithm to use: sha224, sha256 (default), sha384, or sha512", "sha256");
    opts.optflag("n", "dry-run", "Display the original and new filenames but do not perform the rename");
    opts.optflag("h", "help", "Print this help and exit");
    opts.optflag("V", "version", "Print the version of the program and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => { panic!("{}", e.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage();
        std::process::exit(Status::Success as i32);
    }

    if matches.opt_present("V") {
        print_version();
        std::process::exit(Status::Success as i32);
    }

    let mut boxed_alg = if let Some(a) = get_digest_alg(&matches.opt_str("a").unwrap_or_default()) {
        a
    } else {
        eprintln!("{0}: unsupported algorithm {1}",
            env!("CARGO_BIN_NAME"),
            &matches.opt_str("a").unwrap());
        std::process::exit(Status::UnsupportedAlg as i32);
    };

    let mode = if matches.opt_present("n") {
        Mode::DryRun
    } else {
        Mode::Rename
    };

    let pathnames: Vec<String> = if ! matches.free.is_empty() {
        matches.free
    } else {
        eprintln!("{}: must specify at least one file", env!("CARGO_BIN_NAME"));
        print_usage();
        std::process::exit(Status::InsufficientArgs as i32);
    };

    let mut paths: Vec<PathBuf> = Vec::with_capacity(pathnames.len());
    let mut digests: Vec<String> = Vec::with_capacity(pathnames.len());

    for p in pathnames {
        let path = Path::new(&p);
        if !path.exists() {
            eprintln!("{0}: file not found {1}", env!("CARGO_BIN_NAME"), p);
            std::process::exit(Status::FileNotFound as i32);
        }
        paths.push(path.to_path_buf());
    }

    for p in paths.as_slice() {
        match calculate_digest(&mut *boxed_alg, &p) {
            Ok(d) => digests.push(d),
            Err(err) => {
                eprintln!("{0}: error calculating digest for: {1:?}: {2}",
                    env!("CARGO_BIN_NAME"),
                    p.as_path().file_name().unwrap(),
                    err);
                std::process::exit(Status::DigestError as i32);
            }
        };
    }

    for i in paths.iter().zip(digests.iter()) {
        let (path, digest) = i;

        let new_file_name = if let Some(ext) = path.extension() {
            format!("{}.{}", digest, ext.to_string_lossy())
        } else {
            format!("{}", digest)
        };

        let new_path = path.with_file_name(new_file_name);

        match mode {
            Mode::DryRun => { println!("{:?} → {:?}", path, new_path); },
            Mode::Rename => { if let Err(err) = fs::rename(&path, &new_path) {
                    eprintln!("{0}: error renaming file {1:?}: {2}",
                        env!("CARGO_BIN_NAME"),
                        path.as_path().file_name().unwrap(),
                        err);
                }
            },
        }
    }
}
