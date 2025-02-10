extern crate clair;
use clap::{ValueEnum, Parser};
use std::fmt::Display;
use std::ops::MulAssign;
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

fn multiply_and_print<T>(t: T) -> Result<(), clair::AggregationErrorWithLine>
where T: MulAssign + FromStr + Display
{
    let mut product = clair::Product::new(t);
    let status = clair::aggregate_stdin(&mut product, clair::HandleErrors::WARN);
    if status.is_ok() {
        println!("{}", product.get_product());
    }

    return status;
}

fn main() -> Result<(), clair::AggregationErrorWithLine> {
    let cli = Cli::parse();
    return match cli.dtype {
        DType::I64 => multiply_and_print(1i64),
        DType::U64 => multiply_and_print(1u64),
        DType::F64 => multiply_and_print(1f64),
    }
}

