use proconio::input;
use std::io::{prelude::*, BufWriter};

type TResult<T> = Result<T, Box<dyn std::error::Error>>;

fn run(mut stdout: impl Write) -> TResult<()> {
    // Write here
    // writeln!(stdout, "{}", s)?;

    input! {
        n: i64,
        y: i64,
    }

    let mut result: (i64, i64, i64) = (-1, -1, -1);

    'lp: for c10k in 0..=n {
        for c5k in 0..=(n - c10k) {
            let c1k = n - c10k - c5k;

            if c10k + c5k + c1k != n {
                continue;
            }
            if y == (c10k * 10000) + (c5k * 5000) + (c1k * 1000) {
                result = (c10k, c5k, c1k);
                break 'lp;
            }
        }
    }

    let (c10k, c5k, c1k) = result;
    writeln!(stdout, "{} {} {}", c10k, c5k, c1k)?;

    Ok(())
}

fn main() -> TResult<()> {
    let stdout = std::io::stdout();
    let stdout = BufWriter::new(stdout.lock());

    run(stdout)
}
