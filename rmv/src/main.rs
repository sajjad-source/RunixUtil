use clap::Parser;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(
    name = "rmv",
    about = "A simple UNIX mv command",
    long_about = "The rmv utility renames the file named by the source operand to the destination path named by the
     target operand.",
    help_template = "{about}\n\n{usage-heading}\n    {usage}\n\n{all-args}{after-help}",
    after_help = "\nEXAMPLES:\n    rmv file1 file2        # Rename file1 to file2\n    rmv -i file1 file2     # Prompt before overwriting\n    rmv -n file1 file2     # Do not overwrite existing file\n    rmv -f file1 file2     # Force overwrite without prompting"
)]

struct Cli {
    #[arg(
        short = 'f',
        help = "Do not prompt for confirmation before overwriting the destination path. If the target file already exists, it will be overwritten.",
        overrides_with("interactive"),
        overrides_with("no_clobber")
    )]
    force: bool,
    #[arg(
        short = 'i',
        help = "Cause rmv to write a prompt to standard error before moving a file that would overwrite an existing file. If the response from the standard input begins with the character 'y' or 'Y', the move is attempted",
        overrides_with("force"),
        overrides_with("no_clobber")
    )]
    interactive: bool,
    #[arg(
        short = 'n',
        help = "Do not overwrite an existing file.",
        overrides_with("interactive"),
        overrides_with("force")
    )]
    no_clobber: bool,
    source: String,
    target: String,
}

fn should_overwrite(path: &Path, cli: &Cli) -> bool {
    if !path.exists() {
        return true;
    }

    if cli.no_clobber {
        return false;
    }

    if cli.interactive {
        return handle_interactive(&cli.target);
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

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let path = Path::new(&cli.target);

    if !should_overwrite(path, &cli) {
        return Ok(());
    }

    fs::rename(cli.source, cli.target)?;
    Ok(())
}
