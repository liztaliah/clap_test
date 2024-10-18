use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, help = "enable to reverse string", action = clap::ArgAction::Count)]
    reverse: u8,

    #[arg(help = "user input to echo")]
    user_input: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let input = cli.user_input.join(" ");
    match cli.reverse {
        0 => println!("{}", input),
        _ => println!("{}", reverse_string(input.as_str())),
    }
}


fn reverse_string(input: &str) -> String {
    input.to_string().chars().rev().collect()
} 