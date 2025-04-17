use clap::Parser;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Read;

/* TODO:
    1. Need to implement proper STDIN handling. Currently it doesn't work like the UNIX cat
    2. Need to add support for multiple files. Switch file: Option<String> -> Vec<String>
    3. Need to refactor my read_file(file: String) method to, instead of reading the entire file into a String::new() buffer, to
       instead loop line by line, and call the proper transformations on it (squueze, number, etc.) and then build up the buffer, to then return
       This will allow the helper functions to work for both STDIN and files, and allow chaining, i.e, rcat file1.txt - file2.txt
*/

#[derive(Parser, Debug)]
#[command(
    name = "rcat",
    about = "A simple UNIX cat command",
    long_about = "The rcat utility reads files sequentially, writing them to the standard output. The file operands 
     are processed in command-line order.  If file is a single dash (-) or absent, rcat reads from 
     the standard input.",
    help_template = "{about}\n\nUsage: {name} [OPTIONS] [file ...]\n\n{all-args}"
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
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut contents = match cli.file {
        Some(file) => read_file(file),
        None => {
            read_stdin(cli);
            return;
        }
    };

    if cli.squeeze {
        contents = squeeze_blank_lines(contents);
    }

    if cli.number_non_blank {
        contents = number_non_blank(contents);
    } else if cli.number_all {
        contents = number_all(contents);
    }

    println!("{}", contents);
}

fn read_file(file: String) -> String {
    let mut file = match File::open(file) {
        Ok(file) => file,
        Err(e) => {
            panic!("Error: {}", e)
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(e) => {
            panic!("Error: {}", e)
        }
    }
}

fn read_stdin(cli: Cli) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(mut line) => {
                if cli.squeeze {
                    line = squeeze_blank_lines(line);
                }

                if cli.number_non_blank {
                    line = number_non_blank(line);
                } else if cli.number_all {
                    line = number_all(line);
                }
                println!("{}", line);
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

fn number_non_blank(contents: String) -> String {
    let mut new_content = String::new();
    let mut line_num = 1;
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }

        new_content.push_str(format!("{:6} {}\n", line_num, line).as_str());
        line_num += 1;
    }
    new_content
}

fn number_all(contents: String) -> String {
    let mut new_content = String::new();
    let mut line_num = 1;
    for line in contents.lines() {
        new_content.push_str(format!("{:6} {}\n", line_num, line).as_str());
        line_num += 1;
    }
    new_content
}
