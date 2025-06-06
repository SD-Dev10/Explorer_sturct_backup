//use std :: env;
use colored::*;
use std::path::PathBuf;
use std::io::{self, Write};
mod mod_cd;
use mod_cd::cd_command;

fn main (){

    let banner = r#"
    ███████╗██╗███╗   ██╗██████╗ ███████╗██████╗ 
    ██╔════╝██║████╗  ██║██╔══██╗██╔════╝██╔══██╗
    █████╗  ██║██╔██╗ ██║██║  ██║█████╗  ██████╔╝
    ██╔══╝  ██║██║╚██╗██║██║  ██║██╔══╝  ██╔══██╗
    ██║     ██║██║ ╚████║██████╔╝███████╗██║  ██║
    ╚═╝     ╚═╝╚═╝  ╚═══╝╚═════╝ ╚══════╝╚═╝  ╚═╝
    "#;
    let first_cmd = "current_path".bright_magenta().italic();
    let second_cmd = "view_tree".bright_magenta().italic();
    let third_cmd = "change_directory".bright_magenta().italic();
    let cmds = format!(
        "
        cp               {}

        vt               {}

        cd               {}
        ", 
        first_cmd, second_cmd, third_cmd

    );
        
    println!("{}", banner.bright_blue().bold());
    println!("{}", cmds.bright_blue().bold());
    

    loop{
        print!("{}", "Finder ---> ".bright_blue());
        io::stdout().flush().unwrap();

        //read input
        let mut input = String::new();
        io::stdin().read_line(&mut input ).unwrap();
        println!("{} {}", 
             "You have selected:".bright_green(),
             input.bright_magenta());
        
        //trimming the input
        let parts:Vec<&str> = input.trim().split_whitespace().collect();
        println!("parts:{:?}",parts);
        if parts.is_empty(){
            continue;
        }
        let command = parts[0];
        let args = &parts[1..];
        println!("{:?}",args);
        
        let mut cd_args = PathBuf::new();

        match command{
            "vt"=>println!("{}", "current command 'vt'".bright_blue()),
            "cp"=>println!("current command 'cp', with args:{}", args.join(" ").bright_blue()),
            "cd"=>{
               println!("{}", "current command 'cd'".bright_blue());
               for arg in args{
                    cd_args.push(arg);
               }
               cd_command(cd_args);
            },           
            "exit"=>{
                 println!("{}", "exiting...".bright_magenta());
                 break;
            }
            _=>println!("{} {}",
                "Unknown Command:".bright_red(),
                command.bright_magenta()),

        }
   

    } 
}
