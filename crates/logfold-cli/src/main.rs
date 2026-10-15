use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::Parser;
use logfold_core::{Folder, Pattern};

/// Fold noisy log lines into patterns with counts.
#[derive(Parser)]
#[command(name = "logfold", version, about)]
struct Args {
    /// Log file to read (use '-' or omit for stdin).
    file: Option<PathBuf>,

    /// Show only the top N patterns.
    #[arg(short, long)]
    top: Option<usize>,
}

fn main() -> ExitCode {
    match run(Args::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) if is_broken_pipe(&error) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("logfold: {error:#}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Args) -> Result<()> {
    let path = args.file.as_deref().filter(|path| *path != Path::new("-"));
    let source = path.map_or_else(|| "stdin".to_owned(), |path| path.display().to_string());
    let mut reader: Box<dyn BufRead> = match path {
        Some(path) => {
            let file =
                File::open(path).with_context(|| format!("cannot open {}", path.display()))?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(io::stdin().lock()),
    };

    let mut folder = Folder::new();
    let mut line = String::new();
    let mut line_number = 1;
    loop {
        line.clear();
        let bytes = reader
            .read_line(&mut line)
            .with_context(|| format!("cannot read {source} at line {line_number}"))?;
        if bytes == 0 {
            break;
        }
        folder.add(&line);
        line_number += 1;
    }

    let patterns = folder.finish();
    write_patterns(BufWriter::new(io::stdout().lock()), &patterns, args.top)
        .context("cannot write output")
}

fn write_patterns(
    mut writer: impl Write,
    patterns: &[Pattern],
    top: Option<usize>,
) -> io::Result<()> {
    for pattern in patterns.iter().take(top.unwrap_or(patterns.len())) {
        writeln!(writer, "{:>6}x  {}", pattern.count, pattern.text)?;
    }
    writer.flush()
}

fn is_broken_pipe(error: &anyhow::Error) -> bool {
    error.chain().any(|cause| {
        cause
            .downcast_ref::<io::Error>()
            .is_some_and(|error| error.kind() == io::ErrorKind::BrokenPipe)
    })
}
