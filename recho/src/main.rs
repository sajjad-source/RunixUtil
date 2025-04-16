use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "recho",
    about = "A simple UNIX echo command",
    help_template = "{about}\n\nUsage: {name} [OPTIONS] <STRING>\n\n{all-args}"
)]
struct Cli {
    #[arg(short = 'n', help = "Optional flag to not output the trailing newline")]
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
