use std::env;
use std::fs;
use std::path::PathBuf;

struct CLIObjects {
    pttrn: String,
    file: PathBuf,
}

fn find_pttrn(line: &str, pttrn: &str) -> Vec<usize> {
    let mut matches = Vec::new();
    if line.len() >= pttrn.len() {
        for i in 0..line.len() - pttrn.len() + 1 {
            if &line[i..i + pttrn.len()] == pttrn {
                matches.push(i);
            }
        }
    }
    matches
}

fn main() {
    let pattern = match env::args().nth(1) {
        Some(p) => p,
        None => {
            println!("Error: No pattern provided!");
            return;
        }
    };
    let file = match env::args().nth(2) {
        Some(f) => f,
        None => {
            println!("Error: No file provided!");
            return;
        }
    };

    let args = CLIObjects {
        pttrn: pattern,
        file: PathBuf::from(file),
    };

    let txtstring = match fs::read_to_string(&args.file) {
        Ok(txtstring) => txtstring,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    println!(
        "Looking for: \"{}\" in \"{}\"",
        args.pttrn,
        args.file.display()
    );

    let mut response: Vec<String> = Vec::new();
    for (line_index, line) in txtstring.lines().enumerate() {
        let matches = find_pttrn(line, &args.pttrn);
        for match_index in matches {
            response.push(format!(
                "Pattern found at [{}:{}]",
                line_index + 1,
                match_index + 1
            ));
        }
    }

    for i in response {
        println!("{}", i);
    }
}
