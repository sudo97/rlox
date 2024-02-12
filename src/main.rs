use std::io::Write;

use vm::VM;

use compile::Source;

mod common;
mod compile;
mod parse;
mod tokens;
mod vm;

fn repl(mode: vm::InterpretMode) {
    let mut vm = VM::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n > 0 {
                    if let Some(chunk) = Source(input).compile("repl") {
                        vm.interpret(chunk, mode);
                    }
                } else {
                    println!("Bye!");
                    break;
                }
            }
            Err(error) => {
                println!("error: {}", error);
                break;
            }
        };
    }
}

fn run_file(path: &str, mode: vm::InterpretMode) {
    let source =
        Source(std::fs::read_to_string(path).expect("Something went wrong reading the file"));
    let mut vm = VM::new();
    match source.compile(path) {
        Some(chunk) => {
            if let vm::InterpretResult::RuntimeError = vm.interpret(chunk, mode) {
                std::process::exit(70);
            }
        }
        None => {
            println!("Failed to compile");
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut file_ref: Option<&String> = None;
    let mut interpret_mode = vm::InterpretMode::Release;

    for arg in &args[1..] {
        match arg.as_str() {
            "-d" | "--debug" => interpret_mode = vm::InterpretMode::Debug,
            _ => {
                if file_ref.is_none() {
                    file_ref = Some(arg);
                } else {
                    eprintln!("Unexpected argument: {}", arg);
                    std::process::exit(1);
                }
            }
        }
    }

    if let Some(path) = file_ref {
        run_file(path.as_str(), interpret_mode);
    } else {
        repl(interpret_mode);
    }
}

#[cfg(test)]
mod tests {}
