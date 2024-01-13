use clap::{arg, command, Parser};
use lisp::parse_program;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let contents = std::fs::read_to_string(args.path).unwrap();
    let input = contents.trim();
    match parse_program(input) {
        Ok((rest, parsed)) => {
            println!("Parsed successfully:\n\n{}", parsed);
            assert!(rest.is_empty());
        }
        Err(e) => eprintln!("Error parsing: {:?}", e),
    };
}
