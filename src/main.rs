use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: pink <file>");
        std::process::exit(1);
    }

    let file_name = &args[1];
    let file_paths = [
        Path::new("/usr/share/pink/").join(file_name).with_extension("pink"),
        Path::new("/usr/local/share/pink/").join(file_name).with_extension("pink"),
    ];

    let file_path = file_paths.iter()
        .find(|path| path.exists())
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("Pink file not found: {}.pink", file_name);
            std::process::exit(1);
        });

    let content = fs::read_to_string(&file_path).unwrap_or_else(|err| {
        eprintln!("Failed to read the pink file at {:?}: {:?}", file_path, err);
        std::process::exit(1);
    });

    let formatted_content = format_pink_content(&content);

    let mut moar = Command::new("moar")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start moar");

    if let Some(stdin) = &mut moar.stdin {
        stdin.write_all(formatted_content.as_bytes()).expect("Failed to write to moar's stdin");
    }

    moar.wait().expect("Failed to wait on moar");
}

fn parse_and_print_line(line: &str, inside_pink: &mut bool) {
    
    println!("{}", line);
}

fn load_pink_file(file_name: &str) -> io::Result<String> {
    let path = format!("/usr/local/share/pink/{}.pink", file_name);
    let mut file = fs::File::open(&path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn format_pink_content(content: &str) -> String {
    let mut formatted_content = String::new();
    let mut inside_pink = false;

    let mut start_idx = 0;
    while start_idx < content.len() {
        if inside_pink {
            if let Some(end_idx) = content[start_idx..].find("</pink>") {
                formatted_content.push_str(&content[start_idx..start_idx + end_idx]);
                start_idx += end_idx + 7; 
                inside_pink = false;
            } else {
                formatted_content.push_str(&content[start_idx..]);
                break;
            }
        } else {
            if let Some(start_pink_idx) = content[start_idx..].find("<pink>") {
                formatted_content.push_str(&apply_formatting(&content[start_idx..start_idx + start_pink_idx]));
                start_idx += start_pink_idx + 6; 
                inside_pink = true;
            } else {
                formatted_content.push_str(&apply_formatting(&content[start_idx..]));
                break;
            }
        }
    }

    formatted_content
}

fn apply_formatting(content: &str) -> String {
    content.replace("<HP>", "\x1b[38;5;200m")  // Set text color to pink
        .replace("</HP>", "\x1b[0m")  // Reset to default text color
        .replace("<H1>", "\x1b[1;37m")  // Bold and white text for H1
        .replace("</H1>", "\x1b[0m\n")  // Reset to default text color and add a newline
        .replace("<H2>", "\x1b[1;36m")  // Bold and cyan text for H2
        .replace("</H2>", "\x1b[0m\n")  // Reset to default text color and add a newline
        .replace("<H3>", "\x1b[1;34m")  // Bold and blue text for H3
        .replace("</H3>", "\x1b[0m\n")  // Reset to default text color and add a newline
        .replace("<b>", "\x1b[1m")
        .replace("</b>", "\x1b[0m")
        .replace("<i>", "\x1b[3m")
        .replace("</i>", "\x1b[0m")
        .replace("<u>", "\x1b[4m")
        .replace("</u>", "\x1b[0m")
        .replace("<code>", "\x1b[7m")
        .replace("</code>", "\x1b[0m")
}


