// use phf::phf_map;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = rust_vm_translator::run(&args) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
