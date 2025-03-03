use std::{self, fs::{self} };

pub struct CMD {
    cmd: String,
    args: Vec<String>
}

impl CMD {
    pub fn build(cmd: String, args: Vec<String>)->CMD{
        if args.len() < 1 {
            eprintln!("No arguments provided")
        }
        CMD{cmd: cmd, args: args}
    }

    pub fn exec(&self)->bool{
        match self.cmd.as_str() {
            "cat" => self.cat(),
            "grep" => self.grep(),
            "echo" => self.echo(),
            "exit" => self.exit(),
            _ => self.not_existing(),
        }
    }
    fn not_existing(&self)->bool{
        println!("Not valid command");
        true
    }

    fn cat(&self)->bool{
        for line in search_file_to_string(self.args[0].clone()){
            println!("{}", line);
        }
        true
    }

    pub fn grep(&self) -> bool{
        let mut index: i32 = 0;
        for line in search_file_to_string(self.args[1].clone()) {
            for word_of_list in line.split_whitespace(){
                if self.args[0].eq(word_of_list){
                    println!("{} word find in line, {}", self.args[0], index+1);
                }
            }
            index +=1;
        }
        true
    }
    
    pub fn echo(&self) ->bool{
        if self.args.len()> 1 {
            self.write_in_file();
        }else{
            self.print_message();
        }
        true
    }

    fn print_message(&self){
        println!("{}", self.args[0]);
    }

    fn write_in_file(&self){
        let result = fs::write(self.args[2].clone(), self.args[0].clone());
        match result {
            Result::Ok(_) => println!("Writed succesfully."),
            Result::Err(_) =>eprintln!("Error on opening the file"),
        }
    }

    fn exit(&self)->bool{
        false
    }

    
}

fn search_file_to_string(path: String) -> Vec<String> {
    let result_file: String = fs::read_to_string(path.clone()).expect("Should be a file");
    result_file.split("\n").map(str::to_string).collect()   
}


// Uncomment this to use the just one command per execution mode
/*pub trait SearchFile{
    fn search_file_to_string(&self) -> Vec<String>;
}

pub struct GREP {
    pub path: String,
    pub query: String
}

impl GREP {
    pub fn build(args: &[String])->GREP{
        if args.len() < 4{
            panic!("Not enough arguments for grep cmd.")
        }
        GREP{path: args[3].clone(), query: args[2].clone()}
    }

    pub fn search_word_in_list(&self, list:Vec<String>) -> Vec<i32>{
        let mut  result:Vec<i32> = Vec::new();
        let mut index: i32 = 0;
        for line in list {
            for word_of_list in line.split_whitespace(){
                if self.query.eq(word_of_list){
                    result.push(index+1);
                }
            }
            index +=1;
        }
        result
    }
}

impl SearchFile for GREP {
    fn search_file_to_string(&self) -> Vec<String> {
        println!("HOLA :::: {}", self.path);
        let result_file: String = fs::read_to_string(self.path.clone()).expect("Should be a file");
        result_file.split("\n").map(str::to_string).collect()
        
    }
}

pub struct CAT {
    path: String
}

impl CAT {
    pub fn build(args: &[String])->CAT{
        if args.len() < 3{
            panic!("Not enough arguments for cat cmd.")
        }
        CAT{path: args[2].clone()}
    }
}

impl SearchFile for CAT {
    fn search_file_to_string(&self) -> Vec<String> {
        let result_file: String = fs::read_to_string(self.path.clone()).expect("Should be a file");
        result_file.split("\n").map(str::to_string).collect()
    }
}

pub struct ECHO {
    message: String,
    path: String
}

impl ECHO {
    pub fn build(args: &[String])->ECHO{
        if args.len() < 5{
            if args.len() < 3{
                panic!("Not enough arguments for echo cmd.")
            }  
            return ECHO{message: args[2].clone(), path: String::new()}
        }
        ECHO{message: args[2].clone(), path: args[4].clone()}
    }

    pub fn exe_echo(&self){
        if self.path.is_empty() {
            self.print_message();
        }else{
            self.write_in_file();
        }
    }

    fn print_message(&self){
        println!("{}", self.message);
    }

    fn write_in_file(&self){
        let result = fs::write(self.path.clone(), self.message.clone());
        match result {
            Result::Ok(_) => println!("Writed succesfully."),
            Result::Err(_) =>eprintln!("Error on opening the file"),
        }
    }
}



pub struct EXIT {}

impl EXIT{
    pub fn close_app(){}
}*/


