use std::default::Default;
use std::io;
use std::ops::{AddAssign,MulAssign};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum AggregationError {
    EmptyString,
    InvalidString,
}


pub trait Aggregator {
    fn aggregate(&mut self, item: &String) -> Result<(), AggregationError>;
}


pub struct Count {
    count: u64
}

impl Count {
    pub fn new() -> Self {
        return Count { count: 0u64 };
    }

    pub fn get_count(&self) -> u64 {
        return self.count;
    }
}

impl Aggregator for Count {
    fn aggregate(&mut self, item: &String) -> Result<(), AggregationError> {
        self.count += 1;
        return Ok(())
    }
}


pub struct Sum<T> where T: FromStr {
    sum: T
}

impl <T> Sum<T> where T: FromStr {
    pub fn get_sum(&self) -> &T {
        return &self.sum;
    }

    pub fn new(initial: T) -> Self {
        return Self {sum: initial};
    }
}

impl <T> Default for Sum<T> where T: FromStr + Default {
    fn default() -> Self {
        return Self::new(T::default());
    }
}

impl <T> Aggregator for Sum<T> where T: FromStr + AddAssign {
    fn aggregate(&mut self, item: &String) -> Result<(), AggregationError> {
        return match T::from_str(&item) {
            Err(e) => Err(AggregationError::InvalidString),
            Ok(to_add) => {
                self.sum += to_add;
                Ok(())
            }
        }
    }
}


pub struct Product<T> where T: FromStr {
    product: T
}

impl <T> Product<T> where T: FromStr {
    pub fn get_product(&self) -> &T {
        return &self.product;
    }

    pub fn new(initial: T) -> Self {
        return Self {product: initial};
    }
}

impl <T> Aggregator for Product<T> where T: FromStr + MulAssign {
    fn aggregate(&mut self, item: &String) -> Result<(), AggregationError> {
        return match T::from_str(&item) {
            Err(e) => Err(AggregationError::InvalidString),
            Ok(to_add) => {
                self.product *= to_add;
                Ok(())
            }
        }
    }
}


pub struct ArithmeticMean<T> where T: FromStr {
    sum: Sum<T>,
    count: Count,
}

impl <T> ArithmeticMean<T> where T: FromStr + Into<f64> + Clone {

    fn get_sum_f64(&self) -> f64 {
        return self.sum.get_sum().clone().into();
        // return self.sum.get_sum().copy().into();
    }

    fn get_count_f64(&self) -> f64 {
        return self.count.get_count() as f64;
    }
    
    pub fn get_mean(&self) -> f64 {
        return self.get_sum_f64() / self.get_count_f64();
    }

    pub fn new(initial: T) -> Self {
        return Self {sum: Sum::new(initial), count: Count::new()};
    }
}

impl <T> Aggregator for ArithmeticMean<T> where T: FromStr + AddAssign {
    fn aggregate(&mut self, item: &String) -> Result<(), AggregationError> {
        return match self.count.aggregate(item) {
            Err(e) => Err(e),
            Ok(_) => match self.sum.aggregate(item) {
                Err(e) => Err(e),
                Ok(_) => Ok(())
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct AggregationErrorWithLine {
    error: AggregationError,
    line_number: usize
}

impl AggregationErrorWithLine {
    fn new(error: AggregationError, line_number: usize) -> Self {
        return AggregationErrorWithLine {
            error: error,
            line_number: line_number
        }
    }
}

pub enum HandleErrors {
    FAIL,
    IGNORE,
    WARN
}

pub fn aggregate<L, A>(
    lines: L,
    aggregator: &mut A,
    handle_errors: HandleErrors,
) -> Result<(), AggregationErrorWithLine>
where
L: Iterator<Item =Result<String, io::Error>>, A: Aggregator
{
    for (line_number, line) in lines.enumerate() {
        let uw = &line.unwrap();
        match aggregator.aggregate(uw) {
            Ok(_) => continue,
            Err(error) => match handle_errors {
                HandleErrors::IGNORE => continue,
                HandleErrors::FAIL => return Err(AggregationErrorWithLine::new(error, line_number)),
                HandleErrors::WARN => println!("Unable to aggregate line {}: {} ({:?})", line_number, uw, AggregationErrorWithLine::new(error, line_number)),
            }
        }
    }
    return Ok(());
}

pub fn stdin_iterator() -> io::Lines<io::StdinLock<'static>> {
    let stdin = io::stdin();
    return stdin.lines();
}

pub fn aggregate_stdin<A>(aggregator: &mut A, handle_errors: HandleErrors) -> Result<(), AggregationErrorWithLine>
where
A: Aggregator {
    return aggregate(stdin_iterator(), aggregator, handle_errors);
}
