extern crate clair;
use clap::{ValueEnum, Parser};
use std::fmt::Display;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(ValueEnum, Debug, Clone)]
enum DType {
    I64,
    U64,
    F64
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_enum, default_value_t=DType::I64)]
    dtype: DType
}

fn sum_and_print<T>(t: T) -> Result<(), clair::AggregationErrorWithLine>
where T: AddAssign + FromStr + Display
{
    let mut sum = clair::Sum::new(t);
    let status = clair::aggregate_stdin(&mut sum, clair::HandleErrors::WARN);
    if status.is_ok() {
        println!("{}", sum.get_sum());
    }

    return status;
}

fn main() -> Result<(), clair::AggregationErrorWithLine> {
    let cli = Cli::parse();
    return match cli.dtype {
        DType::I64 => sum_and_print(0i64),
        DType::U64 => sum_and_print(0u64),
        DType::F64 => sum_and_print(0f64),
    }
}

