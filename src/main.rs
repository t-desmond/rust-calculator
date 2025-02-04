mod calculator;
use calculator::Calculator;
mod get_input;
use get_input::get_input;

fn main() {
    println!("____***** MINI CALCULATOR APP****_______");
    
    let (first_number, second_number, operator) = get_input();
    let first_number = first_number.unwrap();
    // let second_number = second_number.unwrap();
    let operator = operator.unwrap();

    let second_number = match second_number {
        Some(second_number) => {second_number},
        None => {
            if operator != "fact" {
                println!("invalid second number");
                return;
            } else {
                return;
            }
        }
    };

    let calculator = Calculator { first_number, second_number };
     let answer: f64 = if operator == "+" {
        calculator.add(second_number)
    } else if operator == "-" {
        calculator.subtract(second_number)
    } else if operator == "/" {
        if second_number == 0.0 {
            println!("cannot divide by 0");
            return;
        } else {
            Calculator::divide(first_number, second_number)
        }
    } else if operator == "x" {
        Calculator::multiply(first_number, second_number)
    } else if operator == "^" {
        Calculator::expo(first_number, second_number)
    } else if operator == "fact" {
        Calculator::factorial(first_number)
        
    } else if operator == "%" {
        Calculator::modulo(first_number, second_number)
    }
    else {
        println!("operator not valied");
        return;
    };

    print!("{} {} {} = {}", first_number, operator, second_number, answer);

    Calculator::log(first_number, operator, second_number,  answer);
}


