use computation::{create_locations, run_search};
use std::env;
use std::process;
use std::time::{Duration, Instant};

fn main() {
    let (num_threads, iterations, num_cities) = match parse_args() {
        Ok(values) => values,
        Err(message) => {
            eprintln!("{message}");
            process::exit(2);
        }
    };

    let locations = create_locations(42, num_cities);
    let mut total_time = Duration::ZERO;

    for _ in 0..5 {
        let start = Instant::now();
        run_search(iterations, 42, num_threads, 0, &locations)
            .expect("search should produce a solution");
        total_time += start.elapsed();
    }

    println!("Average search time: {:?}", total_time / 5);
}

fn parse_args() -> Result<(usize, usize, u32), String> {
    let mut args = env::args().skip(1);
    let mut num_threads = None;
    let mut iterations = None;
    let mut num_cities = None;

    while let Some(argument) = args.next() {
        if argument == "--help" || argument == "-h" {
            return Err(
                "Usage: cargo run --example bench -- --num-threads <N> --iterations <N> --num-cities <N>"
                    .to_owned(),
            );
        }

        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {argument}"))?;

        match argument.as_str() {
            "--num-threads" => {
                num_threads = Some(
                    value
                        .parse()
                        .map_err(|_| format!("invalid value for {argument}: {value}"))?,
                );
            }
            "--iterations" => {
                iterations = Some(
                    value
                        .parse()
                        .map_err(|_| format!("invalid value for {argument}: {value}"))?,
                );
            }
            "--num-cities" => {
                num_cities = Some(
                    value
                        .parse()
                        .map_err(|_| format!("invalid value for {argument}: {value}"))?,
                );
            }
            _ => return Err(format!("unknown argument: {argument}")),
        }
    }

    Ok((
        num_threads.ok_or("missing required argument: --num-threads")?,
        iterations.ok_or("missing required argument: --iterations")?,
        num_cities.ok_or("missing required argument: --num-cities")?,
    ))
}
