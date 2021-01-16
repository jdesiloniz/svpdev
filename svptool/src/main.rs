use std::process;
use svptool::Config;

fn main() {
    if let Ok(config) = Config::new_from_args() {
        if let Err(e) = svptool::run(config) {
            eprintln!("Application error(s): \n\n{}", e);
            process::exit(1);
        } else {
            println!("Execution OK.");
        };
    }
}
