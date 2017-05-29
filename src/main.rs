use std::cmp::Ordering;
use std::cmp::max;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write, stderr};

macro_rules! attempt {
	($try:expr, $error:expr) => {
		{
			if let Err(err) = $try {
				writeln!(io::stderr(), "{} (Details: {})", $error, err).unwrap();
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
				writeln!(stderr(), "Usage: spell [-v] [-n%] [query] [file ...]").unwrap();
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
	let mut verbose = false;

	let mut min_percent = 70.0;

	loop {
		if query == "-v" {
			verbose = true;
			query = unwrap!(args);
		} else {
			let query_chars: Vec<_> = query.chars().collect();
			if query_chars.len() >= 3 && query_chars[0] == '-' && query_chars[query_chars.len() - 1] == '%' {
				query.remove(0);
				let len = query.len();
				query.remove(len - 1);

				let parsed = query.parse();
				if parsed.is_err() {
					writeln!(io::stderr(), "Not a valid string").unwrap();
					return;
				}

				min_percent = parsed.unwrap();

				query = unwrap!(args);
			} else {
				break;
			}
		}
	}

	search(
		args.next().unwrap_or(DEFAULT.to_string()).as_str(),
		query.clone(),
		verbose,
		min_percent
	);

	while let Some(file) = args.next() {
		search(file.as_str(), query.clone(), verbose, min_percent);
	}
}

fn search(file: &str, query: String, verbose: bool, min_percent: f32) {
	let query = query.to_lowercase();

	let file = File::open(file);
	let file = attempt!(file, "Could not open file");
	let reader = BufReader::new(file);

	let mut results = Vec::new();

	for line in reader.lines() {
		let line = attempt!(line, "Could not read line from file");
		let line = line.to_lowercase();
		let total = max(query.len(), line.len());
		let mut shared = 0;

		let mut chars_query = query.chars();
		{
			let mut chars_line = line.chars();
			loop {
				let char_query = chars_query.next();
				let char_line = chars_line.next();

				if char_query.is_none() || char_line.is_none() {
					break;
				}

				if char_query.unwrap() == char_line.unwrap() {
					shared += 1;
				}
			}
		}

		let percent = shared as f32 / total as f32 * 100.0;

		if percent >= min_percent {
			results.push((percent, line));
		}
	}

	// Unsure if I should manually sort for performance?
	results.sort_by(
		|&(a, _), &(b, _)| if b == a {
			Ordering::Equal
		} else if b > a {
			Ordering::Greater
		} else {
			Ordering::Less
		}
	);

	for (percent, line) in results {
		if verbose {
			println!("({}% match):\t{}", percent, line);
		} else {
			println!("{}", line);
		}
	}
}
