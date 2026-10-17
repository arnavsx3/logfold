# logfold

[![CI](https://github.com/arnavsx3/logfold/actions/workflows/ci.yml/badge.svg)](https://github.com/arnavsx3/logfold/actions/workflows/ci.yml)

Fold repetitive logs into counted patterns. A small Rust CLI that replaces changing
values, groups matching lines, and puts the most frequent patterns first.

## Try it

With stable Rust installed, run from the repository root:

```sh
cargo run --release -- samples/app.log
```

```text
     3x  user <NUM> logged in from <IP>
     2x  timeout after <NUM>s on db-<NUM>
     1x  request <UUID> finished in <NUM>ms
```

## Install and use

```sh
cargo install --path crates/logfold-cli --locked

logfold samples/app.log
logfold samples/app.log --top 2
cat samples/app.log | logfold
logfold - < samples/app.log
```

`logfold [FILE] [-t N | --top N]`

Omit `FILE` or use `-` to read stdin. `--top` limits the number of printed patterns;
`--top 0` prints none. Use `--help` for usage and `--version` for the version.

## How matching works

| Variable value | Example | Replacement |
| --- | --- | --- |
| ISO-style timestamp | `2026-10-15T12:30:01.123Z` | `<TIMESTAMP>` |
| UUID | `123e4567-e89b-12d3-a456-426614174000` | `<UUID>` |
| IPv4 address | `192.168.1.20` | `<IP>` |
| Hexadecimal value | `0xDEADBEEF` | `<HEX>` |
| Integer, decimal, or exponent | `42`, `-1.5`, `2.4e-3` | `<NUM>` |
| Number with a unit | `120ms` | `<NUM>ms` |

Timestamps support a `T` or space separator, optional fractional seconds, and
optional `Z`, `+05:30`, or `+0530` offsets. Addresses are checked for valid IPv4
octets. Digits inside words stay intact (`api-v1` and `api-v2` remain separate);
numeric suffixes after punctuation are masked (`db-1` becomes `db-<NUM>`).

Decimals without a leading zero are supported, including signed values and
exponents. For example, `latency=.5ms`, `latency=-.5ms`, and `latency=.5e-3ms`
all fold into `latency=<NUM>ms`.

Whitespace is trimmed and collapsed to single spaces. Blank lines are skipped.
Patterns are sorted by descending count, then by text to make ties repeatable.

Matching uses fixed rules. Words, punctuation, and letter case remain significant.
Numeric status codes are masked too, so `status=200` and `status=500` match.
IPv6, arbitrary string IDs, and multiline stack traces have no dedicated rules.

Input must be UTF-8. Results print after the input ends, so use a finite file or
pipe. Input is read one line at a time; memory grows with the number of distinct
patterns. Read and write failures return a nonzero exit code, while a closed
output pipe exits quietly.

## Project layout

```text
crates/
  logfold-core/   # normalization and pattern counting; usable as a library
  logfold-cli/    # arguments, input, and output
samples/
  app.log        # runnable example
```

The core library exposes `mask_line`, `Folder`, and `Pattern`. See its crate docs
for a short usage example (`cargo doc --workspace --no-deps --open`).

## Development

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --release --locked --bin logfold
```

CI runs tests and release builds on Linux, macOS, and Windows, with formatting and
lint checks on Linux. The workspace uses stable Rust and keeps `Cargo.lock` checked
in for reproducible dependency versions.
