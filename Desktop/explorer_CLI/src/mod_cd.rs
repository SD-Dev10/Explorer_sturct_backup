use std::env;
//use std::fs;
use std::path::PathBuf;

pub fn cd_command(args: PathBuf) {
    let curr_dir = env::current_dir().expect("can't find directory");
    println!("current directory is: {}", curr_dir.display());

    let mut new_path = PathBuf::from(&curr_dir);
    new_path.push(args);
    match env::set_current_dir(&new_path) {
        Ok(_) => println!("successfully changed the path: {}", new_path.display()),
        Err(e) => println!("couldn't change the path, for the following error: {}", e),
    }
    //println!("this path is: {}", new_path);
}
