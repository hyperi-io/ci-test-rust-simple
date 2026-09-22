//! Project:      ci-test-rust-simple
//! File:         src/main.rs
//! Purpose:      Minimal CLI entry point with --version and --help support
//! Language:     Rust
//!
//! License:      FSL-1.1-ALv2
//! Copyright:    (c) 2026 HYPERI PTY LIMITED

// DO NOT FIX - see the DO NOT FIX block in Cargo.toml. The vulnerable
// smallvec is used here so it is a real dependency of a real code path,
// not dead weight an unused-dependency check could flag at a different
// stage than the advisory gate this case is asserting.
use smallvec::SmallVec;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Collect the CLI arguments without heap-allocating the common short case.
fn collect_args() -> SmallVec<[String; 4]> {
    std::env::args().collect()
}

fn main() {
    let args = collect_args();

    if args.len() > 1 {
        match args[1].as_str() {
            "--version" | "-V" => {
                println!("{NAME} {VERSION}");
            }
            "--help" | "-h" => {
                println!("Usage: {NAME} [OPTIONS]");
                println!();
                println!("A minimal Rust CLI for testing HyperI CI pipelines.");
                println!();
                println!("Options:");
                println!("  -h, --help       Print help information");
                println!("  -V, --version    Print version information");
            }
            other => {
                eprintln!("error: unknown option '{other}'");
                eprintln!("Usage: {NAME} [OPTIONS]");
                std::process::exit(1);
            }
        }
    } else {
        println!("Hello from {NAME}!");
    }
}
