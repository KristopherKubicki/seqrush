#[cfg(feature = "cli")]
use crate::seqrush::Args;

#[cfg(feature = "cli")]
static mut ARGS_OVERRIDE: Option<Vec<String>> = None;

#[cfg(feature = "cli")]
/// Set command line arguments for testing.
pub fn set_args_override<I, S>(args: I)
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    unsafe {
        ARGS_OVERRIDE = Some(args.into_iter().map(|s| s.into()).collect());
    }
}

#[cfg(feature = "cli")]
/// Clear any argument override for testing.
pub fn clear_args_override() {
    unsafe {
        ARGS_OVERRIDE = None;
    }
}

#[cfg(feature = "cli")]
/// Parse command line arguments into `Args`.
pub fn parse() -> Args {
    let mut sequences = None;
    let mut output = None;
    let mut threads = 1_usize;
    let mut min_match_length = 15_usize;
    let args: Vec<String> = unsafe {
        ARGS_OVERRIDE.clone().unwrap_or_else(|| std::env::args().collect())
    };
    let mut iter = args.into_iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-s" | "--sequences" => sequences = iter.next(),
            "-o" | "--output" => output = iter.next(),
            "-t" | "--threads" => {
                if let Some(t) = iter.next() {
                    if let Ok(v) = t.parse() {
                        threads = v;
                    }
                }
            }
            "-k" | "--min-match-length" => {
                if let Some(m) = iter.next() {
                    if let Ok(v) = m.parse() {
                        min_match_length = v;
                    }
                }
            }
            _ => {}
        }
    }
    let sequences = sequences.expect("input FASTA required");
    let output = output.expect("output file required");
    Args {
        sequences,
        output,
        threads,
        min_match_length,
    }
}

