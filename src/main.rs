mod add;
use add::add;
mod subtract;
use subtract::subtract;
mod multiply;
use multiply::multiply;
mod divide;
use divide::divide;
mod expo;
use expo::expo;
mod factorial;
use factorial::factorial;
mod modulo;
use modulo::modulo;
use std::fs::OpenOptions;
use std::io::Write;
mod get_input;
use get_input::get_input;

fn main() {
    println!("____***** MINI CALCULATOR APP****_______");
    
    let (first_number, second_number, operator) = get_input();
    let first_number = first_number.unwrap();
    let second_number = second_number.unwrap();
    let operator = operator.unwrap();

     let answer: f64 = if operator == "+" {
        add(first_number, second_number)
    } else if operator == "-" {
        subtract(first_number, second_number)
    } else if operator == "/" {
        if second_number == 0.0 {
            println!("cannot divide by 0");
            return;
        } else {
            divide(first_number, second_number)
        }
    } else if operator == "x" {
        multiply(first_number, second_number)
    } else if operator == "^" {
        expo(first_number, second_number)
    } else if operator == "fact" {
        factorial(first_number)
    } else if operator == "%" {
        modulo(first_number, second_number)
    }
    else {
        println!("operator not valied");
        return;
    };


    print!("{} {} {} = {}", first_number, operator, second_number, answer);

    log(first_number, operator, second_number,  answer);
}

fn log(first_number: f64, operator: String, second_number: f64, answer: f64) {
    let log_entry = format!("{} {} {} = {} \n", first_number, operator.trim(), second_number, answer);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("history.txt")
        .expect("Unable to open file");
    file.write_all(log_entry.as_bytes()).expect("Unable to write data");
}