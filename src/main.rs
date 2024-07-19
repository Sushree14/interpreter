use std::collections::HashMap;
use std::io::{self, BufRead};

struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    fn interpret(&mut self, input: &str) {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        match tokens[0] {
            "variable" => self.assign_variable(&tokens),
            "print" => self.print_expression(&tokens[1..]),
            _ => println!("Unknown command"),
        }
    }

    fn assign_variable(&mut self, tokens: &[&str]) {
        if tokens.len() == 5 && tokens[2] == "isequal" && tokens[3] == "to" {
            let var_name = tokens[1];
            if let Ok(value) = tokens[4].parse() {
                self.variables.insert(var_name.to_string(), value);
                println!("Assigned {} to {}", value, var_name);
            } else {
                println!("Invalid value for assignment");
            }
        } else {
            println!("Invalid assignment syntax");
        }
    }

    fn print_expression(&self, tokens: &[&str]) {
        if tokens.len() != 3 {
            println!("Invalid print syntax");
            return;
        }

        let left = self.get_value(tokens[0]);
        let right = self.get_value(tokens[2]);

        let result = match tokens[1] {
            "plus" => left + right,
            "minus" => left - right,
            "times" => left * right,
            "divided" => left / right,
            _ => {
                println!("Unknown operator");
                return;
            }
        };

        println!("Result: {}", result);
    }

    fn get_value(&self, token: &str) -> f64 {
        token.parse().unwrap_or_else(|_| {
            *self.variables.get(token).unwrap_or_else(|| {
                println!("Undefined variable: {}", token);
                &0.0
            })
        })
    }
}

fn main() {
    let mut interpreter = Interpreter::new();
    let stdin = io::stdin();

    println!("Simple Interpreter. Type 'exit' to quit.");

    for line in stdin.lock().lines() {
        let input = line.unwrap();
        if input == "exit" {
            break;
        }
        interpreter.interpret(&input);
    }
}