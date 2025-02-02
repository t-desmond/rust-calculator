pub fn factorial(first_number: f64) -> f64 {
  let mut result: f64 = 1.0;
  for i in 2..=first_number as i32 {
      result *= i as f64;
  }
  result
}