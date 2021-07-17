use proconio::input;
use std::io::{prelude::*, BufWriter};

type TResult<T> = Result<T, Box<dyn std::error::Error>>;

fn run(mut stdout: impl Write) -> TResult<()> {
    // Write here
    // writeln!(stdout, "{}", s)?;

    input! {
        a: u64, b: u64
    }

    let result = if (a * b) % 2 == 0 { "Even" } else { "Odd" };

    writeln!(stdout, "{}", result)?;

    Ok(())
}

fn main() -> TResult<()> {
    let stdout = std::io::stdout();
    let stdout = BufWriter::new(stdout.lock());

    run(stdout)
}
