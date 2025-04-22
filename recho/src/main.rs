use clap::Parser;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(
    name = "recho",
    about = "A simple UNIX echo command",
    long_about = "The recho utility writes any specified operands, separated by single blank (' ') characters and followed by a newline ('\\n') character, to the standard output.",
    help_template = "{about}\n\n{usage-heading}\n    {usage}\n\n{all-args}{after-help}",
    after_help = "\nEXAMPLES:\n    recho hello world     # Print 'hello world' with newline\n    recho -n hello       # Print 'hello' without newline"
)]
struct Cli {
    #[arg(short = 'n', help = "Do not output the trailing newline")]
    no_newline: bool,
    string: String,
}

fn main() {
    let cli = Cli::parse();

    let result = if cli.no_newline {
        io::stdout().write_all(cli.string.as_bytes())
    } else {
        io::stdout().write_all(format!("{}\n", cli.string).as_bytes())
    };

    if let Err(e) = result {
        eprintln!("recho: write error: {}", e);
        std::process::exit(1);
    }
}
