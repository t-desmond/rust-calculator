use std::fs::OpenOptions;
use std::io::Write;

pub struct Calculator {
  pub first_number: f64,
  pub second_number: f64
}

impl Calculator {
  pub fn add(&self, second_number:  f64) -> f64 {
    self.first_number + second_number
  }

  pub fn divide(first_number: f64, second_number:  f64) -> f64 {
    first_number / second_number
  }

  pub fn expo(first_number: f64, second_number:  f64) -> f64 {
    let n: f64 = 10.0;
    first_number * n.powf(second_number)
  }

  pub fn factorial(first_number: f64) -> f64 {
    let mut result: f64 = 1.0;
    for i in 2..=first_number as i32 {
        result *= i as f64;
    }
    result
  }

  pub fn modulo (first_number: f64, second_number: f64) -> f64 {
    first_number % second_number
  }

  pub fn multiply(first_number: f64, second_number:  f64) -> f64 {
    first_number * second_number
  }

  pub fn subtract(&self, second_number: f64) -> f64 {
    self.first_number - second_number
}
  pub fn log(first_number: f64, operator: String, second_number: f64, answer: f64) {
    let log_entry = format!("{} {} {} = {} \n", first_number, operator.trim(), second_number, answer);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("history.txt")
        .expect("Unable to open file");
    file.write_all(log_entry.as_bytes()).expect("Unable to write data");
  }
}