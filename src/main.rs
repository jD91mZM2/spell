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

	let mut min_percent = None;

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

				let parsed = query.parse::<f32>();
				if parsed.is_err() {
					writeln!(io::stderr(), "Not a valid number").unwrap();
					return;
				}
				let parsed = parsed.unwrap();

				if parsed < 0.0 || parsed > 100.0 {
					writeln!(io::stderr(), "Not a valid percentage.").unwrap();
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
		args.next().unwrap_or(DEFAULT.to_string()).as_str(),
		query.clone(),
		verbose,
		min_percent
	);

	while let Some(file) = args.next() {
		search(file.as_str(), query.clone(), verbose, min_percent);
	}
}

fn search(file: &str, query: String, verbose: bool, min_percent: Option<f32>) {
	let query = query.to_lowercase();
	let mut query_sort = query.chars().collect::<Vec<_>>();
	query_sort.sort();

	let file = File::open(file);
	let file = attempt!(file, "Could not open file");
	let reader = BufReader::new(file);

	let mut results = Vec::new();

	for line in reader.lines() {
		let line = attempt!(line, "Could not read line from file");
		let line = line.to_lowercase();

		if query == line {
			if verbose {
				println!("(Exact): {}", query);
			} else {
				println!("{}", query);
			}
			return;
		}

		let mut line_sort = line.chars().collect::<Vec<_>>();
		line_sort.sort();

		// Anagrams!
		if query_sort == line_sort {
			results.push((101.0, line));
			continue;
		}

		let total = max(query.len(), line.len());
		let mut shared = 0;

		{
			let mut chars_query = query.chars();
			let mut chars_line = line.chars();
			loop {
				let char_query = chars_query.next();
				let char_line = chars_line.next();

				if char_query.is_none() || char_line.is_none() {
					break;
				}

				if char_query.unwrap() == char_line.unwrap() {
					shared += 1;
				} else {
					// Search if any of the next characters in chars_line is char_query,
					// and if so shifts chars_line.
					// Useful for recognizing missing characters, like "cde" in "code".

					let mut i = 0;
					let mut clone = chars_line.clone();
					while let Some(next) = clone.next() {
						i += 1;
						if next == char_query.unwrap() {
							shared += 1;
							for _ in 0..i {
								chars_line.next();
							}
							break;
						}
					}
				}
			}
		}

		let percent = shared as f32 / total as f32 * 100.0;

		results.push((percent, line));
	}

	results.sort_by(|&(a, _), &(b, _)| if b == a {
		Ordering::Equal
	} else if b > a {
		Ordering::Greater
	} else {
		Ordering::Less
	});

	let mut min = min_percent.unwrap_or(100.0); // Copy
	loop {
		let mut found = false;

		for &(percent, ref line) in &results {
			if percent < min {
				continue;
			}

			found = true;

			if verbose {
				match percent {
					x if x == 101.0 => println!("(Anagram):\t{}", line),
					_ => println!("({}% match):\t{}", percent, line),
				}
			} else {
				println!("{}", line);
			}
		}

		// Don't continue if found or manually modified min_percent
		if found || min_percent.is_some() {
			break;
		}

		if min <= 0.0 {
			break;
		}
		min -= 10.0;
	}
}
