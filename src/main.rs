use minigrep::runmod;
use minigrep::Config;
use std::env;
use std::process::exit;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem paring arguments:{}", err);
        exit(1);
    });
    if let Err(e) = runmod::run(config) {
        eprintln!("Application error:{}", e)
    };
}
