use clap::Parser;

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

    if cli.no_newline == true {
        print!("{}", cli.string);
    } else {
        println!("{}", cli.string);
    }
}
