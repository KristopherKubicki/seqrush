pub mod seqrush {
    use std::fs::File;
    use std::io::{self, BufRead, Write};

    #[derive(Debug, Clone)]
    pub struct Args {
        /// Path to input FASTA file
        pub sequences: String,
        /// Path to output GFA file
        pub output: String,
        /// Number of worker threads (unused in this stub)
        pub threads: usize,
        /// Minimum match length (unused)
        pub min_match_length: usize,
    }

    /// Sequence record parsed from a FASTA file.
    ///
    /// Contains the identifier and raw sequence bytes for a single entry.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FastaSequence {
        /// Identifier without the leading `>` character
        pub id: String,
        /// Raw nucleotide or amino acid data
        pub data: Vec<u8>,
    }

    /// Read sequences from a FASTA file.
    ///
    /// The `path` argument should point to a plaintext FASTA file. Each record
    /// is parsed into a [`FastaSequence`] with its identifier and raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if the file cannot be read.
    pub fn load_sequences(path: &str) -> io::Result<Vec<FastaSequence>> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let mut sequences = Vec::new();
        let mut id = None;
        let mut data: Vec<u8> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.starts_with('>') {
                if let Some(id_val) = id.take() {
                    sequences.push(FastaSequence {
                        id: id_val,
                        data: data.clone(),
                    });
                    data.clear();
                }
                id = Some(line[1..].to_string());
            } else {
                data.extend(line.trim().as_bytes());
            }
        }
        if let Some(id_val) = id {
            sequences.push(FastaSequence { id: id_val, data });
        }
        Ok(sequences)
    }

    /// Generate a minimal GFA representation for the given sequences.
    ///
    /// The provided [`Args`] struct supplies the input FASTA path and the
    /// destination for the GFA output. Sequences are loaded with
    /// [`load_sequences`] and written as segments and paths.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if reading the FASTA file or writing the GFA
    /// file fails.
    pub fn run_seqrush(args: Args) -> io::Result<()> {
        let sequences = load_sequences(&args.sequences)?;
        let mut file = File::create(&args.output)?;
        writeln!(file, "H\tVN:Z:1.0")?;
        for seq in &sequences {
            let seq_str = String::from_utf8_lossy(&seq.data);
            writeln!(file, "S\t{}\t{}", seq.id, seq_str)?;
        }
        if sequences.is_empty() {
            return Ok(());
        }

        let path_line = sequences
            .iter()
            .map(|s| format!("{}+", s.id))
            .collect::<Vec<_>>()
            .join(",");
        writeln!(file, "P\tp1\t{}\t*", path_line)?;

        for window in sequences.windows(2) {
            writeln!(file, "L\t{}\t+\t{}\t+\t0M", window[0].id, window[1].id)?;
        }

        Ok(())
    }
}

pub use seqrush::{load_sequences, run_seqrush, Args, FastaSequence};

