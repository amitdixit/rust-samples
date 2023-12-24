use std::env;
use std::process;

use new_cli_grep_sample::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let query = &args[1];
    // let filename = &args[2];
    //  println!("Searching for {} in {}", query, filename);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1); // exit with error code 1
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = new_cli_grep_sample::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}
