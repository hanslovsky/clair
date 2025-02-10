extern crate clair;
use clap::{ValueEnum, Parser};
use std::fmt::Display;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(ValueEnum, Debug, Clone)]
enum DType {
    I32,
    U32,
    F64
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_enum, default_value_t=DType::I32)]
    dtype: DType
}

fn aggregate_and_print<T>(t: T) -> Result<(), clair::AggregationErrorWithLine>
where T: AddAssign + FromStr + Display + Into<f64> + Clone
{
    let mut mean = clair::ArithmeticMean::new(t);
    let status = clair::aggregate_stdin(&mut mean, clair::HandleErrors::WARN);
    if status.is_ok() {
        println!("{}", mean.get_mean());
    }

    return status;
}

fn main() -> Result<(), clair::AggregationErrorWithLine> {
    let cli = Cli::parse();
    return match cli.dtype {
        DType::I32 => aggregate_and_print(0i32),
        DType::U32 => aggregate_and_print(0u32),
        DType::F64 => aggregate_and_print(0f64),
    }
}

