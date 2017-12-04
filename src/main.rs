extern crate lcs;

use lcs::LcsTable;
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

macro_rules! attempt {
    ($try:expr, $error:expr) => {
        {
            if let Err(err) = $try {
                eprintln!("{} (Details: {})", $error, err);
                return;
            }

            $try.unwrap()
        }
    }
}
macro_rules! unwrap {
    ($iter:expr) => {
        {
            let next = $iter.next();

            if next.is_none() {
                eprintln!("Usage: spell [-a] [-v] [-n%] [query] [file ...]");
                return;
            }

            next.unwrap()
        }
    }
}

const DEFAULT: &str = "/usr/share/dict/words";

fn main() {
    let mut args = env::args();
    args.next();

    let mut query = unwrap!(args);
    let mut all = false;
    let mut verbose = false;

    let mut min_percent = None;

    loop {
        if query == "-a" {
            all   = true;
            query = unwrap!(args);
        } else if query == "-v" {
            verbose = true;
            query   = unwrap!(args);
        } else {
            let query_chars: Vec<_> = query.chars().collect();

            if query_chars.len() >= 3 && query_chars[0] == '-' && query_chars[query_chars.len() - 1] == '%' {
                query.remove(0);
                let len = query.len();
                query.remove(len - 1);

                let parsed = query.parse::<f32>();
                if parsed.is_err() {
                    eprintln!("Not a valid number");
                    return;
                }
                let parsed = parsed.unwrap();

                if parsed < 0.0 || parsed > 100.0 {
                    eprintln!("Not a valid percentage.");
                    return;
                }
                min_percent = Some(parsed);

                query = unwrap!(args);
            } else {
                break;
            }
        }
    }

    search(
        &args.next().unwrap_or_else(|| DEFAULT.to_string()),
        &query,
        all,
        verbose,
        min_percent
    );

    for file in args {
        search(&file, &query, all, verbose, min_percent);
    }
}

fn search(file: &str, query: &str, all: bool, verbose: bool, min_percent: Option<f32>) {
    let query = query.to_lowercase();

    let file = File::open(file);
    let file = attempt!(file, "Could not open file");
    let reader = BufReader::new(file);

    let mut results = Vec::new();

    for line in reader.lines() {
        let line = attempt!(line, "Could not read line from file");
        let line = line.to_lowercase();

        let query_chars = query.chars().collect::<Vec<_>>();
        let line_chars = line.chars().collect::<Vec<_>>();
        let lcs = LcsTable::new(&query_chars, &line_chars);

        let diff = lcs.longest_common_subsequence();
        let percent = diff.len() as f32 * 100.0 / query.len() as f32;

        results.push((percent, line));
    }

    results.sort_by(|&(a, ref line_a), &(b, ref line_b)| if (b - a).abs() < std::f32::EPSILON {
        line_a.len().cmp(&line_b.len())
    } else if b > a {
        Ordering::Greater
    } else {
        Ordering::Less
    });

    let mut min = min_percent.unwrap_or(100.0);
    loop {
        let mut printed = 0;

        for &(percent, ref line) in &results {
            if percent < min {
                continue;
            }
            if printed >= 7 && !all {
                break;
            }

            printed += 1;

            if verbose {
                // Because 101.0 isn't calculated, it's defined.
                // So no floating point issue can occur.
                println!("({}% match):\t{}", percent, line);
            } else {
                println!("{}", line);
            }
        }

        // Don't continue if found or manually modified min_percent
        if printed != 0 || min_percent.is_some() {
            break;
        }

        if min <= 0.0 {
            break;
        }
        min -= 10.0;
    }
}
