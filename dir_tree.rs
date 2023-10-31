use std::{env, path};
fn main() {
    // Read argument values.
    let args: Vec<String> = env::args().collect();
    // If no path is given, set it to the current directory.
    let mut target_dir = ".";
    if args.len() >= 2 {
        // If path is given.
        target_dir = &args[1];
    }
    // Convert target_dir to PathBuf.
    let target = path::PathBuf::from(target_dir);
    println!("{}", target_dir);
    tree(&target, 0);
}

fn tree(target: &path::PathBuf, level: isize) {
    // Get list of files.
    let files = target.read_dir().expect("This path does not exist.");
    for entry in files {
        // Get PathBuf.
        let path = entry.unwrap().path();
        // Indentation.
        for _ in 1..=level {
            print!("|   ");
        }
        // Get file name.
        let file_name = path.file_name().unwrap().to_string_lossy();
        // If directory, recurse.
        if path.is_dir() {
            println!("|-- <{}>", file_name);
            tree(&path, level + 1);
            continue;
        }
        println!("|-- {}", file_name);
    }
}
