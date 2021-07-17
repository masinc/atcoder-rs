use proconio::input;
use std::io::{prelude::*, BufWriter};

type TResult<T> = Result<T, Box<dyn std::error::Error>>;

fn run(mut stdout: impl Write) -> TResult<()> {
    // Write here
    // writeln!(stdout, "{}", s)?;

    input! {
        n: u64,
        a: u64,
        x: u64,
        y: u64,
    }

    let result: u64 = (0..n).map(|index| if index < a { x } else { y }).sum();
    writeln!(stdout, "{}", result)?;

    Ok(())
}

fn main() -> TResult<()> {
    let stdout = std::io::stdout();
    let stdout = BufWriter::new(stdout.lock());

    run(stdout)
}
