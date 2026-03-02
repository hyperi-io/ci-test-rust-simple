//! Project:      ci-test-rust-simple
//! File:         src/main.rs
//! Purpose:      Minimal CLI entry point with --version and --help support
//! Language:     Rust
//!
//! License:      FSL-1.1-ALv2
//! Copyright:    (c) 2026 HYPERI PTY LIMITED

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = std::env::args().collect();

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
