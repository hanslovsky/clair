extern crate clair;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
}

fn main() -> Result<(), clair::AggregationErrorWithLine> {
    let _ = Cli::parse();
    let mut count = clair::Count::new();
    let result = clair::aggregate_stdin::<clair::Count>(&mut count, clair::HandleErrors::WARN);
    if result.is_ok() {
        println!("{}", count.get_count());
    }
    return result;
}

