use proconio::input;
use std::io::{prelude::*, BufWriter};
type TResult<T> = Result<T, Box<dyn std::error::Error>>;

fn run(mut stdout: impl Write) -> TResult<()> {
    fn calculate(x500: u64, x100: u64, x50: u64) -> u64 {
        const COIN_500: u64 = 500;
        const COIN_100: u64 = 100;
        const COIN_50: u64 = 50;

        (x500 * COIN_500) + (x100 * COIN_100) + (x50 * COIN_50)
    }

    input! {
        a: u64,
        b: u64,
        c: u64,
        sum: u64,
    }
    let mut count = 0;

    for x500 in 0u64..=a {
        for x100 in 0u64..=b {
            for x50 in 0u64..=c {
                if calculate(x500, x100, x50) == sum {
                    count += 1;
                }
            }
        }
    }

    writeln!(stdout, "{}", count)?;

    Ok(())
}

fn main() -> TResult<()> {
    let stdout = std::io::stdout();
    let stdout = BufWriter::new(stdout.lock());

    run(stdout)
}
