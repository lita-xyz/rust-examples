#![no_main]

entrypoint::entrypoint!(main);

fn main() {
    entrypoint::io::println("Welcome to the Simple Calculator!");

    loop {
        entrypoint::io::println("Enter the first number:");
        let num1 = entrypoint::io::read_line::<f64>().unwrap();

        entrypoint::io::println("Enter the operation (+, -, *, /):");
        let operation = entrypoint::io::read_line::<String>().unwrap();

        entrypoint::io::println("Enter the second number:");
        let num2 = entrypoint::io::read_line::<f64>().unwrap();

        let result = match operation.as_str() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    entrypoint::io::println("Error: Division by zero!");
                    continue;
                }
            }
            _ => {
                entrypoint::io::println("Error: Invalid operation!");
                continue;
            }
        };

        entrypoint::io::println(&format!("Result: {}", result));

        entrypoint::io::println("Do you want to perform another calculation? (y/n)");
        let continue_calc = entrypoint::io::read_line::<String>().unwrap();
        if continue_calc.to_lowercase() != "y" {
            break;
        }
    }

    entrypoint::io::println("Thank you for using the Simple Calculator!");
}
