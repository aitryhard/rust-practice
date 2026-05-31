use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let expression = args[1..].join(" ");
        match calculator::calculate(&expression) {
            Ok(result) => println!("{result}"),
            Err(e) => eprintln!("Error: {e}"),
        }
        return;
    }

    println!("Calculator — enter an expression or :q to quit");

    let mut input = io::stdin().lock();
    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed == ":q" || trimmed == ":quit" {
                    break;
                }
                match calculator::calculate(trimmed) {
                    Ok(result) => println!("{result}"),
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
            Err(_) => break,
        }
    }
}
