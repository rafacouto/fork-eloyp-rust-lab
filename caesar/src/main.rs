use std::io;

mod caesar;
mod cli;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(error) = cli::exec::with(&args[1..], io::stdin().lock(), io::stdout()) {
        println!("{}", error);
        std::process::exit(1)
    }
}
