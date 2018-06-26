use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
    }
    let directory = &args[1];
    let paths = fs::read_dir(directory).unwrap();
    for path in paths {
        let path_buf = path.unwrap().path();
        let file_name = path_buf.file_name().unwrap();
        println!("{}", file_name.to_string_lossy())
    }
}

fn usage() {
    eprintln!("usage: rls <dir>");
    process::exit(1);
            
}
