//! The executable that probes database schemas out-of-process for the CrawRAD IDE.
//!
//! A console program on purpose: the IDE starts it through
//! `crawrad_os::process::no_console`, so no window opens, and run by hand it
//! prints its answer.

#![forbid(unsafe_code)]

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (code, output) = crawrad_dbprobe::answer(&args);
    println!("{output}");
    std::process::exit(code);
}
