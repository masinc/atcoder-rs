use itertools::Itertools;
use proconio::input;
use std::io::{prelude::*, BufWriter};

type TResult<T> = Result<T, Box<dyn std::error::Error>>;

fn process(source: &str, words: &[&str]) -> bool {
    if source.is_empty() {
        return true;
    }

    let target_words = words
        .iter()
        .filter(|&&w| source.starts_with(w))
        .cloned()
        .collect_vec();

    for word in target_words {
        if process(&source[word.len()..], words) {
            return true;
        }
    }

    false
}

fn run(mut stdout: impl Write) -> TResult<()> {
    // Write here
    // writeln!(stdout, "{}", s)?;

    input! {
        s: String,
    }

    let mut words = "dream dreamer erase eraser"
        .split_ascii_whitespace()
        .collect_vec();
    words.sort_by_key(|w| w.len());
    words.reverse();
    let words = words;

    let result = process(&s, &words);
    writeln!(stdout, "{}", if result { "YES" } else { "NO" })?;

    Ok(())
}

fn main() -> TResult<()> {
    let stdout = std::io::stdout();
    let stdout = BufWriter::new(stdout.lock());

    run(stdout)
}
