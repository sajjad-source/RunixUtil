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

fn read_stdin(cli: &Cli) -> io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines().take(cli.count) {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}

fn read_file(file: &str, cli: &Cli) -> io::Result<()> {
    let contents = read_to_string(file)?;

    if cli.files.len() > 1 {
        println!("==> {} <==", file);
    }

    for line in contents.lines().take(cli.count) {
        println!("{}", line);
    }

    if cli.files.len() > 1 {
        println!();
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.files.is_empty() && read_stdin(&cli).is_err() {
        std::process::exit(1);
    }

    for file in &cli.files {
        if let Err(e) = read_file(file, &cli) {
            eprintln!("rhead: cannot open '{}': {}", file, e);
            std::process::exit(1);
        }
    }

    Ok(())
}
