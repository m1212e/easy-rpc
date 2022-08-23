mod transpiler;
mod util;
use std::env::{self, current_dir};

use transpiler::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    run(&current_dir().unwrap(), args.contains(&"-w".to_string())).unwrap();
}
