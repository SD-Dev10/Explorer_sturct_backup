use std::env;
use std::path::PathBuf;
pub fn cp_command(arg: PathBuf) {
    let curr_dir = env::current_dir().expect("couldn't find any path");
    println!("Current path: {}", curr_dir.display());
}
