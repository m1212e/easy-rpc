mod transpiler;
mod util;
use std::{env::current_dir};

use transpiler::run;

fn main() {
    run(&current_dir().unwrap()).unwrap();
}
