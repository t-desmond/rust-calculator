use std::io::{self, Write};
pub fn get_input() -> (Option<f64>, Option<f64>, Option<String>) {
  let mut first_number = String::new();
    
  print!("Enter first number: ");
  io::stdout().flush().unwrap();
  io::stdin()
      .read_line(&mut first_number)
      .expect("Failed to read");

  let first_number: Option<f64> = match first_number
      .trim()
      .parse(){
          Ok(first_number) => Some(first_number),
          Err(_) => {
              println!("enter a number");
              None
          }
      };

  let mut operator = String::new();
  print!("Enter operator: ");
  io::stdout().flush().unwrap();
      io::stdin()
          .read_line(&mut operator)
          .expect("Failed to read");
      
  operator =  operator.trim().to_string();

  let mut second_number = String::new();

  print!("Enter the second number: ");
  io::stdout().flush().unwrap();

  io::stdin()
      .read_line(&mut second_number)
      .expect("failed to read the number");

      let second_number: Option<f64> = match second_number
      .trim()
      .parse()
      {
          Ok(first_number) => Some(first_number),
          Err(_) => {
              println!("enter a number");
              None
          }
      };
  
  (first_number, second_number, Some(operator))
}