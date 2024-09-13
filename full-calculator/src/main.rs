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

    match input.parse::<f64>() {
        Ok(n) => Ok(n),
        Err(_) => {
            let mut parts = input.split('+');
            if parts.count() > 1 {
                let a = parts.next().unwrap();
                let b = parts.next().unwrap();

                match (a.parse::<f64>(), b.parse::<f64>()) {
                    (Ok(a), Ok(b)) => Ok(a + b),
                    _ => Err(format!("Invalid addition: {}", input)),
                }
            } else {
                let mut parts = input.split('-');
                if parts.count() > 1 {
                    let a = parts.next().unwrap();
                    let b = parts.next().unwrap();

                    match (a.parse::<f64>(), b.parse::<f64>()) {
                        (Ok(a), Ok(b)) => Ok(a - b),
                        _ => Err(format!("Invalid subtraction: {}", input)),
                    }
                } else {
                    let mut parts = input.split('*');
                    if parts.count() > 1 {
                        let a = parts.next().unwrap();
                        let b = parts.next().unwrap();

                        match (a.parse::<f64>(), b.parse::<f64>()) {
                            (Ok(a), Ok(b)) => Ok(a * b),
                            _ => Err(format!("Invalid multiplication: {}", input)),
                        }
                    } else {
                        let mut parts = input.split('/');
                        if parts.count() > 1 {
                            let a = parts.next().unwrap();
                            let b = parts.next().unwrap();

                            match (a.parse::<f64>(), b.parse::<f64>()) {
                                (Ok(a), Ok(b)) => Ok(a / b),
                                _ => Err(format!("Invalid division: {}", input)),
                            }
                        } else {
                            Err(format!("Invalid expression: {}", input))
                        }
                    }
                }
            }
        }
    }
}
