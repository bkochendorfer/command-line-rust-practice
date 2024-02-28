use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

use clap::Arg;

fn main() {
    let matches = clap::Command::new("catr")
        .bin_name("catr")
        .version("0.0.1")
        .author("Brett Kochendorfer <brett.kochendorfer@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input FILE")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Print number")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number_noblank")
                .short('b')
                .long("number-noblank")
                .help("Number non-blank lines")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // why am I so dumb, wish I could do this in one line
    let file: Vec<&String> = matches.get_many("file").unwrap().collect();
    let files: Vec<String> = file.into_iter().map(|x| x.to_string()).collect();


    if files.get(0).unwrap() == &"-".to_string() {
        for line in io::BufReader::new(io::stdin()).lines().into_iter() {
            println!("{}", line.unwrap());
        }
        process::exit(0);
    }

    let mut line_num = 0;
    for x in files {
        if Path::new(&x).exists() {
            if let Ok(lines) = read_lines(x) {
                for line in lines.flatten() {
                    if matches.get_flag("number") {
                        line_num += 1;
                        println!("{:6}\t{}", line_num, line);
                    } else if matches.get_flag("number_noblank") {
                        if !line.is_empty() {
                            line_num += 1;
                            println!("{:6}\t{}", line_num, line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        } else {
            eprintln!("catr: {}: No such file or directory", x);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
