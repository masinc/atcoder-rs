use itertools::Itertools;
use proconio::input;
use std::{
    io::{prelude::*, BufWriter},
    ops::Sub,
};

type TResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    time: u64,
    x: i64,
    y: i64,
}

impl Point {
    fn new(time: u64, x: i64, y: i64) -> Self {
        Self { time, x, y }
    }

    fn distance(&self) -> u64 {
        self.x.abs() as u64 + self.y.abs() as u64
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            time: 0,
            x: 0,
            y: 0,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            time: self.time - rhs.time,
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn run(mut stdout: impl Write) -> TResult<()> {
    // Write here
    // writeln!(stdout, "{}", s)?;

    input! {
        n: u64,
        v: [(u64, i64, i64); n]
    }

    let points = v
        .into_iter()
        .map(|(time, x, y)| Point::new(time, x, y))
        .collect_vec();

    let mut current: Point = Default::default();

    let mut result = true;

    for p in points {
        let elasped_point = p - current;

        let distance = elasped_point.distance();
        if (distance > elasped_point.time) || (distance % 2 != elasped_point.time % 2) {
            result = false;
            break;
        }

        current = p;
    }

    writeln!(stdout, "{}", if result { "Yes" } else { "No" })?;

    Ok(())
}

fn main() -> TResult<()> {
    let stdout = std::io::stdout();
    let stdout = BufWriter::new(stdout.lock());

    run(stdout)
}
