use ssp16asm::Config;
use std::process;

fn main() {
    if let Ok(config) = Config::new_from_args() {
        if let Err(e) = ssp16asm::run(config) {
            eprintln!("Application error(s): \n\n{}", e);
            process::exit(1);
        } else {
            println!("Assembly process complete.");
        };
    }
}
