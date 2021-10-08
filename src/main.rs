use proconio::input;
use std::io::{prelude::*, BufWriter};

type TResult<T> = Result<T, Box<dyn std::error::Error>>;

fn get_digits(n: u64) -> TResult<Vec<u64>> {
    let result = n
        .to_string()
        .chars()
        .map(|x| x.to_string().parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()?;

    Ok(result)
}

fn run(mut stdout: impl Write) -> TResult<()> {
    // Write here
    // writeln!(stdout, "{}", s)?;

    input! {
        n: u64,
        a: u64,
        b: u64,
    }

    let mut result = 0;
    for x in 1..=n {
        let digits = get_digits(x)?;
        let sum: u64 = digits.iter().sum();

        if (a <= sum) && (b >= sum) {
            result += x;
        }
    }

    writeln!(stdout, "{}", result)?;

    Ok(())
}

fn main() -> TResult<()> {
    let stdout = std::io::stdout();
    let stdout = BufWriter::new(stdout.lock());

    run(stdout)
}
