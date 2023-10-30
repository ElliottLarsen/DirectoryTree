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
    // Conver target_dir to PathBuf.
    let target = path::PathBuf::from(target_dir);
    println!("{}", target_dir);
    /*
    let files = target.read_dir().expect("This path is invalid.");
    for ent in files {
        let path = ent.unwrap().path();
        let fname = path.file_name().unwrap();
        println!("{}", fname);
    }
    */

}   
