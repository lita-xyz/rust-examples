#![no_main]

#[no_mangle]

fn main() {
    valida_rs::io::println("Welcome to the Simple Calculator!");

    loop {
        valida_rs::io::println("Enter the first number:");
        let num1 = valida_rs::io::read_line::<f64>().unwrap();

        valida_rs::io::println("Enter the operation (+, -, *, /):");
        let operation = valida_rs::io::read_line::<String>().unwrap();

        valida_rs::io::println("Enter the second number:");
        let num2 = valida_rs::io::read_line::<f64>().unwrap();

        let result = match operation.as_str() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    valida_rs::io::println("Error: Division by zero!");
                    continue;
                }
            }
            _ => {
                valida_rs::io::println("Error: Invalid operation!");
                continue;
            }
        };

        valida_rs::io::println(&format!("Result: {}", result));

        valida_rs::io::println("Do you want to perform another calculation? (y/n)");
        let continue_calc = valida_rs::io::read_line::<String>().unwrap();
        if continue_calc.to_lowercase() != "y" {
            break;
        }
    }

    valida_rs::io::println("Thank you for using the Simple Calculator!");
}
