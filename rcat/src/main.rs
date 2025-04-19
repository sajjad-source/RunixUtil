use clap::Parser;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(
    name = "rcat",
    about = "A simple UNIX cat command",
    long_about = "The rcat utility reads files sequentially, writing them to the standard output. The file operands 
     are processed in command-line order.  If file is a single dash (-) or absent, rcat reads from 
     the standard input.",
    help_template = "{about}\n\n{usage-heading}\n    {usage}\n\n{all-args}{after-help}",
    after_help = "\nEXAMPLES:\n    rcat file.txt         # Display contents of file.txt\n    rcat -n file.txt      # Number all lines\n    rcat -b file.txt      # Number non-blank lines\n    rcat -s file.txt      # Squeeze multiple empty lines\n    cat file.txt | rcat   # Read from stdin"
)]
struct Cli {
    #[arg(
        short = 'b',
        help = "Number the non-blank output lines, starting at 1."
    )]
    number_non_blank: bool,
    #[arg(short = 'n', help = "Number all the output lines, starting at 1.")]
    number_all: bool,
    #[arg(
        short = 's',
        help = "Squeeze multiple adjacent empty lines, causing the output to be single spaced."
    )]
    squeeze: bool,
    files: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.files.len() == 0 {
        read_stdin(&cli);
    }

    for file in &cli.files {
        if file == "-" {
            read_stdin(&cli);
        } else {
            read_file(file, &cli);
        }
    }
}

fn read_file(file: &String, cli: &Cli) {
    let mut file = match File::open(file) {
        Ok(file) => file,
        Err(e) => {
            panic!("Error: {}", e)
        }
    };

    let mut contents = String::new();
    let mut contents = match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(e) => {
            panic!("Error: {}", e)
        }
    };

    if cli.squeeze {
        contents = squeeze_blank_lines(contents);
    }

    if cli.number_non_blank {
        contents = number_lines(contents, true);
    } else if cli.number_all {
        contents = number_lines(contents, false);
    }

    println!("{}", contents);
}

fn read_stdin(cli: &Cli) {
    let stdin = io::stdin();
    let mut last_line_was_empty = false;
    let mut line_num = 1;

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let is_empty = line.trim().is_empty();

                if cli.squeeze && is_empty && last_line_was_empty {
                    continue;
                }

                last_line_was_empty = is_empty;

                if cli.number_non_blank && !is_empty {
                    println!("{:6} {}", line_num, line);
                } else if cli.number_all {
                    println!("{:6} {}", line_num, line);
                } else {
                    println!("{}", line);
                }
                line_num += 1;
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }
}

fn squeeze_blank_lines(contents: String) -> String {
    let mut squeezed_contents = String::new();
    for c in contents.chars() {
        if let Some(char) = squeezed_contents.chars().last() {
            if c == '\n' && char == '\n' {
                continue;
            }
        }
        squeezed_contents.push(c);
    }
    squeezed_contents
}

fn number_lines(contents: String, skip_empty: bool) -> String {
    let mut new_content = String::new();
    let mut line_num = 1;
    for line in contents.lines() {
        if skip_empty && line.is_empty() {
            continue;
        }
        new_content.push_str(&format!("{:6} {}\n", line_num, line));
        line_num += 1;
    }
    new_content
}
