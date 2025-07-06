use seqrush::{Args, run_seqrush, load_sequences};
use std::fs::{self, File};
use std::io::Write;
use std::env::temp_dir;

fn temp_file(name: &str) -> std::path::PathBuf {
    let mut path = temp_dir();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros();
    path.push(format!("{}_{}", name, now));
    path
}

#[test]
fn load_sequences_parses_fasta() {
    let path = temp_file("seqs");
    let mut f = File::create(&path).unwrap();
    writeln!(f, ">a\nACGT\n>b\nTTTT").unwrap();
    f.sync_all().unwrap();
    let seqs = load_sequences(path.to_str().unwrap()).unwrap();
    assert_eq!(seqs.len(), 2);
    assert_eq!(seqs[0].id, "a");
    fs::remove_file(path).unwrap();
}

#[test]
fn run_seqrush_writes_output() {
    let in_path = temp_file("in");
    let mut f = File::create(&in_path).unwrap();
    writeln!(f, ">x\nAAAA\n>y\nGGGG").unwrap();
    f.sync_all().unwrap();
    let out_path = temp_file("out");
    let args = Args {
        sequences: in_path.to_str().unwrap().to_string(),
        output: out_path.to_str().unwrap().to_string(),
        threads: 1,
        min_match_length: 1,
    };
    run_seqrush(args).unwrap();
    let content = fs::read_to_string(&out_path).unwrap();
    assert!(content.starts_with("H\tVN:Z:1.0"));
    fs::remove_file(&in_path).unwrap();
    fs::remove_file(&out_path).unwrap();
}
#[test]
fn load_sequences_empty_input() {
    let path = temp_file("empty");
    File::create(&path).unwrap();
    let seqs = load_sequences(path.to_str().unwrap()).unwrap();
    assert!(seqs.is_empty());
}

#[test]
fn load_sequences_missing_file() {
    let path = temp_file("missing");
    let result = load_sequences(path.to_str().unwrap());
    assert!(result.is_err());
}

#[test]
fn run_seqrush_missing_input() {
    let out_path = temp_file("out_missing");
    let args = Args {
        sequences: temp_file("noexist").to_str().unwrap().to_string(),
        output: out_path.to_str().unwrap().to_string(),
        threads: 1,
        min_match_length: 1,
    };
    let result = run_seqrush(args);
    assert!(result.is_err());
}

use std::process::Command;

#[test]
fn cli_no_arguments() {
    let exe = env!("CARGO_BIN_EXE_seqrush");
    let output = Command::new(exe).output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("input FASTA required"));
}

#[cfg(not(feature = "cli"))]
#[test]
fn cli_missing_output() {
    let exe = env!("CARGO_BIN_EXE_seqrush");
    let output = Command::new(exe)
        .arg("somefile")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("output file required"));
}

#[cfg(feature = "cli")]
#[test]
fn cli_parses_flags() {
    use std::process::Command;
    let in_path = temp_file("cli_in");
    let mut f = File::create(&in_path).unwrap();
    writeln!(f, ">z\nAAAA").unwrap();
    f.sync_all().unwrap();
    let out_path = temp_file("cli_out");
    let status = Command::new(env!("CARGO_BIN_EXE_seqrush"))
        .args([
            "-s",
            in_path.to_str().unwrap(),
            "-o",
            out_path.to_str().unwrap(),
            "-t",
            "2",
            "-k",
            "5",
        ])
        .status()
        .unwrap();
    assert!(status.success());
}
