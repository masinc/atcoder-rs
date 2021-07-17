use proconio::input;
use std::io::{prelude::*, BufWriter};

type TResult<T> = Result<T, Box<dyn std::error::Error>>;

fn run(mut stdout: impl Write) -> TResult<()> {
    // Write here
    // writeln!(stdout, "{}", s)?;

    input! {
        _: u64,
        s: String
    }

    let index = s.chars().position(|x| x == '1').unwrap();

    let result = match index % 2 {
        1 => "Aoki",
        0 => "Takahashi",
        _ => unreachable!(),
    };

    writeln!(stdout, "{}", result)?;

    Ok(())
}

fn main() -> TResult<()> {
    let stdout = std::io::stdout();
    let stdout = BufWriter::new(stdout.lock());

    run(stdout)
}
