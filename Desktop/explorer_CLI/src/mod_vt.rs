use std::env;
use std::fs::{self, Metadata};
use std::path::PathBuf;
//use std::collections::HashSet;

pub fn ls_command(
    current_dir: &PathBuf,
    depth: usize,
    indent_level: usize,
) -> Vec<(PathBuf, Metadata)> {
    let mut meta_vector = Vec::new();
    if current_dir.is_dir() {
        let dir_iterator = fs::read_dir(current_dir);
        match dir_iterator {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let full_path = entry.path();

                        match entry.metadata() {
                            Ok(meta) => {
                                //let mut seen: HashSet<PathBuf>= HashSet::new();
                                let indent = "   ".repeat(indent_level);

                                //println!("{}|-- {:?}",indent, full_path.clone().file_name().unwrap());

                                //println!("found: {:?}", entry.file_name().to_str().unwrap());
                                meta_vector.push((full_path.clone(), meta.clone()));
                                // if seen.insert(full_path.clone().file_name().unwrap().into()){
                                //       println!("{}|-- {:?}",indent, full_path.clone().file_name().unwrap());

                                // }
                                //println!("meta_vector: {:?}", meta_vector);
                                if meta.is_dir() && depth > 0 {
                                    println!(
                                        "{}|-- {:?}",
                                        indent,
                                        full_path.clone().file_name().unwrap()
                                    );
                                    let sub_meta_vector =
                                        ls_command(&full_path, depth - 1, indent_level + 1);

                                    // for (sub_path,_) in &sub_meta_vector{
                                    //       let sub_indent = "   ".repeat(indent_level + 1);

                                    //       println!("{}|__ {:?}",sub_indent, sub_path.clone().file_name().unwrap());

                                    // }
                                    meta_vector.extend(sub_meta_vector);
                                } else if meta.is_file()
                                    || meta.is_dir() && full_path.parent().is_some()
                                {
                                    let sub_indent = "   ".repeat(indent_level + 1);
                                    println!(
                                        "{}|__ {:?}",
                                        sub_indent,
                                        full_path.clone().file_name().unwrap()
                                    );
                                } else {
                                    println!(
                                        "{}|-- {:?}",
                                        indent,
                                        full_path.clone().file_name().unwrap()
                                    );
                                }
                            }
                            Err(e) => {
                                eprintln!("error occurred, can't read metadata: {}", e);
                            }
                        }
                    }
                }
            }

            Err(e) => {
                eprintln!("failed to read directory: {}", e);
            }
        }
    } else {
        println!("given path is invalid");
    }
    return meta_vector;
}
