use clap::Parser;
use std::fs::copy;
use std::io;
use std::io::Write;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(
    name = "rcp",
    about = "A simple UNIX cp command",
    long_about = "The rcp utility copies the contents of the source_file to the target_file. 
     If rcp detects an attempt to copy a file to itself, the copy will fail.",
    help_template = "{about}\n\nUsage: {name} [OPTIONS] [source_file target_file]\n\n{all-args}"
)]
struct Cli {
    #[arg(
        short = 'f',
        help = "Force overwrite existing files without prompting",
        overrides_with("interactive"),
        overrides_with("no_clobber")
    )]
    force: bool,
    #[arg(
        short = 'i',
        help = "Prompt before overwriting existing files",
        overrides_with("force"),
        overrides_with("no_clobber")
    )]
    interactive: bool,
    #[arg(
        short = 'n',
        help = "Do not overwrite existing files",
        overrides_with("force"),
        overrides_with("interactive")
    )]
    no_clobber: bool,
    #[arg(short = 'v', help = "Show files as they are copied")]
    verbose: bool,
    source_file: String,
    target_file: String,
}

fn should_overwrite(path: &Path, cli: &Cli) -> bool {
    if !path.exists() {
        return true;
    }

    if cli.no_clobber {
        return false;
    }

    if cli.interactive {
        return handle_interactive(&cli.target_file);
    }

    true
}

fn handle_interactive(target_file: &str) -> bool {
    print!("overwrite {}? (y/n) ", target_file);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_lowercase().starts_with('y')
}
fn main() {
    let cli = Cli::parse();
    let path = Path::new(&cli.target_file);

    if !should_overwrite(path, &cli) {
        return;
    }

    copy(cli.source_file, cli.target_file).expect("Error copying from source file.");
}
