use seqrush::seqrush::Args;

#[cfg(feature = "cli")]
#[test]
fn parse_handles_arguments() {
    // set dummy exe path for cargo to satisfy macros if required
    std::env::set_var("CARGO_BIN_EXE_seqrush", "dummy");
    let args = [
        "binary",
        "-s",
        "input.fa",
        "-o",
        "out.gfa",
        "-t",
        "4",
        "-k",
        "20",
    ];
    seqrush::cli::set_args_override(args.iter().map(|s| s.to_string()));
    let parsed = seqrush::cli::parse();
    seqrush::cli::clear_args_override();
    assert_eq!(parsed.sequences, "input.fa");
    assert_eq!(parsed.output, "out.gfa");
    assert_eq!(parsed.threads, 4);
    assert_eq!(parsed.min_match_length, 20);
}
