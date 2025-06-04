use std :: env;
use colored::*;
use std::io::{self, Write};


fn main (){

    let banner = r#"
    ███████╗██╗███╗   ██╗██████╗ ███████╗██████╗ 
    ██╔════╝██║████╗  ██║██╔══██╗██╔════╝██╔══██╗
    █████╗  ██║██╔██╗ ██║██║  ██║█████╗  ██████╔╝
    ██╔══╝  ██║██║╚██╗██║██║  ██║██╔══╝  ██╔══██╗
    ██║     ██║██║ ╚████║██████╔╝███████╗██║  ██║
    ╚═╝     ╚═╝╚═╝  ╚═══╝╚═════╝ ╚══════╝╚═╝  ╚═╝
    "#;
    let first_cmd = "cureent_path".bright_magenta().italic();
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
        print!("explorer>");
        io::stdout().flush().unwrap();

        //read input
        let mut input = String::new();
        io::stdin().read_line(&mut input ).unwrap();
        println!("this is the input: {}", input);
        
        //trimming the input
        let parts:Vec<&str> = input.trim().split_whitespace().collect();
        println!("parts:{:?}",parts);
        if parts.is_empty(){
            continue;
        }
        let command = parts[0];
        let args = &parts[1..];
        match command{
            "vt"=>println!("would run 'vt'"),
            "cp"=>println!("would run 'cp', with args:{:?}", args),
            "cd"=>println!("would run 'cd'"),
            "exit"=>{
                 println!("bye");
                 break;
             }
            _=>println!("unknown command: '{}'", command),

        }
   

    } 
}
