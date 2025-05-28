use std::io;
use evalexpr::{context_map, eval_with_context};

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

    //output a table of the inputs and outputs
    let results = calc_f(f, &values); // Corrected function call

    println!("{:<15} {:<15}", "Input", "Output");
    for (x, result) in results {
        println!("{:<15} {:<15.2}", x, result);
    }

}
