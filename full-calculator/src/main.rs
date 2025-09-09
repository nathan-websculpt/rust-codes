use std::io;

fn main() {
    loop {
        println!("Enter a mathematical expression: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let result = match calc(input.trim()) {
            Ok(n) => n,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        println!("Result: {}", result);
    }
}

fn calc(input: &str) -> Result<f64, String> {
    let input = input.replace(" ", "");

    if let Ok(n) = input.parse::<f64>() {
        return Ok(n);
    }

    // Helper closure for binary operations
    let eval = |op: char| -> Option<Result<f64, String>> {
        let parts: Vec<&str> = input.split(op).collect();
        if parts.len() == 2 {
            match (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                (Ok(a), Ok(b)) => {
                    return Some(Ok(match op {
                        '+' => a + b,
                        '-' => a - b,
                        '*' => a * b,
                        '/' => a / b,
                        _ => unreachable!(),
                    }))
                }
                _ => return Some(Err(format!("Invalid {} operation: {}", op, input))),
            }
        }
        None
    };

    // Try each operator
    for op in ['+', '-', '*', '/'] {
        if let Some(result) = eval(op) {
            return result;
        }
    }

    Err(format!("Invalid expression: {}", input))
}
