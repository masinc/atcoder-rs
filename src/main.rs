use proconio::input;
use std::io::{prelude::*, BufWriter};

type TResult<T> = Result<T, Box<dyn std::error::Error>>;

fn run(mut stdout: impl Write) -> TResult<()> {
    // Write here
    // writeln!(stdout, "{}", s)?;

    input! {
        n: u64,
        a: [u64; n]
    }

    let mut a = a.to_vec();
    let mut result: u64 = 0;

    while a.iter().all(|x| x % 2 == 0) {
        result += 1;
        a = a.into_iter().map(|x| x / 2).collect();
    }

    writeln!(stdout, "{}", result)?;

    Ok(())
}

fn main() -> TResult<()> {
    let stdout = std::io::stdout();
    let stdout = BufWriter::new(stdout.lock());

    run(stdout)
}
