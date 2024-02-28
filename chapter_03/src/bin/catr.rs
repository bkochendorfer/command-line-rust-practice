use std::fs;
use std::path::Path;

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
            .required(true)
            .num_args(0..)
        )
        .arg(
          Arg::new("number")
          .short('n')
          .long("number")
          .help("Print number")
          .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    // why am I so dumb, wish I could do this in one line
    let file: Vec<&String> = matches.get_many("file").unwrap().collect();
    let files: Vec<String> = file.into_iter().map(|x| x.to_string()).collect();

    for x in files {
        if Path::new(&x).exists() {
            let read_file = fs::read_to_string(x).unwrap();
            println!("{}", read_file);
        } else {
            println!("catr: {}: No such file or directory", x);
        }
    }
}
