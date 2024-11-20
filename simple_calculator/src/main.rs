#![no_main]

#[no_mangle]

fn main() {
    println!("Welcome to the Simple Calculator!");

    loop {
        println!("Enter the first number:");
        let num1 = valida_rs::io::read_line::<f64>().unwrap();

        println!("Enter the operation (+, -, *, /):");
        let operation = valida_rs::io::read_line::<String>().unwrap();

        println!("Enter the second number:");
        let num2 = valida_rs::io::read_line::<f64>().unwrap();

        let result = match operation.as_str() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("Error: Division by zero!");
                    continue;
                }
            }
            _ => {
                println!("Error: Invalid operation!");
                continue;
            }
        };

        println!("Result: {}", result);

        println!("Do you want to perform another calculation? (y/n)");
        let continue_calc = valida_rs::io::read_line::<String>().unwrap();
        if continue_calc.to_lowercase() != "y" {
            break;
        }
    }

    println!("Thank you for using the Simple Calculator!");
}
