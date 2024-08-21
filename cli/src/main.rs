use clap::Parser;
use common::RandomStringGenerator;

#[derive(Parser, Debug)]
#[command(about = "Generates random strings")]
struct Cli {
    /// Number of strings to generate. Defaults to 5 if not provided
    #[arg(short, long)]
    count: Option<u32>,

    /// Length of the strings. Defaults to 32 if not provided
    #[arg(short, long)]
    length: Option<u32>,

    /// Exclude special characters from the generated strings
    #[arg(long)]
    exclude_special_chars: bool,

    /// Exclude numbers from the generated strings
    #[arg(long)]
    exclude_numbers: bool,

    /// Exclude uppercase letters from the generated strings
    #[arg(long)]
    exclude_uppercase: bool,
}

fn main() {
    let args = Cli::parse();
    let mut string_generator = RandomStringGenerator::new();

    if let Some(count) = args.count {
        string_generator.count = count;
    }

    if let Some(length) = args.length {
        string_generator.length = length;
    }

    string_generator.include_special_chars = !args.exclude_special_chars;
    string_generator.include_numbers = !args.exclude_numbers;
    string_generator.include_uppercase = !args.exclude_uppercase;

    let strings = match string_generator.generate() {
        Ok(val) => val,
        Err(err) => return println!("\x1b[0;31mError:\x1b[0m {}", err.to_string()),
    };

    for string in &strings {
        println!("\n{}", *string);
    }
}
