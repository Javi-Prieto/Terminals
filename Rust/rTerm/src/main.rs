use std::{self, io};

use models::commands::CMD;


pub mod models;


// This is the multi command option
fn main(){
    let mut continue_exec = true;
    while continue_exec{
        let (cmd_str, args) = get_cmd_args();
        let cmd = CMD::build(cmd_str, args);
        continue_exec = cmd.exec();
    }
}

fn get_cmd_args() -> (String, Vec<String>){
    println!("Cool user>");
    let args:Vec<String> = io::stdin().lines().next().unwrap().unwrap().split_whitespace().map(|s| s.to_string()).collect();
    return (args[0].clone(), args.split_at(1).1.to_owned());
}

// Uncomment this to use the just one command per execution mode
/*fn main() {
    let args_it: Vec<String> = std::env::args().collect();
    let exit = "exit".to_string();
    let cmd_name = args_it.get(1).unwrap_or(&exit).as_str();
    if cmd_name == "grep"{
        let cmd_grep = GREP::build(&args_it);
        let indexes = cmd_grep.search_word_in_list(cmd_grep.search_file_to_string());
        for index in indexes{
            println!("{} word find in line, {}", cmd_grep.query, index);
        }
    }else if cmd_name == "cat"{
        let cmd_cat = CAT::build(&args_it);
        let lines = cmd_cat.search_file_to_string();
        for line in lines{
            println!("{}", line);
        }
    }else if cmd_name == "echo"{
        let cmd_echo = ECHO::build(&args_it);
        cmd_echo.exe_echo();
    }else{
        println!("Exiting without doing nothing")
    }
}*/


