use proconio::input;
use std::io::{prelude::*, BufWriter};

type TResult<T> = Result<T, Box<dyn std::error::Error>>;

fn run(mut stdout: impl Write) -> TResult<()> {
    // Write here
    // writeln!(stdout, "{}", s)?;

    input! {
        n: u64,
        mut d: [u64; n]
    }

    d.sort();
    d.dedup();
    writeln!(stdout, "{}", d.len())?;

    Ok(())
}

fn main() -> TResult<()> {
    let stdout = std::io::stdout();
    let stdout = BufWriter::new(stdout.lock());

    run(stdout)
}
