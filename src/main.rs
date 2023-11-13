use std::env;
use chrono::prelude::*;

fn main() {
    // Point at which we think this must be ms, not seconds
    const CUTOFF:i64 = 3_000_000_000;

    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <unix-timestamp>", args[0]);
        return;
    }

    let input = &args[1];

    if let Ok(mut parsed) = i64::from_str_radix(input, 10) {
        if parsed > CUTOFF {
            parsed = parsed / 1000;
        }
        if let Some(dt) = DateTime::from_timestamp(parsed, 0) {
            println!("{}", dt);
        } else {
            eprint!("Failed to parse as date: {}", parsed);
        }
    } else {
        eprintln!("Failed to parse as number: {}", input);
    }
}
