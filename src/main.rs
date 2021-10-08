use proconio::input;
use std::io::{prelude::*, BufWriter};

type TResult<T> = Result<T, Box<dyn std::error::Error>>;

fn run(mut stdout: impl Write) -> TResult<()> {
    // Write here
    // writeln!(stdout, "{}", s)?;

    input! {
        n: usize,
        a: [u64; n]
    }

    let mut cards = a;
    cards.sort();
    cards.reverse();

    let (alice, bob): (Vec<_>, Vec<_>) = cards
        .iter()
        .enumerate()
        .partition(|(index, _)| index % 2 == 0);
    let alice = alice.iter().map(|(_, &n)| n);
    let bob = bob.iter().map(|(_, &n)| n);
    let result = alice.sum::<u64>() - bob.sum::<u64>();

    writeln!(stdout, "{}", result)?;

    Ok(())
}

fn main() -> TResult<()> {
    let stdout = std::io::stdout();
    let stdout = BufWriter::new(stdout.lock());

    run(stdout)
}
