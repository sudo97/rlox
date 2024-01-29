use std::io::Write;

use vm::VM;

use compile::Source;

mod common;
mod compile;
mod parse;
mod tokens;
mod vm;

fn repl() {
    let mut vm = VM::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n > 0 {
                    if let Some(chunk) = Source(input).compile("repl".into()) {
                        vm.interpret(chunk, vm::InterpretMode::Debug);
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

fn run_file(path: &str) {
    let source =
        Source(std::fs::read_to_string(path).expect("Something went wrong reading the file"));
    let mut vm = VM::new();
    if let Some(chunk) = source.compile(path.into()) {
        match vm.interpret(chunk, vm::InterpretMode::Debug) {
            vm::InterpretResult::Ok => {}
            vm::InterpretResult::CompileError => {
                std::process::exit(65);
            }
            vm::InterpretResult::RuntimeError => {
                std::process::exit(70);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    for arg in &args {
        println!("{}", arg);
    }

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage: rlox [path]");
        std::process::exit(64);
    }
}
