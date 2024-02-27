use clap::Arg;
    
fn main() {
    let matches = clap::Command::new("ehcor")
        .bin_name("echor")
        .version("0.0.1")
        .author("Brett Kochendorfer <brett.kochendorfer@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .num_args(0..)
        )
        .arg(
          Arg::new("omit_newline")
          .short('n')
          .help("Do not print newline")
          .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    // why am I so dumb, wish I could do this in one line
    let text: Vec<&String> = matches.get_many("text").unwrap().collect();
    let text2: Vec<String> = text.into_iter().map(|x| x.to_string()).collect();

    let omit_newline = matches.get_flag("omit_newline");

    print!("{}{}", text2.join(" ").trim_end(), if omit_newline { "" } else { "\n" });
}
