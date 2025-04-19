use clap::Parser;
use std::fs::read_to_string;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(
    name = "rhead",
    about = "A simple UNIX head command",
    long_about = "This filter displays the first count lines or bytes of each of the specified files, 
    or of the standard input if no files are specified.  If count is omitted it defaults to 10.",
    help_template = "{about}\n\n{usage-heading}\n    {usage}\n\n{all-args}{after-help}",
    after_help = "\nEXAMPLES:\n    rhead file.txt         # Show first 10 lines of file.txt\n    rhead -n 5 file.txt    # Show first 5 lines of file.txt\n    cat file.txt | rhead   # Show first 10 lines from stdin"
)]

struct Cli {
    #[arg(
        short = 'n',
        help = "Displays the first count lines of each specified file.",
        default_value_t = 10
    )]
    count: usize,
    files: Vec<String>,
}

fn read_stdin(cli: &Cli) {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines().take(cli.count) {
        match line {
            Ok(line) => println!("{}", line),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}

fn read_file(file: &str, cli: &Cli) {
    read_to_string(file)
        .unwrap()
        .lines()
        .take(cli.count)
        .for_each(|line| println!("{}", line));
}

fn main() {
    let cli = Cli::parse();

    if cli.files.is_empty() {
        read_stdin(&cli);
    }
    for file in &cli.files {
        read_file(&file, &cli);
    }
}
