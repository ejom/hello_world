use std::io;
use evalexpr::{context_map, eval_with_context};

/* potential error
fn is_poly(f: &str) -> bool {
    let mut valid = false;
    for i in f.chars() {
        if i.is_digit(10) || c == '+' || c == '-' || c == '*' || c == '/' || c == '^' || c == '(' || c == ')' {
            for j in f.chars() {
                if j.is_alphabetic() {
                    let var = j;

                }
            }
            valid = false;
            break;
        }
    }
    valid
}
*/
/* 
fn is_poly(f: &str) -> bool {
    let valid = true;
    let mut has_var = false;
    let mut var = ' ';
    for i in f.chars() {
        if !(i.is_digit(10) || i == '+' || i == '-' || i == '*' || i == '/' || i == '^' || i == '(' || i == ')') {
            return false;}
        }
    for i in f.chars() {
        if i.is_alphabetic() {
            var = i;
            has_var = true;
            break;
        }
    }
    if !has_var {return false;}
    for i in f.chars() {
        if i.is_alphabetic() {
            if i != var {
                return false;
            }
        }
    }
    valid
}
    */

fn is_poly(f: &str) -> bool {
    let mut has_var = false;
    let mut var = None;

    for i in f.chars() {
        if !(i.is_digit(10) || i == '+' || i == '-' || i == '*' || i == '/' || i == '^' || i == '(' || i == ')') {
            if i.is_alphabetic() {
                if let Some(existing_var) = var {
                    if existing_var != i {
                        return false; // Reject multiple different variables
                    }
                } else {
                    var = Some(i); // Store the first variable found
                    has_var = true;
                }
            } else {
                return false; // Reject invalid characters
            }
        }
    }

    has_var // Return true if at least one variable is present
}
    

fn calc_f(f: String, x_vals: &[f64]) -> Vec<(f64, f64)> {
    let mut results = Vec::new();

    for &x in x_vals {
        let context = context_map! { "x" => x }.unwrap();

        match eval_with_context(&f, &context) {
            Ok(result) => results.push((x, result.as_number().unwrap())),
            Err(e) => {
                println!("Error evaluating expression: {:?}", e);
                return vec![];
            }
        }
    }

    results
}

/*fn calc_function(f: &str, x: f64) -> f64 {
    let mut result = 0.0;
    let mut operator = '+';
    let mut current_num = String::new();
    for c in f.chars() {
        if c.is_digit(10) || c == '.' {
            current_num.push(c);
        } else {
            if !current_num.is_empty() {
                let number: f64 = current_num.parse().unwrap();
                match operator {
                    '+' => result += number,
                    '-' => result -= number,
                    '*' => result *= number,
                    '/' => result /= number,
                    _ => {}
                }
                current_num.clear();
            }
            operator = c;
        }
    }
    if !current_num.is_empty() {
        let number: f64 = current_num.parse().unwrap();
        match operator {
            '+' => result += number,
            '-' => result -= number,
            '*' => result *= number,
            '/' => result /= number,
            _ => {}
        }
    }
    result
}
    */

fn main() {
    //input a 1 variable function
    let f = loop {
        println!("Enter a single variable polynomial with integer exponents. Enter only numbers, a single character, and mathmatical operators +, -, /, *, (), and ^:");
        let mut f= String::new();
        io::stdin()
            .read_line(&mut f)
            .expect("Failed to read line");
        f = f.trim().to_string(); //remove whitespaces and lines
        //check if input is valid (only contains 1 character, numbers, and valid operators)
        if is_poly(&f) {
            break f;
        } else {
            println!("Function is invalid.");
        }
    };
 
    //Input an array of numbers
    let values: Vec<f64> = loop {
        println!("Enter an array of values separated by spaces (e.g., 1 2 3 4): ");
        let mut input_values = String::new();
        io::stdin().read_line(&mut input_values).expect("Failed to read line");
        let input_values: Vec<String> = input_values
            .trim() // Remove leading/trailing whitespace
            .split_whitespace() // Split by spaces
            .map(|s| s.to_string())
            .collect();
        //check that inputs are all numbers
        let values: Result<Vec<f64>, _> = input_values.iter()
            .map(|v| v.parse::<f64>())
            .collect(); 

        match values {
            Ok(values) => {
                // Successfully parsed all numbers
                break values;
            }
            Err(_) => {
                println!("Invalid input! Please ensure all values are numbers.");
            }
        }
    };

            /*
             let values: Vec<f64> = input_values.iter().map(|v| v.parse::<f64>().ok()).collect(); //remove ok
        if !(values.iter().all(|x| !x.is_err())) { //remove the ! potential error in x.is_err()
            println!("Invalid input! Please ensure all values are numbers.");
        } else {
            break;
        }
        */
    //output a table of the inputs and outputs
    let results = calc_f(f, &values); // Corrected function call

    println!("{:<15} {:<15}", "Input", "Output");
    for (x, result) in results {
        println!("{:<15} {:<15.2}", x, result);
    }

}
/* 
use std::io::{self, Write};

fn main() {
    // Step 1: Prompt the user to enter a mathematical expression (function)
    println!("Enter a function (e.g., x + 1): ");
    let mut function = String::new();
    io::stdin().read_line(&mut function).expect("Failed to read line");
    let function = function.trim(); // Remove any extra spaces or newline

    // Step 2: Validate that the function contains only valid characters (numbers and operators)
    if !function.chars().all(|c| c.is_numeric() || c == '+' || c == '-' || c == '*' || c == '/' || c == 'x' || c == '.' || c.is_whitespace()) {
        println!("Invalid function! Only numbers, operators (+, -, *, /) and 'x' are allowed.");
        return;
    }

    // Step 3: Get input values for the variable
    println!("Enter an array of values separated by spaces (e.g., 1 2 3 4): ");
    let mut input_values = String::new();
    io::stdin().read_line(&mut input_values).expect("Failed to read line");
    let input_values: Vec<String> = input_values
        .trim() // Remove leading/trailing whitespace
        .split_whitespace() // Split by spaces
        .map(|s| s.to_string())
        .collect();

    // Step 4: Ensure all inputs are valid numbers
    let values: Vec<f64> = input_values.iter().map(|v| v.parse::<f64>()).collect();
    if values.iter().any(|x| x.is_err()) {
        println!("Invalid input! Please ensure all values are numbers.");
        return;
    }

    // Step 5: Output a table of the inputs and outputs
    println!("{:<15} {:<15}", "Input", "Output");
    for value in values.iter().map(|x| x.as_ref().unwrap()) {
        let result = calculate_function(function, *value);
        println!("{:<15} {:<15.2}", value, result);
    }
}

// Function to calculate the function result given an input
fn calculate_function(function: &str, x: f64) -> f64 {
    // Here we use a simple approach to calculate the value of 'x' in the function
    // Assuming the function contains only basic operations and 'x' as a variable

    // Replace 'x' with the value
    let function_with_value = function.replace("x", &x.to_string());

    // Now evaluate the expression (this is a very simple evaluation, for demo purposes)
    // In a real-world scenario, you could use a library like `meval` to parse and evaluate the expression.
    let result = eval_expr(&function_with_value);
    
    result
}

// A simple evaluation function for basic mathematical operations
fn eval_expr(expr: &str) -> f64 {
    // Basic eval implementation (only for +, -, *, and /)
    // Note: This is not a full parser, but a simple evaluator for this demo purpose.
    let expr = expr.replace(" ", ""); // Remove spaces for ease
    let tokens = expr.split_whitespace().collect::<Vec<&str>>();

    let mut result = 0.0;
    let mut operator = '+';

    let mut current_num = String::new();

    for c in expr.chars() {
        if c.is_digit(10) || c == '.' {
            current_num.push(c);
        } else {
            if !current_num.is_empty() {
                let number: f64 = current_num.parse().unwrap();
                match operator {
                    '+' => result += number,
                    '-' => result -= number,
                    '*' => result *= number,
                    '/' => result /= number,
                    _ => {}
                }
                current_num.clear();
            }
            operator = c;
        }
    }

    // At the end, we might have one last number left to process
    if !current_num.is_empty() {
        let number: f64 = current_num.parse().unwrap();
        match operator {
            '+' => result += number,
            '-' => result -= number,
            '*' => result *= number,
            '/' => result /= number,
            _ => {}
        }
    }

    result
}
*/