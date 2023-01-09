use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config= Config::new(&args).unwrap_or_else(|_err|
    {
        eprintln!("Problem parsing arguments:{}",err);
        process::exit(1);}
    );

    println!("{}",config.query);
    println!("{}",config.filename);

    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {}",e);
        process::exit(1);
    }
}




